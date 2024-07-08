# **Blockchain Cheatsheet - Hashing**
---

## § Overview

Hashing is not encryption because you cannot rebuild the original data from the hash as you can with encrypted files. 

- **Implementing Knowledge through Code**:

```Rust
```


We should consider hashing like a fingerprint; it provides a secure genetic reference to the data but is not the data "in person".

- **Implementing Knowledge through Code**:

```Rust
```

---
## § Key Characteristics of Good Cryptographic Hashing

1. **Speed**: It must be easy to compute to a certain extent because we don't want the algorithm to be easily brute-forced due to its speed.
2. **Deterministic**: The same input should always produce the same output.
3. **One-way**: It must be infeasible to recreate the original data from the hash.
4. **Secure**: If you alter the data to be hashed, you get a totally different hash, but if you re-alter back, you get the original hash.
5. **Concurrency-safe**: It is improbable for two different data sets to have the same hash value.

---
## § Salting

**Salting** is the practice of adding a random value to the hashed password stored. This is the only way to securely hash passwords.

- **Implementing Knowledge through Code**:

```Rust
```

---

**Author**: Kenneth Boldrini