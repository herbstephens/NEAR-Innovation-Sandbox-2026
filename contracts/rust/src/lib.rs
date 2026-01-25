// =============================================================================
// TIME Protocol Smart Contract
// NEAR Innovation Sandbox Hackathon 2026 | "AI That Works For You"
//
// One Job: Discover who governs you and keep your political voice allocated.
// =============================================================================

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

// =============================================================================
// CONSTANTS
// =============================================================================

const NANOSECONDS_PER_DAY: u64 = 86_400_000_000_000;
const ONE_TIME: Balance = 1_000_000_000_000_000_000_000_000; // 1 TIME = 10^24 yocto
const UNSTAKE_COOLDOWN_NS: u64 = 24 * 60 * 60 * 1_000_000_000; // 24 hours

// =============================================================================
// DATA STRUCTURES
// =============================================================================

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Citizen {
    pub account_id: AccountId,
    pub birthdate_timestamp: u64,      // Unix timestamp in nanoseconds
    pub time_balance: Balance,          // TIME token balance
    pub total_earned: Balance,          // Total TIME earned from work
    pub claimed_birthright: bool,       // Has claimed birthright TIME
    pub world_id_nullifier: String,     // World ID nullifier hash
    pub verification_level: String,     // "orb" or "device"
    pub jurisdiction: String,           // e.g., "san-francisco-ca"
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceProfile {
    pub jurisdiction: String,
    pub officials_count: u32,
    pub laws_count: u32,
    pub last_updated: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Allocation {
    pub choice_id: String,              // e.g., "mayor-sf", "governor-ca"
    pub choice_type: String,            // "official" or "law"
    pub level: String,                  // "local", "county", "state", "federal"
    pub amount: Balance,                // TIME allocated
    pub percentage: u8,                 // Percentage of total TIME (0-100)
    pub updated_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingPosition {
    pub staked_amount: Balance,
    pub voting_power: u128,             // √(staked_amount) for quadratic
    pub staked_at: u64,
    pub unstake_requested_at: Option<u64>,
    pub allocations: Vec<VotingAllocation>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VotingAllocation {
    pub choice_id: String,
    pub percentage: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WorkReceiptNFT {
    pub token_id: String,
    pub owner_id: AccountId,
    pub hours_worked: u64,
    pub time_earned: Balance,
    pub work_type: String,
    pub description: String,
    pub verified_by: AccountId,
    pub created_at: u64,
    // Soulbound: cannot be transferred
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AgentAction {
    pub action_type: String,            // "DISCOVERY", "ALLOCATION", "REBALANCE", etc.
    pub description: String,
    pub timestamp: u64,
    pub details: String,                // JSON string with action details
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AgentPermissions {
    pub can_auto_allocate: bool,
    pub max_single_allocation_pct: u8,  // Max % to one choice
    pub max_daily_reallocation: Balance,
    pub require_approval_above: Balance,
    pub notify_on_rebalance: bool,
    pub notify_on_new_laws: bool,
}

// =============================================================================
// MAIN CONTRACT
// =============================================================================

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TimeProtocol {
    // Owner
    owner_id: AccountId,
    
    // Citizens (World ID verified humans)
    citizens: LookupMap<AccountId, Citizen>,
    
    // World ID nullifiers (prevent double-claiming)
    world_id_nullifiers: UnorderedSet<String>,
    
    // Governance profiles
    governance_profiles: LookupMap<AccountId, GovernanceProfile>,
    
    // TIME allocations
    allocations: LookupMap<AccountId, Vec<Allocation>>,
    
    // Staking positions
    staking_positions: LookupMap<AccountId, StakingPosition>,
    
    // Work NFTs (soulbound)
    work_nfts: UnorderedMap<String, WorkReceiptNFT>,
    work_nfts_by_owner: LookupMap<AccountId, Vec<String>>,
    
    // Agent audit log
    agent_actions: LookupMap<AccountId, Vec<AgentAction>>,
    
    // Agent permissions
    agent_permissions: LookupMap<AccountId, AgentPermissions>,
    
    // Global stats
    total_time_supply: Balance,
    total_citizens: u64,
    total_staked: Balance,
}

#[near_bindgen]
impl TimeProtocol {
    // =========================================================================
    // INITIALIZATION
    // =========================================================================
    
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            citizens: LookupMap::new(b"c"),
            world_id_nullifiers: UnorderedSet::new(b"w"),
            governance_profiles: LookupMap::new(b"g"),
            allocations: LookupMap::new(b"a"),
            staking_positions: LookupMap::new(b"s"),
            work_nfts: UnorderedMap::new(b"n"),
            work_nfts_by_owner: LookupMap::new(b"o"),
            agent_actions: LookupMap::new(b"l"),
            agent_permissions: LookupMap::new(b"p"),
            total_time_supply: 0,
            total_citizens: 0,
            total_staked: 0,
        }
    }

    // =========================================================================
    // WORLD ID VERIFICATION & TIME CLAIMING
    // =========================================================================
    
    /// Claim birthright TIME with World ID proof
    /// This can only be called once per World ID nullifier
    pub fn claim_time_with_world_id(
        &mut self,
        birthdate_timestamp: u64,
        merkle_root: String,
        nullifier_hash: String,
        proof: String,
        verification_level: String,
    ) -> Citizen {
        let account_id = env::predecessor_account_id();
        
        // Check nullifier hasn't been used
        assert!(
            !self.world_id_nullifiers.contains(&nullifier_hash),
            "World ID nullifier already used"
        );
        
        // Verify the proof (in production, this would verify the ZK proof)
        // For hackathon, we trust the frontend verification
        self.verify_world_id_proof(&merkle_root, &nullifier_hash, &proof);
        
        // Calculate birthright TIME (1 TIME per day lived)
        let now = env::block_timestamp();
        let days_lived = (now - birthdate_timestamp) / NANOSECONDS_PER_DAY;
        let birthright_time = days_lived as u128 * ONE_TIME;
        
        // Create citizen
        let citizen = Citizen {
            account_id: account_id.clone(),
            birthdate_timestamp,
            time_balance: birthright_time,
            total_earned: 0,
            claimed_birthright: true,
            world_id_nullifier: nullifier_hash.clone(),
            verification_level,
            jurisdiction: String::new(), // Set by agent during discovery
            created_at: now,
        };
        
        // Store citizen and mark nullifier as used
        self.citizens.insert(&account_id, &citizen);
        self.world_id_nullifiers.insert(&nullifier_hash);
        
        // Update global stats
        self.total_time_supply += birthright_time;
        self.total_citizens += 1;
        
        // Initialize default agent permissions
        let default_permissions = AgentPermissions {
            can_auto_allocate: true,
            max_single_allocation_pct: 20,
            max_daily_reallocation: 1000 * ONE_TIME,
            require_approval_above: 500 * ONE_TIME,
            notify_on_rebalance: true,
            notify_on_new_laws: true,
        };
        self.agent_permissions.insert(&account_id, &default_permissions);
        
        // Log agent action
        self.log_agent_action(
            &account_id,
            "BIRTHRIGHT_CLAIMED",
            &format!("Claimed {} TIME for {} days lived", days_lived, days_lived),
            &format!("{{\"days_lived\": {}, \"time_claimed\": \"{}\"}}", days_lived, birthright_time),
        );
        
        citizen
    }
    
    fn verify_world_id_proof(&self, _merkle_root: &str, _nullifier_hash: &str, _proof: &str) {
        // In production: verify ZK proof against World ID contract
        // For hackathon: trust frontend verification
        // The nullifier check above prevents double-claiming
    }

    // =========================================================================
    // GOVERNANCE DISCOVERY & ALLOCATION
    // =========================================================================
    
    /// Set governance profile after AI agent discovery
    /// Called by the agent after discovering officials and laws
    pub fn set_governance_profile(
        &mut self,
        jurisdiction: String,
        officials_count: u32,
        laws_count: u32,
    ) {
        let account_id = env::predecessor_account_id();
        
        // Verify citizen exists
        let mut citizen = self.citizens.get(&account_id)
            .expect("Must be a registered citizen");
        
        // Update jurisdiction
        citizen.jurisdiction = jurisdiction.clone();
        self.citizens.insert(&account_id, &citizen);
        
        // Create governance profile
        let profile = GovernanceProfile {
            jurisdiction,
            officials_count,
            laws_count,
            last_updated: env::block_timestamp(),
        };
        self.governance_profiles.insert(&account_id, &profile);
        
        // Log agent action
        self.log_agent_action(
            &account_id,
            "GOVERNANCE_DISCOVERED",
            &format!("Discovered {} officials and {} laws", officials_count, laws_count),
            &format!("{{\"officials\": {}, \"laws\": {}}}", officials_count, laws_count),
        );
    }
    
    /// Initialize status quo allocation
    /// Distributes TIME across all discovered governance choices
    pub fn initialize_status_quo_allocation(&mut self, allocations_json: String) {
        let account_id = env::predecessor_account_id();
        
        // Parse allocations from JSON
        let allocations: Vec<Allocation> = serde_json::from_str(&allocations_json)
            .expect("Invalid allocations JSON");
        
        // Verify total percentage is 100 or less
        let total_pct: u8 = allocations.iter().map(|a| a.percentage).sum();
        assert!(total_pct <= 100, "Total allocation cannot exceed 100%");
        
        // Store allocations
        self.allocations.insert(&account_id, &allocations);
        
        // Log agent action
        self.log_agent_action(
            &account_id,
            "STATUS_QUO_ALLOCATED",
            &format!("Allocated TIME across {} choices", allocations.len()),
            &allocations_json,
        );
    }
    
    /// Reallocate TIME to different governance choices
    pub fn reallocate_time(
        &mut self,
        from_choice_id: String,
        to_choice_id: String,
        amount: U128,
    ) {
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        
        let mut allocations = self.allocations.get(&account_id)
            .unwrap_or_default();
        
        // Find source and update
        let from_idx = allocations.iter().position(|a| a.choice_id == from_choice_id)
            .expect("Source choice not found");
        assert!(allocations[from_idx].amount >= amount, "Insufficient allocation");
        allocations[from_idx].amount -= amount;
        
        // Find or create destination
        if let Some(to_idx) = allocations.iter().position(|a| a.choice_id == to_choice_id) {
            allocations[to_idx].amount += amount;
        } else {
            // This would need more info in production; simplified for hackathon
            allocations.push(Allocation {
                choice_id: to_choice_id.clone(),
                choice_type: "official".to_string(),
                level: "unknown".to_string(),
                amount,
                percentage: 0, // Will be recalculated
                updated_at: env::block_timestamp(),
            });
        }
        
        self.allocations.insert(&account_id, &allocations);
        
        // Log agent action
        self.log_agent_action(
            &account_id,
            "REALLOCATION",
            &format!("Moved {} TIME from {} to {}", amount / ONE_TIME, from_choice_id, to_choice_id),
            &format!("{{\"from\": \"{}\", \"to\": \"{}\", \"amount\": \"{}\"}}", 
                     from_choice_id, to_choice_id, amount),
        );
    }

    // =========================================================================
    // STAKING & VOTING POWER
    // =========================================================================
    
    /// Stake TIME for quadratic voting power
    pub fn stake(&mut self, amount: U128) -> StakingPosition {
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        
        let mut citizen = self.citizens.get(&account_id)
            .expect("Must be a registered citizen");
        
        // Check sufficient balance
        assert!(citizen.time_balance >= amount, "Insufficient TIME balance");
        
        // Check identity cap: max stake = 2× NFT-proven earnings
        let max_stake = citizen.total_earned * 2;
        let existing_stake = self.staking_positions.get(&account_id)
            .map(|p| p.staked_amount)
            .unwrap_or(0);
        assert!(
            existing_stake + amount <= max_stake || max_stake == 0,
            "Stake exceeds identity cap (2× proven earnings)"
        );
        
        // Calculate quadratic voting power
        let new_total_staked = existing_stake + amount;
        let voting_power = (new_total_staked as f64).sqrt() as u128;
        
        // Update citizen balance
        citizen.time_balance -= amount;
        self.citizens.insert(&account_id, &citizen);
        
        // Update or create staking position
        let position = StakingPosition {
            staked_amount: new_total_staked,
            voting_power,
            staked_at: env::block_timestamp(),
            unstake_requested_at: None,
            allocations: vec![],
        };
        self.staking_positions.insert(&account_id, &position);
        
        // Update global stats
        self.total_staked += amount;
        
        // Log agent action
        self.log_agent_action(
            &account_id,
            "STAKED",
            &format!("Staked {} TIME for {} VP", amount / ONE_TIME, voting_power),
            &format!("{{\"amount\": \"{}\", \"voting_power\": {}}}", amount, voting_power),
        );
        
        position
    }
    
    /// Request unstaking (starts cooldown)
    pub fn request_unstake(&mut self) {
        let account_id = env::predecessor_account_id();
        
        let mut position = self.staking_positions.get(&account_id)
            .expect("No staking position found");
        
        position.unstake_requested_at = Some(env::block_timestamp());
        self.staking_positions.insert(&account_id, &position);
    }
    
    /// Complete unstaking after cooldown
    pub fn complete_unstake(&mut self) {
        let account_id = env::predecessor_account_id();
        
        let position = self.staking_positions.get(&account_id)
            .expect("No staking position found");
        
        let requested_at = position.unstake_requested_at
            .expect("Unstake not requested");
        
        assert!(
            env::block_timestamp() >= requested_at + UNSTAKE_COOLDOWN_NS,
            "Cooldown period not complete"
        );
        
        // Return TIME to citizen
        let mut citizen = self.citizens.get(&account_id).unwrap();
        citizen.time_balance += position.staked_amount;
        self.citizens.insert(&account_id, &citizen);
        
        // Remove staking position
        self.staking_positions.remove(&account_id);
        
        // Update global stats
        self.total_staked -= position.staked_amount;
    }
    
    /// Allocate voting power to governance choices
    pub fn allocate_voting_power(&mut self, choice_id: String, percentage: u8) {
        let account_id = env::predecessor_account_id();
        
        let mut position = self.staking_positions.get(&account_id)
            .expect("No staking position found");
        
        // Update or add allocation
        if let Some(idx) = position.allocations.iter().position(|a| a.choice_id == choice_id) {
            position.allocations[idx].percentage = percentage;
        } else {
            position.allocations.push(VotingAllocation { choice_id, percentage });
        }
        
        // Verify total doesn't exceed 100%
        let total: u8 = position.allocations.iter().map(|a| a.percentage).sum();
        assert!(total <= 100, "Total voting allocation cannot exceed 100%");
        
        self.staking_positions.insert(&account_id, &position);
    }

    // =========================================================================
    // WORK NFTs (SOULBOUND)
    // =========================================================================
    
    /// Mint a work receipt NFT (soulbound, non-transferable)
    /// Only authorized verifiers can mint
    pub fn mint_work_nft(
        &mut self,
        recipient: AccountId,
        hours_worked: u64,
        work_type: String,
        description: String,
    ) -> WorkReceiptNFT {
        // In production: verify caller is authorized verifier
        // For hackathon: allow owner to mint
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Only authorized verifiers can mint work NFTs"
        );
        
        // Calculate TIME earned (1 TIME per hour)
        let time_earned = hours_worked as u128 * ONE_TIME;
        
        // Generate token ID
        let token_id = format!("work-{}-{}", recipient, env::block_timestamp());
        
        // Create NFT
        let nft = WorkReceiptNFT {
            token_id: token_id.clone(),
            owner_id: recipient.clone(),
            hours_worked,
            time_earned,
            work_type,
            description,
            verified_by: env::predecessor_account_id(),
            created_at: env::block_timestamp(),
        };
        
        // Store NFT
        self.work_nfts.insert(&token_id, &nft);
        
        // Update owner's NFT list
        let mut owner_nfts = self.work_nfts_by_owner.get(&recipient).unwrap_or_default();
        owner_nfts.push(token_id);
        self.work_nfts_by_owner.insert(&recipient, &owner_nfts);
        
        // Update citizen's earned TIME and balance
        if let Some(mut citizen) = self.citizens.get(&recipient) {
            citizen.total_earned += time_earned;
            citizen.time_balance += time_earned;
            self.citizens.insert(&recipient, &citizen);
            self.total_time_supply += time_earned;
        }
        
        nft
    }

    // =========================================================================
    // AGENT PERMISSIONS & AUDIT LOG
    // =========================================================================
    
    /// Update agent permissions
    pub fn update_agent_permissions(&mut self, permissions: AgentPermissions) {
        let account_id = env::predecessor_account_id();
        self.agent_permissions.insert(&account_id, &permissions);
    }
    
    /// Log an agent action (called internally)
    fn log_agent_action(
        &mut self,
        account_id: &AccountId,
        action_type: &str,
        description: &str,
        details: &str,
    ) {
        let action = AgentAction {
            action_type: action_type.to_string(),
            description: description.to_string(),
            timestamp: env::block_timestamp(),
            details: details.to_string(),
        };
        
        let mut actions = self.agent_actions.get(account_id).unwrap_or_default();
        actions.push(action);
        self.agent_actions.insert(account_id, &actions);
    }

    // =========================================================================
    // VIEW METHODS
    // =========================================================================
    
    pub fn get_citizen(&self, account_id: AccountId) -> Option<Citizen> {
        self.citizens.get(&account_id)
    }
    
    pub fn get_governance_profile(&self, account_id: AccountId) -> Option<GovernanceProfile> {
        self.governance_profiles.get(&account_id)
    }
    
    pub fn get_allocations(&self, account_id: AccountId) -> Vec<Allocation> {
        self.allocations.get(&account_id).unwrap_or_default()
    }
    
    pub fn get_staking_position(&self, account_id: AccountId) -> Option<StakingPosition> {
        self.staking_positions.get(&account_id)
    }
    
    pub fn get_work_nfts(&self, account_id: AccountId) -> Vec<WorkReceiptNFT> {
        let token_ids = self.work_nfts_by_owner.get(&account_id).unwrap_or_default();
        token_ids.iter()
            .filter_map(|id| self.work_nfts.get(id))
            .collect()
    }
    
    pub fn get_agent_actions(&self, account_id: AccountId) -> Vec<AgentAction> {
        self.agent_actions.get(&account_id).unwrap_or_default()
    }
    
    pub fn get_agent_permissions(&self, account_id: AccountId) -> Option<AgentPermissions> {
        self.agent_permissions.get(&account_id)
    }
    
    pub fn get_global_stats(&self) -> (Balance, u64, Balance) {
        (self.total_time_supply, self.total_citizens, self.total_staked)
    }
    
    pub fn is_nullifier_used(&self, nullifier_hash: String) -> bool {
        self.world_id_nullifiers.contains(&nullifier_hash)
    }

    // =========================================================================
    // USER DATA EXPORT (User-Owned Memory)
    // =========================================================================
    
    /// Export all user data (portable, inspectable)
    pub fn export_user_data(&self, account_id: AccountId) -> String {
        let citizen = self.get_citizen(account_id.clone());
        let profile = self.get_governance_profile(account_id.clone());
        let allocations = self.get_allocations(account_id.clone());
        let staking = self.get_staking_position(account_id.clone());
        let nfts = self.get_work_nfts(account_id.clone());
        let actions = self.get_agent_actions(account_id.clone());
        let permissions = self.get_agent_permissions(account_id);
        
        serde_json::json!({
            "citizen": citizen,
            "governance_profile": profile,
            "allocations": allocations,
            "staking_position": staking,
            "work_nfts": nfts,
            "agent_actions": actions,
            "agent_permissions": permissions,
        }).to_string()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_new_contract() {
        let context = get_context("owner.near".parse().unwrap());
        testing_env!(context.build());
        
        let contract = TimeProtocol::new("owner.near".parse().unwrap());
        let (supply, citizens, staked) = contract.get_global_stats();
        
        assert_eq!(supply, 0);
        assert_eq!(citizens, 0);
        assert_eq!(staked, 0);
    }
}
