
# 🎰 MVP Soloterie

Transparent and automated Web3 lottery project on Solana.

---

## 🧩 Features

### ✅ Lottery Creation

- Creation form on the frontend
- Automatic generation of a Solana smart contract with:
  - Calculation of ticket count (`maxSol / ticketPrice`)
  - Mint of X NFTs (one per ticket)
    - Backend creates a Metaplex NFT collection
    - Backend retains *mint authority*
    - Collection address stored in the smart contract
  - Definition of:
    - Resolution time
    - Draw method (e.g., Chainlink VRF)
- Automatic deployment of the smart contract
- Contract address stored to be displayed on frontend

---

### 🎟️ Ticket Sales

- Users send SOL to purchase a ticket
- Frontend fetches lottery info from the smart contract
- Purchase form available if the lottery is active:
  - Wallet connection (Phantom, Solflare…)
  - Validation that the lottery is still open
  - Mint of an NFT (ticket) via Metaplex (CPI or backend)
  - NFT sent directly to the user's wallet

---

### 🏆 Lottery Draw

- Random draw of the winning NFT from the collection
- Validate each NFT belongs to the collection (via Metaplex metadata)
- Look up wallet holding the winning NFT
- Send funds to the winner's wallet

---

### 📌 Notes

- Tickets are **NFTs** to ensure traceability and transparency
- A **closure mechanism** is required if the lottery isn't full:
  - Cancel & refund?
  - Partial draw?
- Final **payment mechanism** must ensure the recipient wallet is active

---

## 📚 Project Documentation

### ▶️ Usage

```bash
# Start the local validator
solana-test-validator
```

```bash
# In another terminal, run the tests
anchor test --skip-local-validator
```

This command:

- Compiles the Solana program
- Deploys the contract to the local validator
- Executes test scripts from `/tests`

---

### 🚧 Current Progress

- ✅ Lottery initialization: **WORKING**
  - Smart contract compiled and deployed
  - Test returns:  
    ```
    Nombre de tickets : 20
    ```
  - For parameters:
    - Total pool: `2 SOL`
    - Ticket: `0.1 SOL`
