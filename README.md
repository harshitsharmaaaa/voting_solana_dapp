# voting-dapp

A starter Solana decentralized application built with Next.js + TypeScript and an Anchor Rust program. This project demonstrates a minimal "SOL Vault" dapp where each wallet has a personal Program Derived Address (PDA) vault to deposit and withdraw SOL. It uses the Solana Foundation framework-kit client hooks, a Codama-generated type-safe program client, and an Anchor program for on-chain logic.

This README explains the architecture, how to run the app locally (using the pre-deployed devnet program), how to build/deploy your own program, run tests, and regenerate the TypeScript client after changes.

Table of Contents
- Why this project
- Highlights / Features
- Tech stack
- Project structure
- How it works (high level)
- Quick start (run locally)
- Deploy your own vault program
- Testing
- Regenerating the TypeScript client
- Useful files & where to look
- Development tips
- Contributing & license

---

Why this project
- Showcases a small full-stack Solana dapp with a React/Next frontend and an Anchor Rust program backend.
- Useful as a learning starter for wallet integration, PDA usage, Anchor programs, and codama-generated clients.

Highlights / Features
- Wallet connection via @solana/react-hooks (auto-discovery of connectors)
- SOL Vault program (deposit / withdraw to a personal PDA)
- Type-safe TypeScript client generated from Anchor IDL (Codama)
- Next.js 16 + TypeScript frontend with Tailwind CSS for styling
- Example test setup for Anchor programs

Tech stack
- Frontend: Next.js, React, TypeScript
- Styling: Tailwind CSS
- Solana client & hooks: @solana/client, @solana/react-hooks
- Program client: Codama-generated TypeScript client (in repo)
- On-chain program: Anchor (Rust)
- Testing: LiteSVM and Anchor testing tools (see anchor/README.md)

Project structure (high level)
- app/ — Next.js app (React Server/Client components)
  - components/providers.tsx — SolanaProvider setup (devnet by default)
  - components/vault-card.tsx — Vault UI, deposit/withdraw flows
  - generated/vault/ — Codama-generated program client (type-safe wrappers)
  - page.tsx — Main page that ties components together
- anchor/ — Anchor workspace with the Rust program (vault)
  - programs/vault/src/ — Rust program source and tests
  - README.md — Anchor-specific instructions and pre-deployed program ID
- codama.json — Configuration for Codama client generation
- package.json, tsconfig.json, etc. — front-end tooling

How it works (overview)
- The frontend creates a Solana client using @solana/client and wraps the app with SolanaProvider (app/components/providers.tsx). This provider exposes hooks such as useWalletConnection, useBalance, and useSendTransaction for easy interactions with wallet connectors and RPC.
- The vault program is an Anchor program that maintains a personal vault PDA per user. The PDA is derived from the user's wallet address and the program ID, so each wallet has a unique vault account on-chain.
- The frontend uses the Codama-generated program client (app/generated/vault) to construct and send deposit and withdraw instructions using the program's IDL in a type-safe way.
- VaultCard (app/components/vault-card.tsx) handles the deposit form, withdraw button, and displays vault balance + status.

Quick start (run locally, using pre-deployed devnet program)
Prerequisites
- Node.js (>= 18 recommended) and npm
- Optional for program development / deploy: Rust, Solana CLI, Anchor

Clone & run
```bash
git clone https://github.com/harshitsharmaaaa/voting_solana_dapp.git
cd voting_solana_dapp
npm install
npm run dev
```

Open http://localhost:3000, connect your wallet (Phantom / Solflare / other connectors discovered automatically), switch the wallet to devnet, and interact with the example vault.

Notes:
- The frontend default RPC endpoint is devnet: "https://api.devnet.solana.com" (see app/components/providers.tsx).
- The repo includes an Anchor program that is already deployed to devnet (see anchor/README.md for the program ID). If you prefer to deploy your own program (below), follow the Deploy steps.

Deploy your own vault (build & deploy the Anchor program)
Prerequisites
- Rust (via rustup)
- Solana CLI
- Anchor (cargo install --git https://github.com/coral-xyz/anchor --tag <release> or follow Anchor docs)
- A funded wallet for deployment (solana airdrop for devnet or a funded mainnet wallet)

Steps (typical)
1. Configure Solana to devnet (or your target cluster):
```bash
solana config set --url devnet
```

2. Create/fund a keypair (if needed), then airdrop:
```bash
solana-keygen new --outfile ~/.config/solana/id.json
solana airdrop 2
```

3. Build and deploy the Anchor program:
```bash
cd anchor
anchor build
anchor keys sync      # updates program ID in source where used
anchor build
anchor deploy
cd ..
```

4. Regenerate the client and restart the frontend:
```bash
npm run setup    # (rebuilds the program and regenerates the client)
npm run dev
```

After deploying, ensure the frontend uses the correct program ID (the Codama-generated client will be regenerated with the new ID). See "Regenerating the TypeScript client" below.

Testing
- The project includes Anchor tests and uses LiteSVM for fast testing.
- Typical commands:
```bash
npm run anchor-build   # builds the program
npm run anchor-test    # runs Anchor unit/integration tests
```
- Unit tests live in anchor/programs/vault/src/tests.rs and will use the program ID declared in the code.

Regenerating the TypeScript client
- The repository uses Codama to generate a TypeScript client from the Anchor IDL (type-safe wrappers).
- If you change the Anchor program IDL or re-deploy with a new ID, regenerate the client:
```bash
npm run setup   # or: npm run anchor-build && npm run codama:js
```
- Generated client output is under app/generated/vault (imported by the frontend).

Useful files & where to look
- app/components/providers.tsx — Solana client + connectors setup (endpoint = devnet by default).
  - Change RPC endpoint or connectors here.
- app/components/vault-card.tsx — Main UI & logic for deposit/withdraw. It uses:
  - useWalletConnection, useSendTransaction, useBalance from @solana/react-hooks
  - getProgramDerivedAddress, getAddressEncoder, getBytesEncoder from @solana/kit
  - getDepositInstructionDataEncoder, getWithdrawInstructionDataEncoder, VAULT_PROGRAM_ADDRESS from the generated client
- app/generated/vault/ — Codama-generated client (TypeScript) — contains program constants like VAULT_PROGRAM_ADDRESS and instruction encoders.
- anchor/ — Anchor workspace for the vault program (Rust sources and tests).
- codama.json — Codama config used to create the TypeScript client from the IDL.

Examples / quick code pointers
- Providers (where the RPC endpoint and client are created):
```tsx
// app/components/providers.tsx
"use client";

import { SolanaProvider } from "@solana/react-hooks";
import { autoDiscover, createClient } from "@solana/client";

const client = createClient({
  endpoint: "https://api.devnet.solana.com",
  walletConnectors: autoDiscover(),
});

export function Providers({ children }) {
  return <SolanaProvider client={client}>{children}</SolanaProvider>;
}
```

- Vault UI & program interactions:
  - See app/components/vault-card.tsx for how deposit/withdraw flows call the Codama-generated encoders and send transactions. The code handles parsing SOL <-> lamports, shows vault balance, and displays transaction status.

Pre-deployed program (devnet)
- The repo includes an Anchor README (anchor/README.md) that documents the program being deployed on devnet (a program ID is included there). You can use that program directly on devnet to test the frontend without deploying.

Development tips
- Use Phantom or other popular Solana wallets; ensure they are set to devnet when testing with the pre-deployed program.
- For faster local development of on-chain logic, use Anchor + localnet (anchor localnet) or LiteSVM for quick unit tests.
- When you deploy a new program ID, don’t forget to run the regeneration step (Codama) so the frontend has the correct VAULT_PROGRAM_ADDRESS.
- If you modify the IDL or program state layout, regenerate the client and re-run frontend builds.

Security & production notes
- This is a learning template / starter — do not use as-is in production. Audit on-chain logic carefully before real funds are involved.
- For production rollouts:
  - Consider stricter checks in the program (account ownership checks, rent-exemption and lamports handling).
  - Use a reliable RPC provider (e.g., QuickNode, Alchemy, or your own RPC cluster).
  - Use a secure deployer wallet and follow Solana’s best practices for program upgrades.

Contributing
- Contributions and improvements are welcome. Please open issues or PRs on the repository.
- If you modify the Anchor program, include updated IDL and steps to regenerate the client.

License
- (Add your license here — e.g., MIT / Apache-2.0 / etc.) If the repo includes a LICENSE file follow that.

Acknowledgements & Resources
- Solana docs: https://solana.com/docs
- Anchor docs: https://www.anchor-lang.com/docs
- @solana/react-hooks / framework-kit: https://github.com/solana-foundation/framework-kit
- Codama: https://github.com/codama-idl/codama

Maintainer
- harshitsharmaaaa — this repository: https://github.com/harshitsharmaaaa/voting_solana_dapp

---

If you’d like, I can:
- Generate a ready-to-commit README.md with minor adjustments (add license text, badges).
- Add a short “Getting started in 5 steps” badge + commands at the top.
- Create a short CONTRIBUTING.md or developer checklist for deploying/updating the program.
Tell me which and I’ll produce the file ready to paste/commit.
