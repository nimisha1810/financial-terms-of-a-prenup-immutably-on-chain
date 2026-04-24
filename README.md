# 💍 Prenup Smart Contract (Soroban)

A minimal and immutable smart contract built with Soroban (Stellar’s smart contract platform) using Rust. This project demonstrates how prenuptial agreement terms can be securely recorded on-chain, ensuring transparency and tamper-proof storage.

---

## 📌 Overview

This smart contract enables two parties to store the financial terms of a prenuptial agreement on the Stellar blockchain. Once recorded, the data cannot be modified or overwritten, making it a reliable and immutable source of truth.

The contract is implemented in `src/lib.rs` and uses Soroban’s native storage system.

---

## ⚙️ Functionality

### 🔹 Create Prenup
Stores a new prenup agreement with:
- Unique identifier (`id`)
- Party A (`party_a`)
- Party B (`party_b`)
- Agreement terms (`terms`)

If the ID already exists, the transaction fails to preserve immutability.

---

### 🔹 Get Prenup
Retrieves an existing prenup record using its unique ID.

Returns a structured map containing:
- `party_a`
- `party_b`
- `terms`

---

## ✨ Key Features

- 🔒 **Immutable Records**  
  Once stored, agreements cannot be changed or deleted

- 🆔 **Unique Identifiers**  
  Prevents duplicate or overwritten prenups

- 📦 **On-Chain Storage**  
  Data is securely stored on the Stellar network

- 🔍 **Simple Retrieval**  
  Easily fetch agreement details anytime

- ⚡ **Lightweight & Efficient**  
  Minimal design for clarity and performance

---

## 🗂️ Project Structure

---

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable)
- Soroban CLI
- Stellar testnet account

---

### 🔧 Build

```bash
cargo build --target wasm32-unknown-unknown --release
🚀 Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/prenup_contract.wasm
🧪 Example Usage
Create a Prenup
create_prenup(
    id: symbol_short!("prenup1"),
    party_a: "Alice".into(),
    party_b: "Bob".into(),
    terms: "All pre-marital assets remain individually owned".into()
);
Retrieve a Prenup
get_prenup(symbol_short!("prenup1"));
⚠️ Important Considerations
All data is publicly accessible on-chain
No identity verification or authentication is implemented
No encryption of sensitive data
This contract is not legally binding by default
Intended for educational and demonstration purposes

🔮 Future Enhancements
🔐 Add authentication (Address + require_auth)
🧾 Store hashed or encrypted agreement terms
🕒 Include timestamps for audit trails
✍️ Enable multi-party digital signatures
📢 Emit events for indexing and transparency
🛠️ Tech Stack
Rust
Soroban SDK
Stellar Blockchain

📜 License

This project is licensed under the MIT License.

Wallet address:GDJX43JKOWXTWQPWQOD6VJCGF4P4RFZFBA2QJDGVDDADG2HZ5JG34FFX

contract address:CATGRHTKR3FUVDI2YKCCNJQCZSDS7DRL5YLH5IBZ6VSRDCBNKLMJ4GM3

<img width="1600" height="900" alt="image" src="https://github.com/user-attachments/assets/b5202e5d-6d8e-433d-bc8f-c29e0b081d7c" />
