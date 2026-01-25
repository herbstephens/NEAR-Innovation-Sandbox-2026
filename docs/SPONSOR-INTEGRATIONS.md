# Sponsor Integrations

TIME Protocol integrates three sponsor technologies to deliver a complete, privacy-first, multi-chain governance agent with programmable payments.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                      TIME Protocol MVP                               │
│              "AI Governance Agent" + Sponsor Stack                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐                │
│  │    NOVA     │   │  HOT KIT    │   │  PingPay    │                │
│  │  (Storage)  │   │  (Wallet)   │   │ (Payments)  │                │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘                │
│         │                 │                 │                        │
│  ┌──────▼─────────────────▼─────────────────▼──────┐                │
│  │                                                  │                │
│  │  ONBOARDING                                      │                │
│  │  ├─ Connect wallet (HOT KIT - any chain)        │                │
│  │  ├─ Verify humanity (World ID)                  │                │
│  │  └─ Buy TIME with fiat (PingPay Onramp)         │                │
│  │                                                  │                │
│  │  DISCOVERY                                       │                │
│  │  ├─ Agent finds officials & laws                │                │
│  │  └─ Profile stored encrypted (NOVA Vault)       │                │
│  │                                                  │                │
│  │  ALLOCATION (Agent-Initiated Payments)          │                │
│  │  ├─ Agent reads preferences (NOVA)              │                │
│  │  ├─ Agent builds intent (HOT KIT)               │                │
│  │  └─ Agent executes payment (PingPay API)        │                │
│  │                                                  │                │
│  │  SHARING                                         │                │
│  │  └─ "Support Official X" link (PingPay Links)   │                │
│  │                                                  │                │
│  └──────────────────────────────────────────────────┘                │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔐 NOVA Integration — Private Memory

**Why NOVA?** Political preferences are sensitive. Governance profiles should be encrypted and user-controlled.

### What We Store

| Data | Privacy Need | NOVA Benefit |
|------|--------------|--------------|
| Governance profile | Political leanings are sensitive | Encrypted vault |
| Allocation weights | Reveals priorities | User-controlled access |
| Agent audit log | Personal activity trail | Zero-exposure persistence |
| Voting history | Private until shared | Selective disclosure |

### Implementation

```typescript
// Store governance profile in encrypted NOVA vault
const vault = await Nova.createVault({
  name: 'time-governance-profile',
  encryption: 'user-owned',
  accessControl: 'intent-based'  // Agent accesses via intents
});

// Store sensitive data
await vault.store({
  jurisdiction: 'san-francisco-ca',
  officials: discoveredOfficials,        // 33 officials
  allocations: currentAllocations,        // TIME distribution
  preferences: { localWeight: 0.30, ... },
  auditLog: agentActions                  // Full history
});

// Agent retrieves via intent (zero-exposure)
const profile = await Nova.intentQuery(vault, {
  action: 'get-allocations',
  purpose: 'rebalance-after-election',
  expiry: '1h'
});

// User can selectively share
const shareLink = await vault.createShareLink({
  fields: ['allocations.local'],  // Only local allocations
  recipient: 'campaign-xyz',
  expiry: '7d'
});
```

### Prize Alignment

- ✅ **Privacy Vaults**: Encrypted governance profile
- ✅ **User-owned AI**: Agent operates on encrypted data via intents
- ✅ **Zero-exposure**: Profile never leaves vault unencrypted
- ✅ **Selective sharing**: User controls what to reveal

---

## 🔗 HOT KIT Integration — Multi-Chain Wallet

**Why HOT KIT?** Users should connect any wallet (ETH, NEAR, etc.) and see TIME across all chains.

### Features We Use

| HOT KIT Feature | TIME Protocol Use |
|-----------------|-------------------|
| Multi-wallet connect | Support ETH, NEAR, Solana wallets |
| Omni-balances | Show TIME across all chains |
| Token swaps | Any token → TIME conversion |
| Intent builder | Stake TIME via intents |

### Implementation

```typescript
import { HotKit } from '@aspect-build/hot-kit';

// 1. Connect any wallet
const wallet = await HotKit.connect({
  supportedChains: ['near', 'ethereum', 'base'],
  preferredChain: 'near'
});

// 2. View TIME across all chains
const omniBalance = await HotKit.getOmniBalances(wallet.address, 'TIME');
// Result: { near: 18250, ethereum: 0, base: 500, total: 18750 }

// 3. Swap any token to TIME
const swap = await HotKit.swap({
  from: { token: 'USDC', chain: 'ethereum', amount: '100' },
  to: { token: 'TIME', chain: 'near' },
  slippage: '0.5%'
});

// 4. Build staking intent
const stakeIntent = HotKit.buildIntent({
  action: 'stake',
  amount: '1000',
  token: 'TIME',
  target: 'governor-ca.time-protocol.near',
  constraints: {
    maxSlippage: '0.5%',
    deadline: Date.now() + 3600000  // 1 hour
  }
});

// 5. Execute intent (HOT KIT finds best solver)
const result = await HotKit.executeIntent(stakeIntent);
```

### Prize Alignment

- ✅ **Multi-chain wallet**: Connect from any chain
- ✅ **Omni-balances**: See TIME everywhere
- ✅ **Intent-based flows**: Stake via intents
- ✅ **Flexible toolkit**: Powers our payment layer

---

## 💳 PingPay Integration — Agent Payments

**Why PingPay?** Our agent initiates payments autonomously — exactly what PingPay is built for.

### Features We Use

| PingPay Feature | TIME Protocol Use |
|-----------------|-------------------|
| Ping Onramp | Fiat → TIME (buy birthright with USD) |
| Ping API | Agent executes TIME allocations |
| Payment Links | "Support Official X" shareable links |
| AI + Payments | Agent-initiated governance payments |

### Implementation

```typescript
import { PingPay } from '@pingpay/sdk';

// 1. Fiat onramp: USD → TIME
const onramp = await PingPay.createOnramp({
  fiatAmount: 100,          // $100 USD
  fiatCurrency: 'USD',
  outputToken: 'TIME',
  outputChain: 'near',
  recipient: wallet.address,
  metadata: { type: 'birthright-topup' }
});

// Redirect user to Ping checkout
window.location.href = onramp.checkoutUrl;

// 2. Agent-initiated allocations (THE KEY FEATURE)
async function agentAllocateTime(profile, allocations) {
  const batch = [];
  
  for (const [officialId, amount] of Object.entries(allocations)) {
    batch.push({
      to: `${officialId}.time-protocol.near`,
      amount: amount,
      token: 'TIME',
      memo: JSON.stringify({
        type: 'agent-allocation',
        reason: 'status-quo-distribution',
        timestamp: Date.now()
      })
    });
  }
  
  // Execute batch payment via PingPay API
  const result = await PingPay.batchTransfer({
    from: profile.wallet,
    transfers: batch,
    authorization: 'agent-permission-token'
  });
  
  // Log to audit trail
  await Nova.vault.append('auditLog', {
    action: 'AGENT_ALLOCATION',
    transfers: result.transfers,
    txHash: result.hash
  });
  
  return result;
}

// 3. Create shareable payment link
const supportLink = await PingPay.createPaymentLink({
  recipient: 'mayor-sf.time-protocol.near',
  suggestedAmounts: [10, 50, 100],
  token: 'TIME',
  title: 'Support Mayor London Breed',
  description: 'Allocate TIME to local governance',
  metadata: { official: 'mayor-sf', level: 'local' }
});

// Result: https://ping.pay/link/abc123xyz
```

### Prize Alignment

- ✅ **AI + Payments**: Agent initiates TIME allocations autonomously
- ✅ **Ping Onramp**: Fiat → TIME for easy onboarding
- ✅ **Ping API**: Programmatic payment execution
- ✅ **Payment Links**: Shareable "support official" links

---

## 📊 Summary

| Sponsor | Integration | Prize Fit |
|---------|-------------|-----------|
| **NOVA** | Encrypted governance profile vault, agent accesses via intents | ⭐⭐⭐⭐⭐ |
| **HOT KIT** | Multi-chain wallet, omni-balances, intent-based staking | ⭐⭐⭐⭐⭐ |
| **PingPay** | Agent-initiated payments, fiat onramp, payment links | ⭐⭐⭐⭐⭐ |

---

## User Flow with All Sponsors

| Step | Action | Sponsor |
|------|--------|---------|
| 1 | Connect wallet (ETH, NEAR, etc.) | **HOT KIT** |
| 2 | Verify with World ID | Core |
| 3 | Buy TIME with credit card | **PingPay Onramp** |
| 4 | OR Claim birthright TIME | Core |
| 5 | Agent discovers governance | Core |
| 6 | Profile stored encrypted | **NOVA Vault** |
| 7 | Agent auto-allocates TIME | **PingPay API** |
| 8 | View omni-balance | **HOT KIT** |
| 9 | Share "Support X" link | **PingPay Links** |
| 10 | Export profile privately | **NOVA** |

---

*All three sponsors integrate naturally with TIME Protocol's core design: privacy-first, multi-chain, and agent-driven.*
