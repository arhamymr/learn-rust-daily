# Rust Cryptography for Cybersecurity

This document is a **complete learning + implementation roadmap** to master cryptography using **Rust**, with a strong **cybersecurity mindset**. It focuses on *how crypto is used, how it fails, and how attackers abuse mistakes*.

> ⚠️ Rule #1: **Never invent your own cryptographic algorithm for production.**  
> Use this roadmap to *understand*, *implement for learning*, and *securely apply* cryptography.

---

## Phase 0 — Cryptography Mindset

### Core Security Goals

- Confidentiality
  - What it is: Preventing unauthorized access to data, both at rest and in transit. Only intended parties should learn the plaintext or derive sensitive information from it.
  - Typical threats:
    - Passive eavesdropping on networks (sniffing, on‑path attackers before TLS).
    - Data exfiltration from storage (stolen disks, leaked backups, misconfigured buckets).
    - Inference via metadata (message length, timing, access patterns).
  - How to achieve:
    - Use authenticated encryption (AEAD) for data and messages: AES‑GCM or ChaCha20‑Poly1305.
    - Use TLS for transport security; prefer modern cipher suites and enforce certificate validation.
    - Apply access control and least privilege; encrypt only sensitive fields when partial encryption suffices.
  - Rust patterns:
    - Use audited crates (aes-gcm, chacha20poly1305, rustls).
    - Use strong randomness for keys and nonces (rand with OS RNG).
    - Treat secrets as sensitive memory; wipe when done (secrecy, zeroize).
  - Pitfalls:
    - Reusing nonces/IVs with stream/AEAD ciphers.
    - Rolling your own padding, mode, or RNG.
    - Confusing encryption with hashing (never “encrypt” passwords).
  - Checklist:
    - AEAD chosen; unique nonce policy documented and enforced.
    - Keys generated from sufficient entropy and stored securely.
    - TLS correctly configured and verified on all links.

- Integrity
  - What it is: Ensuring data has not been altered (accidentally or maliciously) since creation or transmission.
  - Typical threats:
    - Tampering with messages or files in transit/storage.
    - Bit‑flips or partial overwrites in unreliable channels.
  - How to achieve:
    - Use MACs (e.g., HMAC) for messages when you share a secret key.
    - Prefer AEAD so confidentiality and integrity are coupled.
    - For files/artifacts distributed broadly, use digital signatures.
  - Rust patterns:
    - Use hmac with sha2 for keyed integrity; verify before use.
    - Prefer AEAD for application messages; reject on tag failure.
  - Pitfalls:
    - Using a bare hash (e.g., SHA‑256 alone) to “ensure integrity”.
    - Ignoring verification return values or truncating tags.
    - Length‑extension issues when misusing Merkle–Damgård hashes.
  - Checklist:
    - Every message/file has a MAC or signature.
    - Verification is constant‑time and fail‑closed.
    - Tags/signatures are stored/transmitted intact and compared fully.

- Authentication
  - What it is: Proving identity (entity authentication) and/or data origin (that a message really came from who it claims).
  - Typical threats:
    - Credential theft or replay; token forgery; service impersonation.
    - Confusing “encrypted” with “authenticated”.
  - How to achieve:
    - Users/services: password hashing with Argon2, multi‑factor, or mutual TLS.
    - Data origin: MACs with shared secrets, or digital signatures with public keys.
    - Protocols: challenge‑response with nonces/timestamps to prevent replay.
  - Rust patterns:
    - Passwords: store Argon2 hashes with unique salts; constant‑time comparisons.
    - Services: rustls for TLS; pin or properly validate certificates where appropriate.
    - Messages: MAC with HMAC for shared‑secret peers; signatures for third‑party verifiability.
  - Pitfalls:
    - Storing plaintext or fast hashes (MD5/SHA‑1/SHA‑256 without KDF).
    - Using encryption as “proof of identity”.
    - Omitting nonces/timestamps, enabling replay.
  - Checklist:
    - Strong password KDF parameters set and tested.
    - Mutual authentication where needed (client and server).
    - Nonces/timestamps validated; anti‑replay in place.

- Non-repudiation
  - What it is: Preventing a sender from plausibly denying having performed an action (e.g., sending a message or approving a transaction).
  - Typical threats:
    - Shared‑key systems where any party could have produced the MAC.
    - Missing audit trails or weak time‑stamping.
  - How to achieve:
    - Digital signatures with unique private keys (e.g., Ed25519).
    - Append‑only, verifiable logs with timestamps and hash chains.
    - Clear key ownership, issuance, and revocation procedures.
  - Rust patterns:
    - Use ed25519‑dalek for signatures; distribute and pin public keys.
    - Sign structured, canonicalized data with domain separation (context strings) to avoid ambiguity.
    - Log signature digests and timestamps; protect logs against tampering.
  - Pitfalls:
    - Using HMAC for non‑repudiation (any holder can forge).
    - Poor key management (lost/compromised keys invalidate claims).
    - Ambiguous data formats (different byte representations complicate verification).
  - Checklist:
    - Actions/events are signed with a unique private key.
    - Public keys are discoverable and bound to identities.
    - Audit logs are integrity‑protected and time‑anchored.

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

