# **Blockchain Cheat Sheet - Technical Use**
---
##### § Addresses
- Use Cases
- The steps
##### § Cryptotransactions
- Analogy
- Mechanics of transactions
- Validation of the Proposal
- Cryptotransactions in Depth 
##### § Scalability

##### § Machine to Machine
---
## § Addresses<br>
#### Use Cases
- Sign transaction with public key to Identify and Validation of data.
- Anyone that possesses the public key can Identify and Validate data.<br>
#### The steps
1. Generating Key Pairs:
	- Create ***Private Key*** :: `256 bit or 64 hexadecimal Chars`<br>Randomly.
	- Derive ***Public Key Base*** :: `512 bits or 128 Hexadecimal Chars`<br>   We use the ***Private Key*** with the *Elliptic Curve Digital Signature Algorithm* <br>   ( Algorithm => x_coordinate-256bits + y_coordinate-256bits = 512 bits ***Public Key Base*** ).
1. Hashing ( Ethereum ):
	- Hash ***Public Key*** :: `From 512 bits to 256 bits or 64 Hexadecimal Chars`<br>   Hash the ***Public Key Base*** with *Kekkak-256 *or *Sha-3*.
3. Generating Public Address ( Ethereum ):
	- Create ***Public Address*** :: `From 64 Hexadecimal Chars to 42 Hexadecimal Chars`<br>   Take last 40 Hexadecimal Chars (20 bytes) and prefix with 0x to 42 Hexadecimal Chars.<br>
- **Implementing Concept through Code**:<br>
```Rust
```
<br><br>
---
## § Cryptotransactions<br>
#### Analogy<br>
Suppose that the parties **A**, **B**, and **C** each have a _lockbox_ which contains content that travels through the Blockchain Protocol System, which enforces the rules of how everything works. These _lockboxes_ have a slot that only accepts inward content, and the only way to retrieve the content is with the private key of the owner.<br>
- **Implementing Concept through Code**:<br>
```Rust

```
<br><br>
#### Mechanics of transactions<br>
**A Sends -> to B Data or Cryptocurrency**
1. **B** Creates ***Public Address*** and ***Public Key*** from ***Private Key***:
	- **B** ***Private Keys*** :: **B** ***Public Address*** and ***Public Key***
2. **B** Sends the ***Public Address*** -> to **A** (*Public address can change for every transaction*).
	- **B** ***Public Address*** -> to **A** 
3. **A** will add the ***Public Address*** of **B** and the data or amount to a "Transaction" Message:
	- **A** Initialize Transaction :: **B** Public Address and Content
4. **A** will Sign the transaction with the ***Digital Signature***:
	- ***Digital Signature*** :: Derive from **A**'s own Private Key
		With the *Elliptic Curve Digital Signature Algorithm* ( x_coordinate-256bits + y_coordinate-256bits ).
5. **A** Transaction is *<ins>Proposed</ins>* by the blockchain protocol in the *Memory Pool*:
	- ***Validation*** :: Miners attempt to validate the transaction by including it in a block from the memory pool.<br>
- **Implementing Concept through Code**:<br>
```Rust
```
<br>
#### Validation of the Proposal<br>
**B then Sends -> to C** 
- We need to check before imprinting the transaction in the Blockchain that B has effectively the content necessary to be sent again:<br>
	**B**'s Transaction is Sent -> to **Blockchain Memory Pool** then the Protocol Sends -> to **C**<br>
- **Implementing Concept through Code**:<br>
```Rust
```
<br>
#### Bitcoin Cryptotransactions in Depth <br>
**Bitcoin vs Ethereum**
- Bitcoin: We need to think Every transaction as a container of an unique unspent cryptocurrency which is not mixed with the others.
- Ethereum: Different from Bitcoin has an accounting system which keeps track of the total balance.<br>
- **Implementing Concept through Code**:<br>
```Rust
```
<br>
**Transaction management**
Cryptocurrency as it is linked to transaction containers 
that we will name *X*-Trsct-C*n* (*X* = ID, Trsct = Transaction , C*n* = Container Number) 
then need to be accessed manipulating the container to handle it.<br>
**A Sends 10 Bitcoin -> to B from a Transaction container that has 20 Bitcoin**
1. **B** Creates ***Public Address*** and ***Public Key*** from ***Private Key***
2. **B** Sends the ***Public Address*** -> to **A** (*Public address can change for every transaction*)
3. **A** will add the ***Public Address*** of **B** and the amount to a "Transaction" Message<br><br>
4. The *New empty Transaction container* (**A**-Trsct-C4) will Take an input and will send one <ins>or two</ins> outputs, the import and the eventual change:
	- The input is based on the Transaction containers that has the unspent Cryptocurrency or UTXO ( Unspent Transaction Output ) which covers the import of The New Transaction
		**A**-Trsct-C1 = 10 Bitcoin
		**A**-Trsct-C2 = 30 Bitcoin  -> Input
		**A**-Trsct-C3 = 5 Bitcoin
	- The first output will be the import of The New Transaction
		**A**-Trsct-C4  = 20 Bitcoin -> Output to **B**-Trsct-C1
	- <ins>The Optional output will be the change which is sent back to the sender A</ins>
		**A**-Trsct-C4 = 10 Bitcoin -> Output to **A**-Trsct-C4
		!!!
		**A**-Trsct-C2 = 30 Bitcoin is then Destroyed<br><br>
5. **A** will Sign the transaction with the ***Digital Signature***
6. **A** Transaction is *Proposed* by the blockchain protocol in the *Memory Pool*
7. Validation of the *Proposal*<br>
- **Implementing Concept through Code**:<br>
```Rust
```
<br>
**Proof of Work Validation**
	


	
	
---
## § Scalability

## § Machine to Machine

	
	
---
###### Suggested Follow Up
[[]]
	
---
	
**Author**: Kenneth Boldrini