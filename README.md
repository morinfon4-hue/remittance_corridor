# remittance_corridor

remittance_corridor is a Stellar Level 3 dApp for a cross-border remittance corridor on Stellar testnet.

The project includes a Soroban smart contract, React frontend, Freighter wallet integration, verification scripts, deployment records, documentation, and GitHub Actions CI.

## Contract Deployment

<pre>
Network: Stellar testnet
Contract ID: CDEEHC2BY2O73N4OTIKKGLJ2IP442AFY3BEEOXGKRETUGV5LWFTRERTO
Contract Explorer: https://stellar.expert/explorer/testnet/contract/CDEEHC2BY2O73N4OTIKKGLJ2IP442AFY3BEEOXGKRETUGV5LWFTRERTO
</pre>

## Problem

Cross-border remittance flows often involve multiple parties such as sender, recipient, payout partner, and admin.

Without a transparent shared record, it can be difficult to track transfer status, payout progress, dispute state, and final release or refund.

## Solution

remittance_corridor demonstrates how a Stellar Soroban contract can model a remittance transfer lifecycle on Stellar testnet.

The project keeps the workflow simple for a Level 3 build: the contract manages transfer state, while the frontend presents a dashboard and prepares common remittance actions.

## Level 3 Scope

<pre>
Soroban smart contract
Contract tests
Contract WASM build
React frontend
Freighter wallet integration
Remittance dashboard
Prepared contract action flow
Frontend tests
Verification script
Deployment script
GitHub Actions CI
Documentation
</pre>

## Repository Structure

<pre>
remittance_corridor
|-- .github
|   +-- workflows
|       +-- ci.yml
|-- contracts
|   +-- remittance_corridor
|       +-- Cargo.toml
|       +-- src
|           +-- lib.rs
|           +-- test.rs
|-- docs
|   +-- ARCHITECTURE.md
|   +-- DEPLOYMENT_NOTES.md
|   +-- LOCAL_RUN_GUIDE.md
|   +-- QUALITY_AND_VERIFICATION.md
|-- evidence
|   +-- README.md
|   +-- SUBMISSION_CHECKLIST.md
|-- frontend
|   +-- package.json
|   +-- vite.config.ts
|   +-- src
|       +-- App.tsx
|       +-- contractConfig.ts
|       +-- services
|-- scripts
|   +-- deploy-and-save.ps1
|   +-- verify-level3.ps1
|-- CONTRACT_ID.txt
|-- DEPLOYMENT.md
|-- Cargo.toml
|-- Cargo.lock
|-- README.md
|-- vercel.json
+-- .gitignore
</pre>

## Smart Contract Scope

The contract represents a remittance corridor lifecycle.

Common lifecycle actions include:

<pre>
Open transfer
Fund transfer
Confirm payout
Open dispute
Resolve dispute
Release transfer
Refund transfer
Read transfer status
Read corridor statistics
</pre>

## Frontend Features

<pre>
Freighter wallet connection
Wallet address display
Contract ID display
Corridor metrics
Transfer form
Prepared action preview
Recent transfer list
Transaction status message
Responsive dashboard layout
</pre>

## Local Setup

<pre>
cd frontend
npm install
npm run dev
</pre>

## Contract Tests

<pre>
cargo test --workspace
</pre>

## Contract WASM Build

<pre>
cargo build --workspace --target wasm32v1-none --release
</pre>

## Frontend Tests

<pre>
cd frontend
npm test
</pre>

## Frontend Build

<pre>
cd frontend
npm run build
</pre>

## Full Verification

<pre>
powershell -ExecutionPolicy Bypass -File scripts/verify-level3.ps1
</pre>

## Deployment

<pre>
powershell -ExecutionPolicy Bypass -File scripts/deploy-and-save.ps1
</pre>

Deployment files:

<pre>
CONTRACT_ID.txt
DEPLOYMENT.md
</pre>

## Documentation

<pre>
docs/ARCHITECTURE.md
docs/QUALITY_AND_VERIFICATION.md
docs/DEPLOYMENT_NOTES.md
docs/LOCAL_RUN_GUIDE.md
evidence/SUBMISSION_CHECKLIST.md
</pre>

## Generated Files Policy

Do not commit generated files:

<pre>
target/
node_modules/
dist/
.vite/
contracts/**/test_snapshots/
*.tsbuildinfo
frontend/vite.config.js
frontend/vite.config.d.ts
.env files
deploy logs
</pre>

## Tech Stack

<pre>
Stellar testnet
Soroban
Rust
React
TypeScript
Vite
Vitest
Freighter wallet
PowerShell
GitHub Actions
</pre>

## Repository

<pre>
https://github.com/morinfon4-hue/remittance_corridor
</pre>

## License

MIT
