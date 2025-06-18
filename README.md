# Incident Reporting App on Stellar

This project is a decentralized web application for reporting road incidents using Stellar smart contracts. It combines a Rust-based smart contract with a frontend built using Astro, providing a seamless interface to log incidents directly on-chain.

---

## Contract Details

* **Deployed Contract ID:** `CALH5YR6MDFTC5Z5NILDXKNXPEF3FX2Y33ULI4ELPMMHO2SSJMYYUKWE`

* **Check on block explorer:** https://stellar.expert/explorer/testnet/contract/CALH5YR6MDFTC5Z5NILDXKNXPEF3FX2Y33ULI4ELPMMHO2SSJMYYUKWE
* **Alias:** `incident-report`

---

## Developer Setup

### Smart Contract

1. Navigate to the contracts directory:

   ```bash
   cd stellar-contracts
   ```

2. Build the contract:

   ```bash
   stellar contract build
   ```

3. Run contract tests:

   ```bash
   cargo test
   ```

4. Upload the compiled contract to Stellar testnet:

   ```bash
   stellar contract upload \
     --network testnet \
     --source vasu \
     --wasm target/wasm32v1-none/release/incident_report.wasm
   ```

5. Deploy the contract:

   ```bash
   stellar contract deploy \
     --wasm-hash 9082a394bcab327186f1269e426d95385a32d60ebc47ff4f1ec3b7fbab897fd7 \
     --source vasu \
     --network testnet \
     --alias incident-report
   ```

---

### Web App

1. Navigate to the frontend directory:

   ```bash
   cd incident-report-app
   ```

2. Set up the environment:

   ```bash
   cp .env.example .env
   ```

3. Install dependencies:

   ```bash
   npm install
   ```

4. Run the development server:

   ```bash
   npm run dev
   ```

5. Access the app at:
   [http://localhost:4321/](http://localhost:4321/)

---

## Usage Guidelines

1. **Install Freighter Wallet** in your browser.
2. **Fund your wallet** with test tokens via the 
Friendbot:
   [Friendbot link](https://lab.stellar.org/account/fund?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;;)
3. **Connect your wallet** in the web app.
4. **Sign transactions** through Freighter to report incidents.

5. **Web app demo:** https://drive.google.com/drive/folders/1LHl2fMOLZe-ikh-zSZpR3aesWE58GhxY?usp=sharing

---

## References

* **Stellar Smart Contract Guide:**
  [https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)

* **Astro Web App Integration:**
  [https://developers.stellar.org/docs/build/apps/dapp-frontend](https://developers.stellar.org/docs/build/apps/dapp-frontend)

* **Freighter Wallet Integration:**
  [https://docs.freighter.app/docs/guide/usingfreighterwebapp/](https://docs.freighter.app/docs/guide/usingfreighterwebapp/)


