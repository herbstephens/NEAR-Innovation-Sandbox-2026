// =============================================================================
// Governance Agent Smart Contracts
// NEAR Innovation Sandbox 2026 | "AI That Works For You"
//
// Your Governance Agent: Know who governs you. Allocate locally.
// =============================================================================

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

// =============================================================================
// CONSTANTS
// =============================================================================

const ONE_TIME: Balance = 1_000_000_000_000_000_000_000_000; // 1 TIME = 10^24 yocto
const LOCAL_MULTIPLIER: u128 = 110; // 1.1x = 10% bonus for local allocation
const UNSTAKE_COOLDOWN_NS: u64 = 24 * 60 * 60 * 1_000_000_000; // 24 hours

// =============================================================================
// DATA STRUCTURES
// =============================================================================

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Citizen {
    pub account_id: AccountId,
    pub birthdate_timestamp: u64,
    pub time_balance: Balance,
    pub birthright_claimed: Balance,
    pub work_earned: Balance,
    pub world_id_nullifier: String,
    pub jurisdiction: String,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceProfile {
    pub jurisdiction: String,
    pub officials_count: u32,
    pub laws_count: u32,
    pub officials: Vec<Official>,
    pub last_updated: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Official {
    pub name: String,
    pub role: String,
    pub level: String, // federal, state, county, local
    pub jurisdiction: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WorkNFT {
    pub token_id: String,
    pub owner_id: AccountId,
    pub hours_worked: u64,
    pub time_earned: Balance,
    pub work_type: String,
    pub description: String,
    pub jurisdiction: String,
    pub verifier: AccountId,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeInfo {
    pub amount: Balance,
    pub voting_power: u128,
    pub staked_at: u64,
    pub unlock_requested_at: Option<u64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct LocalOpportunity {
    pub id: String,
    pub creator: AccountId,
    pub title: String,
    pub description: String,
    pub jurisdiction: String,
    pub goal: Balance,
    pub raised: Balance,
    pub stake: Balance, // Creator stakes for legitimacy
    pub status: String, // active, funded, closed
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Allocation {
    pub opportunity_id: String,
    pub amount: Balance,
    pub local_bonus: Balance,
    pub timestamp: u64,
}

// =============================================================================
// MAIN CONTRACT
// =============================================================================

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GovernanceAgent {
    owner_id: AccountId,
    
    // Citizens
    citizens: LookupMap<AccountId, Citizen>,
    world_id_nullifiers: UnorderedSet<String>,
    
    // Governance profiles
    governance_profiles: LookupMap<AccountId, GovernanceProfile>,
    
    // Work NFTs (soulbound)
    work_nfts: UnorderedMap<String, WorkNFT>,
    work_nfts_by_owner: LookupMap<AccountId, Vec<String>>,
    
    // Staking
    stakes: LookupMap<AccountId, StakeInfo>,
    total_staked: Balance,
    
    // Local opportunities
    opportunities: UnorderedMap<String, LocalOpportunity>,
    opportunities_by_jurisdiction: LookupMap<String, Vec<String>>,
    
    // Allocations
    allocations: LookupMap<AccountId, Vec<Allocation>>,
    
    // Stats
    total_time_supply: Balance,
    total_citizens: u64,
    total_local_allocated: Balance,
}

#[near_bindgen]
impl GovernanceAgent {
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
            work_nfts: UnorderedMap::new(b"n"),
            work_nfts_by_owner: LookupMap::new(b"o"),
            stakes: LookupMap::new(b"s"),
            total_staked: 0,
            opportunities: UnorderedMap::new(b"p"),
            opportunities_by_jurisdiction: LookupMap::new(b"j"),
            allocations: LookupMap::new(b"a"),
            total_time_supply: 0,
            total_citizens: 0,
            total_local_allocated: 0,
        }
    }

    // =========================================================================
    // IDENTITY + BIRTHRIGHT (World ID)
    // =========================================================================
    
    /// Claim birthright TIME with World ID proof
    pub fn claim_birthright(
        &mut self,
        birthdate_timestamp: u64,
        nullifier_hash: String,
        jurisdiction: String,
    ) -> Citizen {
        let account_id = env::predecessor_account_id();
        
        // Check nullifier hasn't been used
        assert!(
            !self.world_id_nullifiers.contains(&nullifier_hash),
            "World ID already used"
        );
        
        // Calculate birthright TIME (1 TIME per day lived)
        let now = env::block_timestamp();
        let nanoseconds_per_day: u64 = 86_400_000_000_000;
        let days_lived = (now - birthdate_timestamp) / nanoseconds_per_day;
        let birthright_time = days_lived as u128 * ONE_TIME;
        
        // Create citizen
        let citizen = Citizen {
            account_id: account_id.clone(),
            birthdate_timestamp,
            time_balance: birthright_time,
            birthright_claimed: birthright_time,
            work_earned: 0,
            world_id_nullifier: nullifier_hash.clone(),
            jurisdiction,
            created_at: now,
        };
        
        // Store
        self.citizens.insert(&account_id, &citizen);
        self.world_id_nullifiers.insert(&nullifier_hash);
        self.total_time_supply += birthright_time;
        self.total_citizens += 1;
        
        citizen
    }

    // =========================================================================
    // GOVERNANCE DISCOVERY
    // =========================================================================
    
    /// Store governance profile after agent discovery
    pub fn set_governance_profile(
        &mut self,
        jurisdiction: String,
        officials_count: u32,
        laws_count: u32,
        officials: Vec<Official>,
    ) {
        let account_id = env::predecessor_account_id();
        
        // Must be registered citizen
        assert!(
            self.citizens.contains_key(&account_id),
            "Must be registered citizen"
        );
        
        let profile = GovernanceProfile {
            jurisdiction,
            officials_count,
            laws_count,
            officials,
            last_updated: env::block_timestamp(),
        };
        
        self.governance_profiles.insert(&account_id, &profile);
    }

    // =========================================================================
    // WORK + EARNING
    // =========================================================================
    
    /// Mint work NFT and earn TIME
    pub fn mint_work_nft(
        &mut self,
        worker: AccountId,
        hours: u64,
        work_type: String,
        description: String,
        jurisdiction: String,
    ) -> WorkNFT {
        // Only owner or authorized verifiers can mint
        // For hackathon: allow owner
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "Only authorized verifiers"
        );
        
        let time_earned = hours as u128 * ONE_TIME;
        let token_id = format!("work-{}-{}", worker, env::block_timestamp());
        
        let nft = WorkNFT {
            token_id: token_id.clone(),
            owner_id: worker.clone(),
            hours_worked: hours,
            time_earned,
            work_type,
            description,
            jurisdiction,
            verifier: env::predecessor_account_id(),
            created_at: env::block_timestamp(),
        };
        
        // Store NFT
        self.work_nfts.insert(&token_id, &nft);
        
        // Update owner's NFTs
        let mut owner_nfts = self.work_nfts_by_owner.get(&worker).unwrap_or_default();
        owner_nfts.push(token_id);
        self.work_nfts_by_owner.insert(&worker, &owner_nfts);
        
        // Credit TIME to worker
        if let Some(mut citizen) = self.citizens.get(&worker) {
            citizen.time_balance += time_earned;
            citizen.work_earned += time_earned;
            self.citizens.insert(&worker, &citizen);
            self.total_time_supply += time_earned;
        }
        
        nft
    }

    // =========================================================================
    // LOCAL ALLOCATION
    // =========================================================================
    
    /// Create a local opportunity
    pub fn create_opportunity(
        &mut self,
        title: String,
        description: String,
        jurisdiction: String,
        goal: U128,
        stake: U128,
    ) -> LocalOpportunity {
        let account_id = env::predecessor_account_id();
        let goal: Balance = goal.into();
        let stake: Balance = stake.into();
        
        // Must stake TIME for legitimacy
        let mut citizen = self.citizens.get(&account_id)
            .expect("Must be citizen");
        assert!(citizen.time_balance >= stake, "Insufficient TIME for stake");
        
        // Deduct stake
        citizen.time_balance -= stake;
        self.citizens.insert(&account_id, &citizen);
        
        let id = format!("opp-{}-{}", account_id, env::block_timestamp());
        
        let opportunity = LocalOpportunity {
            id: id.clone(),
            creator: account_id,
            title,
            description,
            jurisdiction: jurisdiction.clone(),
            goal,
            raised: 0,
            stake,
            status: "active".to_string(),
            created_at: env::block_timestamp(),
        };
        
        // Store
        self.opportunities.insert(&id, &opportunity);
        
        // Index by jurisdiction
        let mut jurisdiction_opps = self.opportunities_by_jurisdiction
            .get(&jurisdiction)
            .unwrap_or_default();
        jurisdiction_opps.push(id);
        self.opportunities_by_jurisdiction.insert(&jurisdiction, &jurisdiction_opps);
        
        opportunity
    }
    
    /// Allocate TIME to a local opportunity
    pub fn allocate(
        &mut self,
        opportunity_id: String,
        amount: U128,
    ) -> (Balance, Balance) {
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        
        // Get citizen and opportunity
        let mut citizen = self.citizens.get(&account_id)
            .expect("Must be citizen");
        let mut opportunity = self.opportunities.get(&opportunity_id)
            .expect("Opportunity not found");
        
        assert!(citizen.time_balance >= amount, "Insufficient TIME");
        assert!(opportunity.status == "active", "Opportunity not active");
        
        // Calculate local bonus (10% if same jurisdiction)
        let local_bonus = if citizen.jurisdiction == opportunity.jurisdiction {
            amount * (LOCAL_MULTIPLIER - 100) / 100
        } else {
            0
        };
        
        // Deduct from citizen
        citizen.time_balance -= amount;
        self.citizens.insert(&account_id, &citizen);
        
        // Add to opportunity (including bonus which comes from protocol)
        let total = amount + local_bonus;
        opportunity.raised += total;
        
        // Check if funded
        if opportunity.raised >= opportunity.goal {
            opportunity.status = "funded".to_string();
        }
        
        self.opportunities.insert(&opportunity_id, &opportunity);
        
        // Record allocation
        let allocation = Allocation {
            opportunity_id,
            amount,
            local_bonus,
            timestamp: env::block_timestamp(),
        };
        
        let mut user_allocations = self.allocations.get(&account_id).unwrap_or_default();
        user_allocations.push(allocation);
        self.allocations.insert(&account_id, &user_allocations);
        
        self.total_local_allocated += total;
        
        (amount, local_bonus)
    }

    // =========================================================================
    // STAKING + GOVERNANCE
    // =========================================================================
    
    /// Stake TIME for voting power
    pub fn stake(&mut self, amount: U128) -> StakeInfo {
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        
        let mut citizen = self.citizens.get(&account_id)
            .expect("Must be citizen");
        assert!(citizen.time_balance >= amount, "Insufficient TIME");
        
        // Deduct from balance
        citizen.time_balance -= amount;
        self.citizens.insert(&account_id, &citizen);
        
        // Calculate voting power (quadratic)
        let existing = self.stakes.get(&account_id).unwrap_or(StakeInfo {
            amount: 0,
            voting_power: 0,
            staked_at: 0,
            unlock_requested_at: None,
        });
        
        let new_amount = existing.amount + amount;
        let voting_power = (new_amount as f64).sqrt() as u128;
        
        let stake_info = StakeInfo {
            amount: new_amount,
            voting_power,
            staked_at: env::block_timestamp(),
            unlock_requested_at: None,
        };
        
        self.stakes.insert(&account_id, &stake_info);
        self.total_staked += amount;
        
        stake_info
    }
    
    /// Request unstake (starts cooldown)
    pub fn request_unstake(&mut self) {
        let account_id = env::predecessor_account_id();
        
        let mut stake_info = self.stakes.get(&account_id)
            .expect("No stake found");
        
        stake_info.unlock_requested_at = Some(env::block_timestamp());
        self.stakes.insert(&account_id, &stake_info);
    }
    
    /// Complete unstake after cooldown
    pub fn complete_unstake(&mut self) {
        let account_id = env::predecessor_account_id();
        
        let stake_info = self.stakes.get(&account_id)
            .expect("No stake found");
        
        let requested_at = stake_info.unlock_requested_at
            .expect("Unstake not requested");
        
        assert!(
            env::block_timestamp() >= requested_at + UNSTAKE_COOLDOWN_NS,
            "Cooldown not complete"
        );
        
        // Return TIME
        let mut citizen = self.citizens.get(&account_id).unwrap();
        citizen.time_balance += stake_info.amount;
        self.citizens.insert(&account_id, &citizen);
        
        self.total_staked -= stake_info.amount;
        self.stakes.remove(&account_id);
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
    
    pub fn get_stake_info(&self, account_id: AccountId) -> Option<StakeInfo> {
        self.stakes.get(&account_id)
    }
    
    pub fn get_voting_power(&self, account_id: AccountId) -> u128 {
        self.stakes.get(&account_id)
            .map(|s| s.voting_power)
            .unwrap_or(0)
    }
    
    pub fn get_work_nfts(&self, account_id: AccountId) -> Vec<WorkNFT> {
        let token_ids = self.work_nfts_by_owner.get(&account_id).unwrap_or_default();
        token_ids.iter()
            .filter_map(|id| self.work_nfts.get(id))
            .collect()
    }
    
    pub fn get_opportunity(&self, id: String) -> Option<LocalOpportunity> {
        self.opportunities.get(&id)
    }
    
    pub fn get_local_opportunities(&self, jurisdiction: String) -> Vec<LocalOpportunity> {
        let ids = self.opportunities_by_jurisdiction.get(&jurisdiction).unwrap_or_default();
        ids.iter()
            .filter_map(|id| self.opportunities.get(id))
            .filter(|opp| opp.status == "active")
            .collect()
    }
    
    pub fn get_allocations(&self, account_id: AccountId) -> Vec<Allocation> {
        self.allocations.get(&account_id).unwrap_or_default()
    }
    
    pub fn get_stats(&self) -> (Balance, u64, Balance, Balance) {
        (
            self.total_time_supply,
            self.total_citizens,
            self.total_staked,
            self.total_local_allocated,
        )
    }
    
    pub fn is_nullifier_used(&self, nullifier: String) -> bool {
        self.world_id_nullifiers.contains(&nullifier)
    }
}
