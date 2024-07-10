## **Blockchain Cheatsheet - Cryptography & Signatures**
---
### § Fundamentals

#### Cryptanalysis

- **Definition**: The art of decryption, which is the analysis and overcoming of cryptographic systems.

#### Cryptography

- **Definition**: The art of encryption, which is the practice of protecting information using ciphers.

#### Ciphers

- **Definition**: Rules used to encrypt data.
    - **Symmetric**: Uses the same key for encryption and decryption.
    - **Asymmetric**: Uses a pair of keys, a public key to encrypt and a private key to decrypt.
- **Protocols**: Sets of rules that determine how encryption and decryption operations should be performed.
- **Properties of Valid Ciphers**:
    1. Easy to encrypt
    2. Easy to transmit
    3. Easy to decode
    4. Hard to decode if intercepted
    5. Source of message should be validated

- **Implementing Knowledge through Code**:

```Rust
```
	
---
## § Symmetric Cyphers

#### Monoalphabetic Symmetric Ciphers

- **Definition**: Use a single fixed substitution between plaintext and ciphertext.

Example of Cipher Alphabet (Inverse)

|  Alphabet   |   A   |   B   |   C   |   ...   |   K   |   L   |   M   |   N   |   O   |   ...   |   Z   |
| :---------: | :---: | :---: | :---: | :-----: | :---: | :---: | :---: | :---: | :---: | :-----: | :---: |
| **Inverse** | **Z** | **Y** | **X** | **...** | **P** | **O** | **N** | **M** | **L** | **...** | **A** |

Example of Encryption

|   H   |   E   |   L   |   L   |   O   |
| :---: | :---: | :---: | :---: | :---: |
| **S** | **V** | **O** | **O** | **L** |

- **Implementing Knowledge through Code**:

```Rust
```

#### Polyalphabetic Symmetric Ciphers

Phrase to encrypt: "HELLO WORLD" Repeated key: "KEYKEYKEYKE"

| Message           |  H  |  E  |  L  |  L  |  O  |  W  |  O  |  R  |  L  |  D  |
| ----------------- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| Repeated Key      |  K  |  E  |  Y  |  K  |  E  |  Y  |  K  |  E  |  Y  |  K  |
| Message (numbers) |  7  |  4  | 11  | 11  | 14  | 22  | 14  | 17  | 11  |  3  |
| Key (numbers)     | 10  |  4  | 24  | 10  |  4  | 24  | 10  |  4  | 24  | 10  |
| Sum mod 26        | 17  |  8  |  9  | 21  | 18  | 20  | 24  | 21  |  9  | 13  |
| Encrypted         |  R  |  I  |  J  |  V  |  S  |  U  |  Y  |  V  |  J  |  N  |

- **Implementing Knowledge through Code**:

```Rust
```


---
## § Symmetric Digital Signatures

Symmetric key Exchange
- **Key Usage**: Uses a single key for both signing and verification.
- **Speed**: Generally faster because it uses simpler algorithms.
- **Key Management**: Key distribution can be challenging since the same key must be shared securely between parties.
- **Use Case**: Commonly used in scenarios where both parties already share a secret key, like within closed systems.

#### Diffie Hellman Key Exchange

**Definition**: Diffie-Hellman Key Exchange is a secret-sharing algorithm that returns the components needed for arithmetic operations to generate a shared secret key.

**Process**:

1. **Establish Public Components**:
    - **Modulus (M)**: A large prime number used as the mathematical dividend.
    - **Generator (G)**: A base number used for exponentiation.
    
2. **Private Keys**:
    - Each party generates their own private key (**PK**).
    
3. **Arithmetic Operations**:
    - Each party performs the following operation using their private key: 
	    
		**G<sup>PK</sup> mod M**
		
    - The remainder (R) from this operation is shared between the parties.
	
4. **Secret Unveiling**:
    - Each party then takes the received remainder (**R**) and performs the following operation using their private key: 
	    
		**R<sup>PK</sup> mod M**
		
    - The final remainder (**LR**) will be the same for both parties and will serve as the common encryption and decryption key.
	
- **Implementing Knowledge through Code**:

```Rust
```
	
**Security**:
- No attacker can decipher the shared secret key (**LR**) by only knowing **G**, **M**, and **R** without access to the private keys (**PK**) of the parties involved.
	
	
---
## § Asymmetric Digital Signatures

RSA




---

**Author**: Kenneth Boldrini