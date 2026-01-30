# 🏆 NEAR Innovation Sandbox Hackathon 2026

## Track: "AI That Works For You"

---

# 🌍 WHO GOVERNS YOU?

**A personal AI political agent for every human on Earth.**

---

## 💡 The Pitch (30 seconds)

> "Right now, 33 people govern you and 12,847 laws apply to you — based on where you're standing. You don't know who they are. Your agent does. It discovers them, monitors for changes, and notifies you when elections shift your representation. You close the tab. The agent keeps working."

---

## 🎯 The Problem

8 billion humans are governed by people they can't name, under laws they've never read.

- **Quiz:** Name your city council members. (You can't.)
- **Quiz:** How many federal regulations apply to your work? (You have no idea.)
- **Quiz:** Your state representative just lost their primary. (You didn't know there was an election.)

Democracy assumes informed citizens. The information exists — scattered across thousands of government websites, updated inconsistently, formatted for lawyers not humans.

**No one has time to research this. So no one does.**

---

## ✨ The Solution

**One AI agent per human that:**

1. **Discovers** every official who governs you (location-based)
2. **Counts** every law that applies to you
3. **Allocates** your political voice across the status quo
4. **Monitors** for changes (elections, new laws, redistricting)
5. **Notifies** you when something changes
6. **Suggests** actions (vote, contact, donate, organize)

**The killer feature:** You close the tab. The agent keeps working.

---

## 📊 Two Numbers

Based on a sample location (San Francisco, CA):

### The 33 People Who Govern You

| Level | Officials | Count |
|-------|-----------|-------|
| **City** | Mayor, Board of Supervisors, City Attorney, DA, Sheriff | 14 |
| **County** | (SF is city-county consolidated) | — |
| **State** | Governor, Lt. Gov, AG, State Senator, Assemblymember | 5 |
| **Federal** | President, VP, 2 Senators, 1 Representative | 5 |
| **Judicial** | Local judges → CA Supreme → SCOTUS | 9 |

**Click any name → voting record, contact info, upcoming elections, campaign finance**

### The 12,847 Laws That Apply to You

| Level | Category | Count |
|-------|----------|-------|
| **Federal** | Statutes, CFR regulations | 4,200 |
| **California** | State laws, admin code | 6,100 |
| **San Francisco** | Municipal code, ordinances | 2,547 |

**Search any keyword → plain-English summary, who voted for it, how to challenge it**

---

## 🤖 Agent Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         USER JOURNEY                                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. VERIFY                                                          │
│     └── World ID ZK proof → confirms human, preserves privacy       │
│                                                                      │
│  2. CLAIM                                                           │
│     └── Receive birthright TIME (1 token per day lived)             │
│     └── 30-year-old = 10,958 TIME to allocate                       │
│                                                                      │
│  3. DISCOVER (Agent takes over)                                     │
│     └── Agent queries: "Where are you?"                             │
│     └── GPS/IP → jurisdiction mapping                               │
│     └── Returns: "33 officials, 12,847 laws"                        │
│                                                                      │
│  4. ALLOCATE (Automatic)                                            │
│     └── Default: TIME distributed across status quo                 │
│     └── Weighting: Local 30%, State 25%, Federal 20%, etc.          │
│     └── User can override and reallocate manually                   │
│                                                                      │
│  5. MONITOR (Background)                                            │
│     └── Daily: Check for new laws, regulations                      │
│     └── Weekly: Scan upcoming elections in jurisdiction             │
│     └── On-event: Election results → rebalance allocations          │
│                                                                      │
│  6. NOTIFY                                                          │
│     └── "Your state senator lost their primary"                     │
│     └── "New city ordinance affects your neighborhood"              │
│     └── "Voter registration deadline in 14 days"                    │
│                                                                      │
│  7. SUGGEST                                                         │
│     └── "Contact your representative about [issue]"                 │
│     └── "Upcoming town hall you might attend"                       │
│     └── "Candidate aligned with your allocations needs volunteers"  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## ✅ Track Fit: "AI That Works For You"

| Requirement | Our Implementation |
|-------------|--------------------|
| **"Keeps working after user closes tab"** | Agent runs scheduled jobs: daily law checks, weekly election scans, real-time redistricting alerts |
| **"Clear boundary for what system can/cannot do"** | CAN: discover, monitor, notify, suggest. CANNOT: vote on your behalf, transfer TIME without explicit approval, take political action in your name |
| **"User-owned memory"** | All governance data stored on-chain (NEAR). User can: inspect full profile, export to any format, delete entirely, revoke agent permissions |
| **"Saves time"** | Replaces hours of research with instant answers: "Who represents me?" → 2 seconds |
| **"Unlocks workflow previously not practical"** | Continuous political awareness was impossible for individuals. Now it's automatic. |

---

## 🔗 Sponsor Integrations

### NOVA — Encrypted Governance Profile

Your political profile (location, allocations, engagement history) stored in NOVA's privacy vault. The agent accesses it to personalize recommendations. No one else can see it — not even us.

**Integration point:** `nova.store(governanceProfile)` / `nova.retrieve(agentKey)`

### HOT KIT — Omni-Chain Political Identity

Your TIME tokens may exist across multiple chains (NEAR, Base, Ethereum). HOT KIT provides unified balance view and seamless transfers.

**Integration point:** `hotkit.getBalance(worldId)` / `hotkit.transfer(from, to, amount)`

### PingPay — Agent-Initiated Donations

When you allocate TIME to a candidate or cause, your agent can accept micro-donations on their behalf. PingPay handles the payment rails.

**Integration point:** `pingpay.createCampaignLink(candidate)` / `pingpay.processDonation(amount)`

---

## 🏗️ Technical Architecture

### Smart Contract (Rust/NEAR)

```rust
// Core structures
pub struct Citizen {
    world_id_nullifier: String,      // ZK proof hash
    birthdate: u64,                   // For TIME calculation
    jurisdiction: Jurisdiction,       // Current location
    time_balance: u128,              // Birthright + earned
    staked_time: u128,               // Allocated to governance
    governance_profile: GovernanceProfile,
}

pub struct GovernanceProfile {
    officials: Vec<Official>,        // The 33 people
    laws: Vec<LawReference>,         // The 12,847 laws
    allocations: HashMap<EntityId, u128>,  // How TIME is distributed
    last_updated: u64,
}

// Key functions
fn claim_birthright(world_id_proof: Proof) -> Result<u128>;
fn discover_governance(location: Location) -> GovernanceProfile;
fn allocate_time(entity_id: EntityId, amount: u128) -> Result<()>;
fn get_notifications() -> Vec<Notification>;
```

### Frontend (Next.js + World ID)

```typescript
// World ID verification
const { result } = await IDKit.verify({
  app_id: process.env.WORLD_ID_APP_ID,
  action: "claim_time_birthright",
});

// Agent discovery
const profile = await agent.discoverGovernance(userLocation);
// Returns: { officials: [...], laws: [...], allocations: {...} }

// Continuous monitoring
useEffect(() => {
  const unsubscribe = agent.subscribe((notification) => {
    // "Your representative voted on..."
    // "Election results updated..."
  });
  return unsubscribe;
}, []);
```

---

## 📁 Repository Structure

```
NEAR-Innovation-Sandbox-2026/
├── README.md                    # Project overview (you are here)
├── HACKATHON.md                 # Full submission document
├── contracts/
│   └── rust/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs           # TIME Protocol smart contract
├── frontend/
│   ├── package.json
│   ├── .env.example
│   └── src/app/
│       ├── page.tsx             # Main UI with World ID
│       └── api/verify/route.ts  # ZK proof verification
└── docs/
    ├── ARCHITECTURE.md
    └── SPONSOR-INTEGRATIONS.md
```

---

## 🚀 Demo Flow

**Screen 1: Landing**
> "Who governs you right now?"
> [Verify with World ID]

**Screen 2: Verification**
> World ID orb scan / phone verification
> ZK proof generated (no identity revealed)

**Screen 3: Claim**
> "You are 34 years old. You have earned 12,419 TIME."
> "TIME = your political voice. 1 day = 1 TIME."
> [Claim Birthright TIME]

**Screen 4: Discovery**
> Agent working...
> "Based on your location:"
> **33 PEOPLE** govern you
> **12,847 LAWS** apply to you
> [See Details] [Auto-Allocate]

**Screen 5: Dashboard**
> Officials list (expandable)
> Law categories (searchable)
> Allocation sliders
> Notification feed
> "Close this tab — your agent keeps working."

---

## 👥 Team

### Herb Stephens
**Co-founder, Democracy Earth Foundation**
- TIME Protocol architect
- The Great Reset author
- Building governance systems since 2015

### Santiago Siri
**Co-founder, Democracy Earth Foundation**
- Democracy OS creator (2012)
- Proof of Humanity pioneer — **PoH #1**
- UBI Token architect

**Combined experience:** 20+ years building open-source democracy tools

---

## 🎯 Judging Criteria Response

| Criteria | Evidence |
|----------|----------|
| **Working, deployable product** | Smart contract compiles, frontend runs, World ID integration functional |
| **Engage with feedback** | Democracy Earth has iterated on governance tools since 2012. We ship and learn. |
| **Credible path to adoption** | World ID: 10M+ verified humans. NEAR: existing governance community. The Great Reset: Jan 1, 2034 target with detailed roadmap. |

---

## 🌍 The Vision

**Today:** You don't know who governs you.

**Tomorrow:** Every human has an agent that does.

**8 billion agents. 8 billion informed citizens. Real democracy.**

---

## 📜 License

MIT — Democracy Earth Foundation

---

*The Great Reset: January 1, 2034*

*"You can't govern yourself if you don't know who's governing you."*
