# **Blockchain - Cheatsheet**
---

## § Fundamentals

#### Blockchain: A Peer-to-Peer Distributed Database

- **Definition**: Blockchain is a peer-to-peer distributed database technology where every machine (peer) acts as a node-block.
- **Mechanism**: Each block is linked to the next through a cryptographic hash.

- **Implementing Knowledge through Code**:

```Rust
```

#### Trust and Immutability

- **No Trust Needed**: Features of the blocks and ownership are registered in the immutable history of the chain (ledgers).
- **Consensus Control**: Consensus is always controlled by every block in the chain through Proof of Work or Proof of Stake.

- **Implementing Knowledge through Code**:

```Rust
```
#### Security and Efficiency

- **Ownership Verification**: This solves ownership verification and exchange in a secure way without a middle person.
- **Speed**: Data transfer times are significantly faster, almost instantaneous, which is useful for market exchanges and property-related transfers.
- **Core Features**: Security, speed, and ownership verification are the main features that make blockchain crucial for economic services.

- **Implementing Knowledge through Code**:

```Rust
```
#### Anti-Counterfeiting

- **Ledger Verification**: Blockchain solves counterfeiting by checking the ledgers of the blockchains.

- **Implementing Knowledge through Code**:

```Rust
```
#### Disruptive Potential

- **Applications**: The practical applications of blockchains are many: voting by unique tokens, IoT security, medical ecosystem enhancements, financial statements, secure process validations, transaction transparency for governance, passports, and more.

- **Implementing Knowledge through Code**:

```Rust
```

---
## § Blockchain Tech Bic Picture

#### Hashing Function

- A hashing function creates a "fingerprint" of the block elements in a dynamic way, used as a key to connect the blocks.

- **Implementing Knowledge through Code**:

```Rust
```

#### Private/Public Key System

- **Related to Each Other:** The private key and public key are mathematically related.
- **Easy to Trace back**: Private Key => Public Key 
- **Complicated to Trace back**: Public Key => Private Key

- **Implementing Knowledge through Code**:

```Rust
```

#### Public Address

- **Relation to Public Key:** The public address is related to the public key.
- **Derivation:** It can either be the public key itself or a value derived from the public key using a function.

- **Implementing Knowledge through Code**:

```Rust
```

#### Digital Signature Algorithms (DSA)

- **Proof of Ownership:** DSA proves who is the owner of the private key.
- **Verification without Revelation:** They allow verification of the signature without revealing the private key.

- **Implementing Knowledge through Code**:

```Rust
```

#### Transaction Mechanics

**UTXO Concept:** The system operates with the concept of UTXO (unspent transaction outputs), which represents the value that the block possesses and establishes the units that are unspent and spendable.

1. **Start:** Begin the transaction process.
2. **Verify unspent transaction outputs (UTXO):** Check the available UTXOs.
3. **Generate Keys (Sender):** The sender generates a new pair of private and public keys.
4. **Generate Keys (Recipient):** The recipient (Jenna) generates a new pair of private and public keys.
5. **Create Transaction:** Create a transaction to send 7 units to Jenna and 3 units to the sender as change.
6. **Sign Transaction:** The sender signs the transaction with their private key.
7. **Broadcast Transaction to Network:** The signed transaction is broadcast to the blockchain network.
8. **Validate Transaction:** Network nodes verify and validate the transaction.
9. **Update Blockchain:** The blockchain is updated with the new transaction.
10. **New UTXO:** The sender has a new UTXO of 3 units, while the old UTXO is now valueless.
11. **End:** End of the transaction process.

- **Implementing Knowledge through Code**:

```Rust
```

#### Cryptography

- **Integral Part of the Ecosystem:** Cryptography flows within the structure of the ecosystem.
- **Usage:** It is used for generating private keys and storing encrypted data in the block.

- **Implementing Knowledge through Code**:

```Rust
```

#### Consensus Mechanism

- **Different Methods:** There are different ways to achieve consensus, such as:
    - **Proof-of-Work (PoW):** Miners solve complex problems to validate transactions.
    - **Proof-of-Stake (PoS):** Major token holders create consensus, as they have the most interest in validating correct transactions.

- **Implementing Knowledge through Code**:

```Rust
```

#### Incentives

- **Purpose:** Incentives are designed to encourage participation in the system.
- **Proof-of-Work Systems:** In PoW systems, rewards are given to those who contribute to the well-being of the system, such as by validating transactions.
- **Rewards:** These rewards typically have some value and motivate participants to maintain the network.

- **Implementing Knowledge through Code**:

```Rust
```

---

Author: Kenneth Boldrini
