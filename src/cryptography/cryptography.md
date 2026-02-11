# Rust Cryptography for Cybersecurity

This document is a **complete learning + implementation roadmap** to master cryptography using **Rust**, with a strong **cybersecurity mindset**. It focuses on *how crypto is used, how it fails, and how attackers abuse mistakes*.

> ⚠️ Rule #1: **Never invent your own cryptographic algorithm for production.**  
> Use this roadmap to *understand*, *implement for learning*, and *securely apply* cryptography.

---

## Phase 0 — Cryptography Mindset

### Core Security Goals
- Confidentiality
- Integrity
- Authentication
- Non-repudiation

### Non‑negotiable Rules
- ❌ Do not roll your own crypto
- ❌ Do not reuse nonces / IVs
- ❌ Do not encrypt passwords
- ✅ Always use audited libraries
- ✅ Always define a threat model

---

## Phase 1 — Foundations (Week 1)

### Concepts
- Symmetric vs Asymmetric cryptography
- Hashing vs Encryption
- Entropy & randomness
- One-way functions
- Threat modeling

### Rust Practice
- Implement Caesar cipher (toy)
- Break it using frequency analysis

**Purpose:** attacker intuition

---

## Phase 2 — Hashing & Password Security (Week 2)

### Why This Matters
Most real-world breaches leak **password hashes**, not plaintext.

### Concepts
- Salt & pepper
- Key stretching
- Timing attacks
- Password verification

### Rust Crates
```toml
argon2
rand
zeroize
```

### Implement
- Password hashing with Argon2
- Password verification
- Secure salt generation
- Memory zeroization

---

## Phase 3 — Symmetric Encryption (Week 3)

### Concepts
- AES
- Stream vs block ciphers
- Authenticated Encryption (AEAD)
- Nonce reuse attacks

### Rust Crates
```toml
aes-gcm
chacha20poly1305
rand
```

### Implement
- AES-GCM encrypt/decrypt
- Tamper detection
- Secure nonce generation

---

## Phase 4 — Integrity & Authentication (Week 4)

### Concepts
- MAC vs hash
- HMAC
- Replay attacks

### Rust Crates
```toml
hmac
sha2
```

### Implement
- HMAC signing
- HMAC verification
- Replay attack demo

---

## Phase 5 — Asymmetric Cryptography (Week 5)

### Concepts
- Public/private keys
- Digital signatures
- Key exchange
- RSA vs ECC
- Forward secrecy

### Rust Crates
```toml
ed25519-dalek
x25519-dalek
rsa
```

### Implement
- Digital signature creation
- Signature verification
- Diffie–Hellman key exchange

---

## Phase 6 — Secure Key Management (Week 6)

### Concepts
- Key derivation functions (KDF)
- Key rotation
- Secret handling
- Memory safety

### Rust Crates
```toml
ring
secrecy
zeroize
```

### Implement
- Key derivation from passwords
- Secure secret storage
- Prevent secret leakage

---

## Phase 7 — TLS & HTTPS (Week 7)

### Concepts
- TLS handshake
- Certificates & CAs
- MITM attacks
- Certificate pinning

### Rust Crates
```toml
rustls
webpki
```

### Implement
- TLS client
- Certificate validation
- Pinning example

---

## Phase 8 — JWT & Token Security (Week 8)

### Concepts
- Signing vs encryption
- Algorithm confusion attacks
- Token expiration
- Refresh tokens

### Rust Crates
```toml
jsonwebtoken
ed25519-dalek
```

### Implement
- JWT signing
- JWT verification
- Exploit insecure JWT configs

---

## Phase 9 — Cryptographic Attacks (Week 9)

### Attacks to Understand
- Padding oracle
- Timing attacks
- Replay attacks
- Nonce reuse
- Weak randomness

### Practice
- Implement vulnerable crypto
- Exploit it intentionally

---

## Phase 10 — Capstone Projects (Week 10)

Choose at least one:

- 🔐 Secure password manager (CLI)
- 🧾 Encrypted file storage tool
- 💬 End-to-end encrypted chat
- 🪪 Authentication server (JWT + Argon2)
- 🧪 Crypto audit of a Rust project

---

## Recommended Rust Crypto Stack

| Purpose | Crate |
|------|------|
| Hashing | argon2, sha2, blake3 |
| Symmetric | aes-gcm, chacha20poly1305 |
| Asymmetric | ed25519-dalek, x25519-dalek |
| TLS | rustls |
| Secrets | secrecy, zeroize |
| RNG | rand |

---

## Final Advice

> Crypto usually fails **not because algorithms are broken**, but because developers use them incorrectly.

If you master this roadmap, you will:
- Understand real-world crypto systems
- Spot cryptographic vulnerabilities
- Build secure Rust applications
- Think like both defender and attacker

🦀🔐 **Rust + Cryptography = Elite Cybersecurity Skillset**

