# NEAR Innovation Sandbox 2026 — Submission

## Your Governance Agent

### Track: "AI That Works For You"

---

## One Sentence

**An AI agent that shows you exactly who governs your decisions — so you can allocate your time, money, and energy to local people and initiatives you actually know.**

---

## The Problem We Solve

### The Governance Blindness Problem

Every human is governed by dozens of officials and thousands of laws. **Nobody knows who they are.**

Ask anyone:
- *"How many elected officials represent you right now?"* — They won't know.
- *"Which laws affect your business decision?"* — No idea.
- *"Who votes on zoning in your neighborhood?"* — Blank stare.

**Result:** People make major life decisions — starting businesses, buying homes, relocating, supporting initiatives — completely blind to the governance that will determine outcomes.

### The Abstract Investment Problem

People are told to "invest" in:
- Index funds of 500 companies they'll never visit
- Bonds of governments they can't influence
- Tokens with anonymous teams they'll never meet

**Meanwhile:**
- Their neighbor needs $5K for a bakery expansion
- Their local school needs volunteer coders
- Their community has a solar initiative seeking participants

**There's no system connecting people to local opportunities. No agent showing them what's nearby, what's relevant, what they can actually influence.**

### The Parallel Society Problem

Movements want to build "parallel societies" but lack infrastructure:
- No identity system independent of governments
- No economic system for local exchange
- No governance that emerges from participation
- No way to SEE the existing system they're paralleling

---

## Our Solution

### The Governance Agent

An AI agent that:

1. **Discovers your governance** — Shows you exactly who governs you based on your location
2. **Contextualizes decisions** — "Before you start that business, here's who approves permits"
3. **Surfaces local opportunities** — Businesses, initiatives, people near you seeking support
4. **Enables local allocation** — Support what you know, not abstract instruments
5. **Tracks changes** — Elections, new laws, council votes — you're always informed

### The Two Numbers

Wherever you are standing:

```
┌──────────────┐    ┌──────────────┐
│      33      │    │   12,847     │
│   PEOPLE     │    │    LAWS      │
│  govern you  │    │  apply to    │
│    here      │    │    you       │
└──────────────┘    └──────────────┘
```

Tap either number. See exactly who. See exactly what.

**This is the foundation.** Everything else builds on knowing who governs you.

---

## Track Fit: "AI That Works For You"

### Requirement Mapping

| Track Requirement | Our Delivery |
|-------------------|--------------|
| **Autonomous system** | Agent discovers governance, monitors changes, surfaces opportunities — no user action needed |
| **Works after tab closes** | Background jobs watch elections, law changes, local opportunities |
| **Clear boundaries** | Agent CAN: discover, inform, suggest. Agent CANNOT: transact without approval |
| **User-owned memory** | All data on NEAR blockchain — user controls, exports, deletes |
| **Payments in workflow** | TIME token enables local allocation directly through agent |
| **Real job-to-be-done** | "Help me understand who governs my decisions and what's local" |
| **One bounded job** | Governance discovery → decision context → local allocation |

### Why This Is Different

**Not a chatbot.** The agent actively discovers your governance stack and monitors it.

**Not a voting app.** This is about EVERY decision, not just elections.

**Not DeFi.** This is about LOCAL allocation to things you know, not yield farming.

**Not abstract.** Shows your actual officials, your actual laws, your actual neighbors.

---

## Multi-Chain Architecture

### Why Three Chains

| Chain | Purpose | Why This Chain |
|-------|---------|----------------|
| **NEAR** | AI agent, governance logic, local coordination | Best AI/agent infrastructure, fast, cheap |
| **Worldchain** | Identity (World ID) | One person = one agent, sybil-resistant |
| **Ethereum** | TIME token liquidity, DeFi bridges | Deepest liquidity, widest reach |

### Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│                       USER                                   │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    WORLDCHAIN                                │
│                                                             │
│   World ID Verification                                     │
│   • Prove humanity (ZK proof)                               │
│   • One person = one agent                                  │
│   • Nullifier prevents duplicates                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                       NEAR                                   │
│                                                             │
│   Governance Agent (AI)                                     │
│   • Jurisdiction detection                                  │
│   • Official/law discovery                                  │
│   • Decision context engine                                 │
│   • Local opportunity matching                              │
│   • Change monitoring                                       │
│                                                             │
│   Smart Contracts                                           │
│   • Governance profiles                                     │
│   • Work NFTs (soulbound)                                   │
│   • Local allocation tracking                               │
│   • Staking + voting                                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                     ETHEREUM                                 │
│                                                             │
│   TIME Token (ERC-20)                                       │
│   • Main liquidity pools                                    │
│   • DeFi integrations                                       │
│   • Bridge to NEAR                                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## The Agent in Action

### Use Case 1: Starting a Business

**User:** "I want to open a coffee shop in my neighborhood."

**Agent Response:**
```
GOVERNANCE AFFECTING YOUR COFFEE SHOP

Permits Required:
├─ Business License — City Clerk (Maria Rodriguez)
├─ Food Handler Permit — Health Dept (Dr. James Chen)  
├─ Building Permit — Planning Dept (if renovating)
└─ Sign Permit — if exterior signage

Key Officials:
├─ Your City Councilmember: District 5, votes on zoning
├─ Planning Commission: Meets 2nd Tuesday
└─ Health Inspector: Serves your area

Relevant Regulations: 47 apply to food service businesses
├─ SF Health Code Section 4
├─ CA Retail Food Code
└─ [View all 47]

LOCAL INTELLIGENCE

Coffee Suppliers in Network (within 25 mi):
├─ Bay Roasters Collective — 3.2 mi, accepts TIME
├─ Mission Coffee Co-op — 5.1 mi, seeking partners
└─ Local Beans Direct — 8.4 mi

Similar Businesses Nearby:
├─ 4 coffee shops within 1 mile
├─ Nearest: "Corner Cup" — 0.3 mi (potential competitor OR collaborator?)

Community Sentiment:
├─ Neighborhood Association: Supportive of local business
├─ Recent petition for more cafes: 234 signatures
```

**The agent doesn't just tell you to "do research." It does the research.**

### Use Case 2: Local Allocation

**User:** "I have $500. I want to support something local."

**Agent Response:**
```
LOCAL ALLOCATION OPPORTUNITIES (within 10 miles)

BUSINESSES SEEKING SUPPORT

Maria's Panaderia — 2.1 mi
├─ Seeking: $5,000 for commercial oven
├─ Raised: $3,200 (64%)
├─ Your $500 would: Complete the round
├─ Returns: 5% revenue share for 2 years
├─ Governance: Permitted, health-inspected, 3yr track record
└─ [Allocate with TIME]

TechRepair Collective — 1.8 mi  
├─ Worker-owned electronics repair
├─ Seeking: New members ($500 buy-in)
├─ Returns: Share of profits, voting rights
├─ Governance: Registered co-op, 12 current members
└─ [Allocate with TIME]

COMMUNITY INITIATIVES

Neighborhood Solar Project — Your block
├─ Goal: $15,000 for shared solar installation
├─ Raised: $8,500 (57%)
├─ Your $500 would: Reduce your electricity cost ~15%
├─ Governance: HOA-approved, permitted
└─ [Allocate with TIME]

Local School Coding Program
├─ Need: Volunteer instructors OR $2,000 for equipment
├─ Your $500 would: Buy 5 Raspberry Pi kits
├─ Impact: 30 students in your district
└─ [Allocate with TIME]
```

**Not "investment." Allocation. To people you can meet, outcomes you can see.**

### Use Case 3: Relocation Decision

**User:** "I'm thinking of moving from SF to Austin."

**Agent Response:**
```
GOVERNANCE COMPARISON: SF → AUSTIN

                        San Francisco    Austin
─────────────────────────────────────────────────
Total Officials               33            29
Total Laws                 12,847         8,234

CHANGES IF YOU MOVE

Officials You'd Lose: 33
├─ Mayor London Breed
├─ Supervisor Matt Dorsey  
├─ Governor Gavin Newsom
├─ Senator Scott Wiener
└─ [all 33]

Officials You'd Gain: 29
├─ Mayor Kirk Watson
├─ Council Member (District TBD based on address)
├─ Governor Greg Abbott
├─ Your State Rep (District TBD)
└─ [all 29]

KEY POLICY DIFFERENCES
├─ State income tax: 13.3% CA → 0% TX
├─ Property tax: 1.18% CA → 1.80% TX
├─ Business regulations: Higher CA → Lower TX
├─ Zoning: Strict SF → Looser Austin
└─ [Full comparison]

LOCAL NETWORK IMPACT
├─ Your current local connections: 47 people, 12 businesses
├─ Known connections in Austin: 3 people
└─ Recommendation: Build Austin network before moving
```

**Every major life decision has a governance dimension. The agent shows it.**

---

## The Parallel Society Layer

### This Isn't Just An App — It's Infrastructure

```
PARALLEL SOCIETY OPERATING SYSTEM
─────────────────────────────────

Layer 1: IDENTITY
└─ World ID — humanity verified, no government

Layer 2: AWARENESS  
└─ Governance Agent — see who governs you

Layer 3: ECONOMY
└─ TIME token — work, earn, trade locally

Layer 4: GOVERNANCE
└─ Stake TIME → voting power in parallel structures

Layer 5: COORDINATION
└─ Find others under same governance, build together
```

### The Agent's Role

The Governance Agent is **Layer 2** — the awareness layer that makes everything else possible.

You can't build parallel structures if you can't see what you're paralleling.
You can't allocate locally if you can't find local opportunities.
You can't make informed decisions if you don't know who governs the outcomes.

**The agent makes the invisible visible.**

---

## TIME Token Economics

### Not "Investment" — Allocation

We deliberately avoid the word "investment." Investment implies:
- Passive — give money, hope it grows
- Abstract — don't know where it goes
- Extractive — take returns, give nothing else

**Allocation implies:**
- Active — choose based on knowledge
- Concrete — know exactly where it goes
- Participatory — you're part of the outcome

### How TIME Works

| Source | How You Earn |
|--------|--------------|
| **Birthright** | 1 TIME per day you've lived (UBI) |
| **Work** | 1 TIME per hour contributed (verified) |
| **Local participation** | Bonus TIME for local allocation |

| Use | What Happens |
|-----|--------------|
| **Allocate locally** | Support businesses/initiatives you know |
| **Trade** | Swap for BTC, ETH, USD when needed |
| **Stake** | Lock TIME → gain voting power |
| **Govern** | Vote on parallel community decisions |

### Local Multiplier

When you allocate TIME locally (within your governance jurisdiction):
- **Standard allocation**: 1:1
- **Local allocation**: 1.1:1 (10% bonus to recipient)

**The system incentivizes local over abstract.**

---

## Technical Implementation

### Agent Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   GOVERNANCE AGENT                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │   DISCOVERY     │  │    CONTEXT      │                  │
│  │                 │  │                 │                  │
│  │ • Location      │  │ • Parse user    │                  │
│  │ • Jurisdiction  │  │   intent        │                  │
│  │ • Officials     │  │ • Match to      │                  │
│  │ • Laws          │  │   governance    │                  │
│  │ • Boundaries    │  │ • Relevant      │                  │
│  │                 │  │   advice        │                  │
│  └─────────────────┘  └─────────────────┘                  │
│                                                             │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │     LOCAL       │  │   MONITORING    │                  │
│  │                 │  │                 │                  │
│  │ • Nearby        │  │ • Elections     │                  │
│  │   businesses    │  │ • Law changes   │                  │
│  │ • Initiatives   │  │ • Council       │                  │
│  │ • People in     │  │   votes         │                  │
│  │   network       │  │ • Redistricting │                  │
│  │ • Opportunities │  │ • Opportunities │                  │
│  └─────────────────┘  └─────────────────┘                  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Smart Contracts (NEAR)

| Contract | Purpose |
|----------|---------|
| `governance_profile.rs` | Store user's discovered governance |
| `time_token.rs` | TIME token (NEP-141) |
| `work_nft.rs` | Soulbound work verification (NEP-171) |
| `local_allocation.rs` | Track local allocations |
| `staking.rs` | Lock TIME for voting power |

### Data Sources

| Data | Source |
|------|--------|
| Officials | Google Civic API, OpenStates, local gov APIs |
| Laws | Legal databases, municipal code APIs |
| Local businesses | User-submitted, verified through agent |
| Initiatives | Community-submitted, staked for legitimacy |

---

## Demo Script

### 1. Onboarding (30 seconds)

```
[Screen: Welcome]
"Your Governance Agent"
"Know who governs you. Allocate locally."

[Action: Sign in with World ID]
→ QR code scan
→ "Verified. You're human. One agent, just for you."
```

### 2. Discovery (30 seconds)

```
[Screen: Discovery]
→ Location detected (or entered)
→ Agent working animation

[Screen: Results]
"33 people govern you here."
"12,847 laws apply to you."

→ Tap "33 people"
→ Drill down: Federal → State → Local
→ Each official with name, role, relevance
```

### 3. Decision Context (45 seconds)

```
[Screen: Ask Agent]
User types: "I want to start a food truck"

[Agent Response]
→ Permits required (with official names)
→ Regulations that apply
→ Local food truck community
→ Suppliers in network
→ "Would you like to connect with existing food truck operators?"
```

### 4. Local Allocation (45 seconds)

```
[Screen: Local Opportunities]
→ Businesses near you seeking support
→ Community initiatives
→ People in your network

[Action: Allocate 50 TIME to Maria's Panaderia]
→ Transaction preview
→ "This supports a local business 2.1 miles away"
→ Confirm with World ID
→ Transaction on NEAR
→ Work NFT minted to your profile (supporter badge)
```

### 5. Ongoing (15 seconds)

```
[Screen: Notification Preview]
"Your city council votes on food truck regulations next Tuesday."
"New coffee roaster joined the network (3.4 mi from you)."
"Election results: Your new supervisor is [Name]."

"Close the app. Your agent keeps watching."
```

---

## Sponsor Integrations

### NOVA — Private Storage

Your governance profile is sensitive. NOVA encrypts it.

- Officials who govern you → encrypted
- Your allocation history → encrypted
- Your local connections → encrypted

Agent accesses via intents. You control access.

### HOT KIT — Multi-Wallet

Connect any wallet. See TIME across chains.

- Ethereum TIME ↔ NEAR TIME
- Any wallet welcome
- Unified balance view

### PingPay — Local Payments

Agent-initiated allocations to local opportunities.

- "Support Maria's Panaderia" → one click
- Fiat onramp for new users
- Payment links for local initiatives

---

## Why We Win

### 1. Real Problem, Real Solution

Nobody else is building "governance discovery + local allocation."

DeFi apps: Abstract yield farming
Civic apps: Just show representatives
Local apps: No governance integration

**We combine them into one agent.**

### 2. AI-Native Architecture

The agent isn't a chatbot wrapper. It:
- Actively discovers your governance
- Monitors for changes
- Contextualizes your decisions
- Surfaces relevant opportunities

**This is what "AI that works for you" means.**

### 3. Movement Infrastructure

This isn't an app. It's the operating system for parallel society.

Thousands of people want to build alternatives. They lack infrastructure. We provide it.

### 4. Real Team, Real Track Record

Democracy Earth has been building decentralized governance since 2015:
- Proof of Humanity: Live, used by World ID
- UBI Token: Distributed to 40,000+ humans
- Democracy OS: Used by governments and organizations

**We ship.**

---

## Post-Hackathon Roadmap

### This Week → Demo

### Q1 2026 → Public Beta
- Governance discovery for US jurisdictions
- Local opportunity network (pilot cities)
- TIME token on NEAR mainnet

### Q2 2026 → Scale
- 100+ jurisdictions
- Mobile app
- Local business onboarding

### 2027 → 1M Users
- Global jurisdiction coverage
- Full parallel economy features
- Governance federation

### 2034 → The Great Reset
- Every human has an agent
- Complete parallel infrastructure
- [github.com/herbstephens/NEAR-Innovation-Sandbox-2026](https://github.com/herbstephens/NEAR-Innovation-Sandbox-2026))

---

## Team

**Herb Stephens** — Co-founder, Democracy Earth Foundation
- Architect of TIME Protocol
- 10+ years in decentralized governance
- Based in Portugal

**Santiago Siri** — Co-founder, Democracy Earth Foundation
- Created Proof of Humanity
- Advisor to Worldcoin
- Based in Buenos Aires

---

## Links

- **This Repo**: github.com/herbstephens/NEAR-Innovation-Sandbox-2026
- **Democracy Earth**: democracy.earth

---

*"Investment is giving resources to strangers. Allocation is supporting what you know. Your Governance Agent shows you the difference — starting with who actually governs your life."*

---

**NEAR Innovation Sandbox 2026**
**Track: AI That Works For You**
**Team: Democracy Earth Foundation**
