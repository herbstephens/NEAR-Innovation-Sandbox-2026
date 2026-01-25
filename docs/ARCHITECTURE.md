# TIME Protocol Architecture

## Overview

TIME Protocol is an AI governance agent that discovers who governs you and keeps your political voice allocated — automatically.

```
┌─────────────────────────────────────────────────────────────┐
│                    TIME Protocol Stack                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Frontend (Next.js 14)                   │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │   │
│  │  │  World ID   │  │   React     │  │   Sponsor   │  │   │
│  │  │  IDKitWidget│  │   Screens   │  │ Integrations│  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                │
│  ┌─────────────────────────▼───────────────────────────┐   │
│  │              AI Governance Agent                     │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │   │
│  │  │  Discovery  │  │  Allocation │  │  Monitoring │  │   │
│  │  │   Engine    │  │   Engine    │  │   Engine    │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                            │                                │
│  ┌─────────────────────────▼───────────────────────────┐   │
│  │           Smart Contract (Rust on NEAR)              │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐  │   │
│  │  │   TIME   │ │   Work   │ │ Staking  │ │ Audit  │  │   │
│  │  │  Token   │ │   NFTs   │ │ + Voting │ │  Log   │  │   │
│  │  │ (NEP-141)│ │(NEP-171) │ │Quadratic │ │        │  │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. TIME Token (NEP-141)

Fungible token representing your political voice.

**Sources of TIME:**
- **Birthright**: 1 TIME per day lived (claimed once with World ID)
- **Earned**: 1 TIME per hour of verified work (minted as Work NFT)
- **Partnership**: Allocated by organizations you're part of

**Properties:**
- Transferable
- Stakeable for voting power
- Allocated to governance choices

### 2. Work NFTs (NEP-171 Soulbound)

Non-transferable proof of contribution.

**Properties:**
- Soulbound to owner (cannot be transferred)
- Records hours worked, TIME earned
- Verified by authorized accounts
- Increases identity cap for staking

### 3. Staking & Voting Power

Quadratic staking with identity cap.

**Formula:**
```
Voting Power = √(Staked TIME)
```

**Identity Cap:**
```
Max Stake = 2 × Total TIME Earned from Work NFTs
```

**Cooldown:**
- 24-hour unstaking period
- Prevents flash governance attacks

### 4. Allocations

How your TIME is distributed across governance choices.

**Levels:**
- Local (city, county)
- State
- Federal
- Global (UN, treaties)

**Types:**
- Officials (elected representatives)
- Laws (statutes, ordinances, regulations)

---

## AI Agent Architecture

### Discovery Engine

Finds all governance that applies to you.

```javascript
async function discoverGovernance(jurisdiction) {
  const officials = await queryOfficials(jurisdiction);
  const laws = await queryLaws(jurisdiction);
  
  return {
    officials: officials.count,  // e.g., 33
    laws: laws.count,            // e.g., 12,847
    breakdown: {
      local: officials.filter(o => o.level === 'local'),
      state: officials.filter(o => o.level === 'state'),
      federal: officials.filter(o => o.level === 'federal'),
    }
  };
}
```

### Allocation Engine

Distributes TIME across governance choices.

**Status Quo Distribution:**
```javascript
const defaultWeights = {
  local: 0.30,    // 30% to local governance
  county: 0.10,   // 10% to county
  state: 0.25,    // 25% to state
  federal: 0.20,  // 20% to federal
  laws: 0.15,     // 15% to law acceptance
};
```

### Monitoring Engine

Watches for governance changes.

**Scheduled Jobs:**
- Daily: Check election results
- Weekly: Check new laws
- On-event: Redistricting detection

---

## World ID Integration

Zero-knowledge proof of humanity.

### Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   World     │    │  Frontend   │    │   Backend   │
│    App      │    │  IDKitWidget│    │   /verify   │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │
       │  Scan QR Code    │                  │
       │<─────────────────│                  │
       │                  │                  │
       │  Generate Proof  │                  │
       │─────────────────>│                  │
       │                  │                  │
       │                  │  Verify Proof    │
       │                  │─────────────────>│
       │                  │                  │
       │                  │  { verified }    │
       │                  │<─────────────────│
       │                  │                  │
```

### Proof Components

| Component | Purpose |
|-----------|---------|
| `merkle_root` | World ID identity tree root |
| `nullifier_hash` | Unique per user+action (prevents double-claim) |
| `proof` | ZK proof of inclusion |

---

## Sponsor Integrations

### NOVA (Storage)

Encrypted governance profile.

```
┌─────────────────────────────────┐
│         NOVA Vault              │
│  ┌───────────────────────────┐  │
│  │  Governance Profile       │  │
│  │  • Jurisdiction           │  │
│  │  • Officials discovered   │  │
│  │  • Allocation weights     │  │
│  │  • Preferences            │  │
│  │  • Audit log              │  │
│  └───────────────────────────┘  │
│                                 │
│  Access: Intent-based only      │
│  Encryption: User-owned keys    │
└─────────────────────────────────┘
```

### HOT KIT (Wallet)

Multi-chain wallet connection.

```javascript
// Connect any wallet
const wallet = await HotKit.connect({
  supportedChains: ['near', 'ethereum', 'base']
});

// View omni-balance
const balances = await HotKit.getOmniBalances(wallet.address, 'TIME');
// { near: 18250, ethereum: 0, base: 500 }

// Intent-based staking
const intent = HotKit.buildIntent({
  action: 'stake',
  amount: '1000',
  token: 'TIME'
});
```

### PingPay (Payments)

Agent-initiated payments.

```javascript
// Fiat onramp
const onramp = await PingPay.createOnramp({
  fiatAmount: 100,
  outputToken: 'TIME'
});

// Agent allocation
await PingPay.batchTransfer({
  from: profile.wallet,
  transfers: allocations
});

// Payment link
const link = await PingPay.createPaymentLink({
  recipient: 'mayor-sf.time-protocol.near',
  token: 'TIME'
});
```

---

## Security Model

### Agent Permissions

```rust
pub struct AgentPermissions {
    can_auto_allocate: bool,
    max_single_allocation_pct: u8,    // Max 20% default
    max_daily_reallocation: Balance,  // Max per day
    require_approval_above: Balance,  // Threshold
    notify_on_rebalance: bool,
    notify_on_new_laws: bool,
}
```

### Boundaries

| Agent CAN | Agent CANNOT |
|-----------|--------------|
| Discover governance | Move TIME without approval |
| Auto-allocate status quo | Exceed user-set limits |
| Monitor changes | Transfer to other accounts |
| Suggest reallocations | Stake beyond identity cap |
| Send notifications | Access precise location |

### Audit Trail

All agent actions logged on-chain:

```rust
pub struct AgentAction {
    action_type: String,
    description: String,
    timestamp: u64,
    details: String,  // JSON
}
```

---

## Data Ownership

All data is user-owned, portable, inspectable, and revocable.

### Export

```javascript
const myData = await contract.exportUserData(myAccount);
// Returns complete history
```

### Revoke

```javascript
await contract.revokeAgentPermissions();
```

---

## Deployment

### Testnet

```bash
# Build contract
cd contracts/rust
cargo build --target wasm32-unknown-unknown --release

# Deploy
near deploy \
  --wasmFile target/wasm32-unknown-unknown/release/time_protocol.wasm \
  --accountId time-protocol.testnet
```

### Frontend

```bash
cd frontend
npm install
npm run build
npm run start
```

---

## Future Enhancements

1. **Cross-chain TIME** via NEAR Intents
2. **Private inference** in TEEs for governance discovery
3. **DAO integration** for collective TIME allocation
4. **Mobile app** with push notifications
5. **NEAR Social** integration for governance discussions

---

*The Great Reset: January 1, 2034*

*"Your time is your vote."*
