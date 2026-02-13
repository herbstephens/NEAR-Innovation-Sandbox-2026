# Technical Architecture

## Multi-Chain Governance Agent + TIME Protocol

---

## Overview

The system operates across three blockchains, each chosen for specific capabilities:

| Chain | Role | Why |
|-------|------|-----|
| **NEAR** | AI agent, smart contracts, governance | Best AI infra, fast, cheap, WASM |
| **Worldchain** | Identity (World ID) | Native World ID integration |
| **Ethereum** | Liquidity, DeFi bridges | Deepest liquidity, widest reach |

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              USER INTERFACE                              │
│                         (Web / Mobile / Agent)                          │
└───────────────────────────────────┬─────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
                    ▼               ▼               ▼
┌─────────────────────┐ ┌─────────────────────┐ ┌─────────────────────┐
│     WORLDCHAIN      │ │        NEAR         │ │      ETHEREUM       │
│                     │ │                     │ │                     │
│  ┌───────────────┐  │ │  ┌───────────────┐  │ │  ┌───────────────┐  │
│  │   World ID    │  │ │  │  Governance   │  │ │  │  TIME Token   │  │
│  │               │  │ │  │    Agent      │  │ │  │   (ERC-20)    │  │
│  │ • Verify      │  │ │  │               │  │ │  │               │  │
│  │ • Nullifier   │  │ │  │ • Discovery   │  │ │  │ • Main pool   │  │
│  │ • ZK Proof    │  │ │  │ • Context     │  │ │  │ • DeFi        │  │
│  └───────────────┘  │ │  │ • Local match │  │ │  │ • Bridges     │  │
│                     │ │  │ • Monitoring  │  │ │  └───────────────┘  │
│                     │ │  └───────────────┘  │ │                     │
│                     │ │                     │ │                     │
│                     │ │  ┌───────────────┐  │ │                     │
│                     │ │  │    Contracts  │  │ │                     │
│                     │ │  │               │  │ │                     │
│                     │ │  │ • TIME Token  │  │ │                     │
│                     │ │  │   (NEP-141)   │  │ │                     │
│                     │ │  │ • Work NFT    │  │ │                     │
│                     │ │  │   (NEP-171)   │  │ │                     │
│                     │ │  │ • Staking     │  │ │                     │
│                     │ │  │ • Governance  │  │ │                     │
│                     │ │  │ • Local Alloc │  │ │                     │
│                     │ │  └───────────────┘  │ │                     │
└─────────────────────┘ └─────────────────────┘ └─────────────────────┘
          │                       │                       │
          └───────────────────────┼───────────────────────┘
                                  │
                    ┌─────────────┴─────────────┐
                    │     CROSS-CHAIN BRIDGE    │
                    │                           │
                    │  • NEAR ↔ Ethereum        │
                    │  • TIME token bridging    │
                    │  • State verification     │
                    └───────────────────────────┘
```

---

## NEAR Components

### 1. Governance Agent

The AI agent runs on NEAR infrastructure:

```
┌─────────────────────────────────────────────────────────────┐
│                   GOVERNANCE AGENT                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  DISCOVERY MODULE                                           │
│  ├─ Input: Location (lat/long or address)                  │
│  ├─ Process:                                                │
│  │   ├─ Determine jurisdiction hierarchy                   │
│  │   ├─ Query official databases                           │
│  │   ├─ Query law databases                                │
│  │   └─ Compile governance stack                           │
│  └─ Output: {officials: [...], laws: [...], jurisdiction}  │
│                                                             │
│  CONTEXT MODULE                                             │
│  ├─ Input: User decision/question                          │
│  ├─ Process:                                                │
│  │   ├─ Parse intent                                       │
│  │   ├─ Match to relevant governance                       │
│  │   ├─ Identify permits/regulations                       │
│  │   └─ Find local resources                               │
│  └─ Output: Contextual advice + opportunities              │
│                                                             │
│  LOCAL MODULE                                               │
│  ├─ Input: User location + preferences                     │
│  ├─ Process:                                                │
│  │   ├─ Query local opportunity registry                   │
│  │   ├─ Match to user interests                            │
│  │   ├─ Verify governance compliance                       │
│  │   └─ Calculate local multiplier                         │
│  └─ Output: Ranked local opportunities                     │
│                                                             │
│  MONITORING MODULE                                          │
│  ├─ Background jobs (cron)                                 │
│  ├─ Watch: Elections, law changes, redistricting           │
│  ├─ Watch: New local opportunities                         │
│  └─ Output: Notifications to user                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2. Smart Contracts

#### TIME Token (NEP-141)

```rust
// time_token.rs

pub struct TimeToken {
    token: FungibleToken,
    owner_id: AccountId,
    total_birthright_claimed: Balance,
    total_work_minted: Balance,
}

impl TimeToken {
    // Claim birthright TIME
    pub fn claim_birthright(
        &mut self,
        world_id_proof: WorldIdProof,
        birthdate: Timestamp,
    ) -> Balance {
        // Verify World ID (one-time claim)
        self.verify_world_id(&world_id_proof);
        
        // Calculate TIME: days_lived * 1 TIME
        let days_lived = self.calculate_days_lived(birthdate);
        let amount = days_lived * ONE_TIME;
        
        // Mint to user
        self.token.internal_deposit(&env::predecessor_account_id(), amount);
        self.total_birthright_claimed += amount;
        
        amount
    }
    
    // Mint work TIME (called by work_nft contract)
    pub fn mint_work_time(
        &mut self,
        recipient: AccountId,
        hours: u64,
    ) -> Balance {
        // Only work_nft contract can call this
        self.assert_work_nft_contract();
        
        let amount = hours * ONE_TIME;
        self.token.internal_deposit(&recipient, amount);
        self.total_work_minted += amount;
        
        amount
    }
}
```

#### Work NFT (NEP-171 Soulbound)

```rust
// work_nft.rs

pub struct WorkNft {
    nft: NonFungibleToken,
    work_records: LookupMap<TokenId, WorkRecord>,
}

pub struct WorkRecord {
    worker: AccountId,
    hours: u64,
    time_earned: Balance,
    work_type: String,
    description: String,
    verifier: AccountId,
    timestamp: Timestamp,
    jurisdiction: String,  // For local verification
}

impl WorkNft {
    // Mint work NFT (soulbound - cannot transfer)
    pub fn mint_work_nft(
        &mut self,
        worker: AccountId,
        hours: u64,
        work_type: String,
        description: String,
        jurisdiction: String,
    ) -> TokenId {
        // Generate token ID
        let token_id = format!("work-{}-{}", worker, env::block_timestamp());
        
        // Create work record
        let record = WorkRecord {
            worker: worker.clone(),
            hours,
            time_earned: hours * ONE_TIME,
            work_type,
            description,
            verifier: env::predecessor_account_id(),
            timestamp: env::block_timestamp(),
            jurisdiction,
        };
        
        // Mint NFT (soulbound)
        self.nft.internal_mint(token_id.clone(), worker.clone(), None);
        self.work_records.insert(&token_id, &record);
        
        // Trigger TIME minting
        ext_time_token::mint_work_time(
            worker,
            hours,
            self.time_token_id.clone(),
            NO_DEPOSIT,
            GAS_FOR_MINT,
        );
        
        token_id
    }
    
    // Override transfer to make soulbound
    pub fn nft_transfer(&mut self, _: AccountId, _: TokenId, _: Option<u64>, _: Option<String>) {
        panic!("Work NFTs are soulbound and cannot be transferred");
    }
}
```

#### Staking Contract

```rust
// staking.rs

pub struct Staking {
    stakes: LookupMap<AccountId, StakeInfo>,
    total_staked: Balance,
}

pub struct StakeInfo {
    amount: Balance,
    voting_power: u128,  // sqrt(amount)
    staked_at: Timestamp,
    unlock_requested_at: Option<Timestamp>,
}

impl Staking {
    pub fn stake(&mut self, amount: Balance) -> u128 {
        let account_id = env::predecessor_account_id();
        
        // Transfer TIME to contract
        ext_time_token::ft_transfer(
            env::current_account_id(),
            amount.into(),
            None,
            self.time_token_id.clone(),
            ONE_YOCTO,
            GAS_FOR_TRANSFER,
        );
        
        // Calculate voting power (quadratic)
        let existing = self.stakes.get(&account_id).unwrap_or_default();
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
        
        voting_power
    }
    
    pub fn get_voting_power(&self, account_id: AccountId) -> u128 {
        self.stakes.get(&account_id)
            .map(|s| s.voting_power)
            .unwrap_or(0)
    }
}
```

#### Local Allocation Contract

```rust
// local_allocation.rs

pub struct LocalAllocation {
    opportunities: UnorderedMap<OpportunityId, Opportunity>,
    allocations: LookupMap<AccountId, Vec<Allocation>>,
    local_multiplier: f64,  // e.g., 1.1 for 10% bonus
}

pub struct Opportunity {
    id: OpportunityId,
    creator: AccountId,
    title: String,
    description: String,
    jurisdiction: String,
    goal: Balance,
    raised: Balance,
    status: OpportunityStatus,
    stake: Balance,  // Creator stakes TIME for legitimacy
}

pub struct Allocation {
    opportunity_id: OpportunityId,
    amount: Balance,
    local_bonus: Balance,
    timestamp: Timestamp,
}

impl LocalAllocation {
    pub fn allocate(
        &mut self,
        opportunity_id: OpportunityId,
        amount: Balance,
        allocator_jurisdiction: String,
    ) -> (Balance, Balance) {
        let opp = self.opportunities.get(&opportunity_id)
            .expect("Opportunity not found");
        
        // Calculate local bonus
        let bonus = if allocator_jurisdiction == opp.jurisdiction {
            (amount as f64 * (self.local_multiplier - 1.0)) as Balance
        } else {
            0
        };
        
        // Transfer TIME
        let total = amount + bonus;
        // ... transfer logic
        
        // Record allocation
        let allocation = Allocation {
            opportunity_id,
            amount,
            local_bonus: bonus,
            timestamp: env::block_timestamp(),
        };
        
        // ... store allocation
        
        (amount, bonus)
    }
}
```

---

## Worldchain Integration

### World ID Verification

```typescript
// Frontend: World ID verification

import { IDKitWidget, VerificationLevel } from '@worldcoin/idkit';

const verifyWorldId = async (proof: ISuccessResult) => {
  // Send to backend for verification
  const response = await fetch('/api/verify', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      proof: proof.proof,
      merkle_root: proof.merkle_root,
      nullifier_hash: proof.nullifier_hash,
      verification_level: proof.verification_level,
    }),
  });
  
  if (response.ok) {
    // Verification successful
    // Now user can claim birthright TIME
    await claimBirthright(proof.nullifier_hash);
  }
};

// Backend: Verify with World ID API
app.post('/api/verify', async (req, res) => {
  const { proof, merkle_root, nullifier_hash, verification_level } = req.body;
  
  const verifyRes = await fetch(
    `https://developer.worldcoin.org/api/v1/verify/${process.env.WORLD_ID_APP_ID}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        merkle_root,
        nullifier_hash,
        proof,
        verification_level,
        action: 'claim_governance_agent',
      }),
    }
  );
  
  if (verifyRes.ok) {
    // Store nullifier to prevent double-claiming
    await storeNullifier(nullifier_hash);
    res.json({ verified: true });
  } else {
    res.status(400).json({ verified: false });
  }
});
```

---

## Ethereum Integration

### TIME Token (ERC-20)

```solidity
// TIME.sol (Ethereum)

contract TIME is ERC20, Ownable {
    address public bridge;
    
    constructor() ERC20("TIME", "TIME") {}
    
    function setBridge(address _bridge) external onlyOwner {
        bridge = _bridge;
    }
    
    // Only bridge can mint (for cross-chain transfers)
    function mint(address to, uint256 amount) external {
        require(msg.sender == bridge, "Only bridge");
        _mint(to, amount);
    }
    
    // Only bridge can burn (for cross-chain transfers)
    function burn(address from, uint256 amount) external {
        require(msg.sender == bridge, "Only bridge");
        _burn(from, amount);
    }
}
```

### Cross-Chain Bridge

```
NEAR → Ethereum:
1. User locks TIME on NEAR
2. Bridge relayer detects lock event
3. Bridge mints equivalent TIME on Ethereum
4. User receives TIME on Ethereum

Ethereum → NEAR:
1. User burns TIME on Ethereum
2. Bridge relayer detects burn event
3. Bridge unlocks TIME on NEAR
4. User receives TIME on NEAR
```

---

## Data Flow

### Governance Discovery

```
1. User provides location
   ↓
2. Agent determines jurisdiction hierarchy
   ├─ Federal (US)
   ├─ State (California)
   ├─ County (San Francisco)
   └─ City (San Francisco)
   ↓
3. Agent queries data sources
   ├─ Google Civic API → Officials
   ├─ OpenStates → State legislators
   ├─ Local gov APIs → City officials
   └─ Legal databases → Applicable laws
   ↓
4. Agent compiles governance stack
   {
     officials: [
       { name: "Joe Biden", role: "President", level: "federal" },
       { name: "Gavin Newsom", role: "Governor", level: "state" },
       // ... 31 more
     ],
     laws: {
       federal: 3000,
       state: 5500,
       local: 4347,
       total: 12847
     },
     jurisdiction: "san-francisco-ca-us"
   }
   ↓
5. Store on NEAR (user's governance profile)
   ↓
6. Return to user interface
```

### Local Allocation

```
1. User requests local opportunities
   ↓
2. Agent queries local_allocation contract
   ├─ Filter by user's jurisdiction
   ├─ Filter by user's interests (optional)
   └─ Rank by relevance, proximity, need
   ↓
3. Agent verifies governance compliance
   ├─ Is business properly permitted?
   ├─ What officials oversee this?
   └─ Any relevant pending regulations?
   ↓
4. Agent returns opportunities with context
   [
     {
       id: "op-123",
       title: "Maria's Panaderia Expansion",
       distance: "2.1 mi",
       goal: 5000,
       raised: 3200,
       governance: {
         permits: ["valid"],
         officials: ["Health Dept", "City Clerk"],
         compliance: "verified"
       }
     },
     // ... more opportunities
   ]
   ↓
5. User allocates TIME
   ↓
6. Transaction on NEAR
   ├─ TIME transferred
   ├─ Local bonus applied (if same jurisdiction)
   └─ Allocation recorded
   ↓
7. Work NFT minted (supporter badge)
```

---

## Security Considerations

### Sybil Resistance
- World ID ensures one person = one agent
- Nullifier stored to prevent double-claiming
- Work NFT verifiers are staked (lose stake if fraudulent)

### Data Privacy
- Governance profile stored on-chain (public)
- Sensitive data in NOVA encrypted vaults (optional)
- Location used for jurisdiction, not stored

### Economic Security
- TIME token has no admin mint function (after initialization)
- Staking has unlock period (prevents flash attacks)
- Opportunity creators stake TIME (forfeit if fraudulent)

### Cross-Chain Security
- Bridge uses multi-sig or threshold signatures
- Time-lock on large transfers
- Regular audits of bridge contracts

---

## Deployment

### NEAR Contracts

```bash
# Build
cd contracts/rust
cargo build --target wasm32-unknown-unknown --release

# Deploy
near deploy --wasmFile target/wasm32-unknown-unknown/release/time_token.wasm \
  --accountId time.governance-agent.near

near deploy --wasmFile target/wasm32-unknown-unknown/release/work_nft.wasm \
  --accountId work.governance-agent.near

near deploy --wasmFile target/wasm32-unknown-unknown/release/staking.wasm \
  --accountId stake.governance-agent.near

near deploy --wasmFile target/wasm32-unknown-unknown/release/local_allocation.wasm \
  --accountId local.governance-agent.near
```

### Ethereum Contracts

```bash
# Deploy with Hardhat
npx hardhat run scripts/deploy.js --network mainnet
```

### Frontend

```bash
cd frontend
npm install
npm run build
npm run start
```

---

## API Reference

### Agent Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/discover` | POST | Discover governance for location |
| `/api/context` | POST | Get decision context |
| `/api/local` | GET | Get local opportunities |
| `/api/allocate` | POST | Allocate TIME to opportunity |
| `/api/profile` | GET | Get user's governance profile |

### Contract Methods

| Contract | Method | Description |
|----------|--------|-------------|
| `time_token` | `claim_birthright` | Claim birthright TIME |
| `time_token` | `ft_transfer` | Transfer TIME |
| `work_nft` | `mint_work_nft` | Mint work verification |
| `staking` | `stake` | Stake TIME for voting |
| `staking` | `get_voting_power` | Get voting power |
| `local_allocation` | `create_opportunity` | Create local opportunity |
| `local_allocation` | `allocate` | Allocate to opportunity |

---

## Dependencies

### NEAR
- near-sdk-rs: 5.0.0
- near-contract-standards: 5.0.0

### Frontend
- Next.js: 14.x
- @worldcoin/idkit: 1.x
- near-api-js: 3.x

### Ethereum
- OpenZeppelin: 5.x
- Hardhat: 2.x

---

*Architecture designed for: reliability, security, cross-chain interoperability, and local-first economics.*
