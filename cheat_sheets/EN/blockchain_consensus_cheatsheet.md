# Blockchain Cheat Sheet - Consensus
---
##### **Table of Contents**
###### [§ Overview](#-Overview-1)
- [Consensus](#Consensus)
- [Nodes](#Nodes)
- [Needs](#Needs)
###### [§ Proof of Work (PoW)](#- Proof-of-Work-PoW)
- [Overview](#Overview)
- [Strengths](#Strengths)
- [Weaknesses](#Weaknesses)
- [Current PoW Systems](#Current-PoW-Systems)
###### [§ Profitability](#-Profitability-1)

---
## § Overview
	
### Consensus 
**Definition**: Consensus in blockchain refers to the mechanism through which nodes (independent computers connected in a network) agree on the state of a distributed ledger. It ensures that all transactions are valid and irreversible, according to rules defined by the consensus algorithm.
	
**Types**
- Majority = 51%
- Super-Majority = +66%
- Unanimous = 100%
- Weighted = Proof of Stake Votes are weighted based on the stake (or the amount of cryptocurrency held) by each node.
	
### Nodes
	
**Definition**: A node is a computer that runs software supporting a specific blockchain architecture, forming a part of the blockchain's distributed network.
	
##### Common nodes
	
**Mining Node**: Highly specialized and powerful computers that perform computations to propose new blocks. They receive mining rewards, covering the cost of their operations.
	
**Full Node**: Acts as a relay between the creation of blocks and their distribution. They maintain a complete copy of the blockchain's ledger and validate all transactions and blocks to ensure consistency and security.
	
**Light Node**: Acts as a relay between the creation of blocks and their distribution. They maintain a complete copy of the blockchain's ledger and validate all transactions and blocks to ensure consistency and security.
	
- **Implementing Concepts through Code**:
	
```Rust
```
	
### Needs
	
In a blockchain system, which is distributed and decentralized, a robust mechanism is essential since parties involved often cannot inherently trust each other. We need to ensure the integrity of the ledger so that the transaction history is reliable. This leads to the necessity of validating transactions without needing trust.
	
The consensus mechanism and its forms are designed to address these issues.
	
> **Consensus** is the process that allows trust in the outcome of a transaction or a block within a blockchain, without needing to trust the individual parties involved in the transaction or the entity that verifies it.
	
	
---
## § Proof of Work (PoW)
	
### Overview
	
- **Purpose**: Ensure the immutability of the blockchain.
- **How it is done**: _Nonces_ (Numbers Only Used Once) are added to the end of each block's hash to find a hash that meets a specific difficulty target, often requiring a certain number of leading zeros. This validates the block.
- **Why**: The system requires proof that computational work has been done. Finding a hash that meets the difficulty target is challenging and requires many attempts, demonstrating that significant computational work has been performed.
- **Security**: This process makes it difficult for anyone to alter the data without redoing all the computational work, thus enhancing the security of the blockchain.
	
### Strengths
	
- **Predictable Block Times**: Maintains a consistent time interval between blocks.
- **Fully Decentralized**: Allows any participant to contribute to network security.
- **High Cost of Attack**: The expense of achieving a 51% attack makes it unfeasible.
- **Uncensorable and Public**: Transactions and blocks are broadcasted publicly and cannot easily be censored.
	
### Weaknesses
	
- **High Energy Consumption**: PoW is resource-intensive, often criticized for its environmental impact.
- **Centralization of Mining Pools**: Can lead to potential centralization, as few pools might dominate the mining process.
- **Unfeasible for Standard Computers**: Mining has become impractical for ordinary computers due to the high computational requirements.
- **Variable Mining Profitability**: Mining profitability can fluctuate, sometimes making it less rewarding financially.
	
### Current PoW Systems
	
> For the most accurate and up-to-date information, I recommend performing quick research with the help of AI assistants to stay current with the latest developments in PoW systems.
	
	
---
###### Suggested Follow-up
[Blockchain Cheat Sheet - ](./.md)
  
---
  
**Author**: Kenneth Boldrini