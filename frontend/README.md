# TIME Protocol Frontend

Next.js 14 application with World ID integration for the TIME Protocol governance agent.

## Setup

### 1. Install dependencies

```bash
npm install
```

### 2. Configure World ID

1. Go to [developer.worldcoin.org](https://developer.worldcoin.org)
2. Create a new **Staging** application
3. Add an action called `claim_time_birthright`
4. Copy your App ID

### 3. Set environment variables

```bash
cp .env.example .env.local
```

Edit `.env.local` and add your World ID credentials:

```
NEXT_PUBLIC_WORLD_ID_APP_ID=app_staging_YOUR_APP_ID
NEXT_PUBLIC_WORLD_ID_ACTION=claim_time_birthright
WORLD_ID_APP_ID=app_staging_YOUR_APP_ID
WORLD_ID_ACTION=claim_time_birthright
```

### 4. Run the development server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000)

## User Flow

1. **Welcome Screen** - Introduction to TIME Protocol
2. **World ID Verification** - Prove you're human with ZK proof
3. **Birthdate Entry** - Calculate birthright TIME (1 per day lived)
4. **Agent Discovery** - AI discovers your governance structure
5. **Two Numbers** - See officials and laws that govern you
6. **Explore** - Drill down and reallocate your TIME

## World ID Integration

The app uses `@worldcoin/idkit` for World ID verification:

- **IDKitWidget** - Modal component for verification flow
- **verifyCloudProof** - Backend verification of ZK proofs
- **Nullifier** - Ensures one-person-one-claim

## Files

```
frontend/
├── src/
│   └── app/
│       ├── page.tsx              # Main app with all screens
│       └── api/
│           └── verify/
│               └── route.ts      # World ID proof verification API
├── package.json
├── tsconfig.json
├── .env.example
└── README.md
```

## Sponsor Integrations (Planned)

- **NOVA** - Encrypted governance profile storage
- **HOT KIT** - Multi-chain wallet connection
- **PingPay** - Fiat onramp and agent payments

## License

MIT - Democracy Earth Foundation
