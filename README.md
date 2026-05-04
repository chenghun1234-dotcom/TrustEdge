# TrustEdge (트러스트에지) 🛡️

**"The World's First Real-time Attestation API"**

TrustEdge solves the massive operational burden and psychological pressure of **Genius Act**'s strict 1:1 reserve proof and monthly 3rd party attestation requirements. By using **Wasm** and **Edge Computing**, we provide a "Digital Iron Lung" for stablecoin and RWA issuers.

## 🚀 Key Features
- **Real-time PoR**: Perform 1:1 reserve proof every second at the Edge.
- **Wasm Logic Core**: Deterministic financial logic executed in a high-performance Rust-based Wasm sandbox.
- **Auto-Attestation**: One-click (or zero-click) generation of 3rd party attestation reports.
- **Zero Operating Cost**: Built on Cloudflare Workers Free Tier (No-AI architecture).

## 🛠 Tech Stack
- **Runtime**: Cloudflare Workers
- **Core Logic**: Rust (Wasm)
- **Database**: Cloudflare D1 (Audit Logs)
- **Frontend**: Vanilla HTML/CSS/JS (Glassmorphic Design)

## 📦 Getting Started

### 1. Prerequisites
- Node.js & npm
- Rust & `wasm-pack`
- Cloudflare Account

### 2. Installation
```bash
npm install
```

### 3. Build Wasm Core
```bash
npm run wasm:build
```

### 4. Local Development
```bash
npm run dev
```

### 5. Configuration (wrangler.toml)
Set your API keys in `wrangler.toml` or via `wrangler secret`:
- `ETHERSCAN_API_KEY`: Your Etherscan API key.
- `TOKEN_ADDRESS`: The contract address of the token to audit.
- `BANK_API_URL`: Your off-chain treasury API endpoint.
- `BANK_API_KEY`: Authentication for the bank API.

### 6. Deployment
```bash
# Initialize D1 Database
npx wrangler d1 create trust_db
npx wrangler d1 execute trust_db --file=schema.sql

# Deploy to Cloudflare
npm run deploy
```

## ⚖️ Mathematical Model (Proof of Solvency)
The verifier node ensures the following condition is met at any given time $t$:
$$A_{t} \ge L_{t} \quad (\forall t)$$
Where:
- $A_{t}$: Real-time verified bank balance sum (Edge Oracle).
- $L_{t}$: Total token supply on smart contracts.

---
**TrustEdge**: Because trust should be calculated, not inferred.
