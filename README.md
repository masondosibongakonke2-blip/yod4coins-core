# YOD4COINS

**Offline-first settlement protocol with adaptive Byzantine consensus**

YOD4COINS is an offline-first transaction and state protocol designed for partitioned, constrained, and high-latency environments. It enables local finality when central networks fail and performs deterministic reconciliation when connectivity returns.

It processes pre-existing digital value (tokenized assets / digital fiat representations) and includes a transparent protocol-level utility fee.

---

## Core Idea

Most digital value systems assume permanent global connectivity. When that assumption breaks (power outages, network partitions, sparse coverage, or disruption), economic activity freezes.

YOD4COINS is built from the opposite assumption: the network will fail. The protocol is designed to keep operating under progressive degradation.

### Adaptive Local Consensus

| Mode          | Threshold | Description                                      |
|---------------|-----------|--------------------------------------------------|
| Normal        | 3f+1      | Standard local quorum                            |
| Partitioned   | 2f+1      | Network split or high latency                    |
| Extreme       | 1f+1      | One local hardware node + human cryptographic key |

- Local finality with hard offline spending limits  
- Sequential nonces + explicit state locks for double-spend protection  
- Deterministic reconciliation on rejoin  
- Transparent 0.1% protocol utility fee with automatic split

---

## Edge Risk Controls

Most payment systems only run fraud and compliance checks when they can reach central servers. When the network drops, they either freeze or allow transactions with almost no checks.

YOD4COINS moves a first layer of risk control to the edge:

- **Edge Compliance Layer** — local identity attributes, velocity checks, and hard offline spending limits by risk tier
- **Edge AI Bandwidth Filter** — suppresses routine traffic and elevates anomalous or high-value activity
- Strong double-spend resistance through nonces and state locks even while offline

These controls continue to function during network failure.

---

## Key Components

- **YODPAY** — First practical application (offline payments)
- **Slide 10** — Shared edge coordination engine
- **YOD4COINS** — Value and incentive layer built on the same foundation

One coherent architecture. Multiple applications.

---

## Repository Structure
yod4coins-core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── fees.rs
├── cpp/                  # C++ reference (optional)
├── docs/
│   └── TOKENOMICS.md
├── LICENSE
└── README.md

---

## Protocol Utility Fee

Every settlement incurs a fixed **0.1% (10 basis points)** protocol utility fee.  
The fee is calculated deterministically and can be automatically split between:

- Founder / Architect treasury  
- Host jurisdiction / sovereign treasury  

This logic is designed to be enforced at the protocol level.

---

## Tokenomics Summary

- Fixed maximum supply set at genesis  
- Founder allocation with multi-year vesting  
- Node and validator rewards  
- Ecosystem / community allocation  
- Transparent 0.1% protocol utility fee  

See `docs/TOKENOMICS.md` for full details.

---

## Current Status

- Core offline state machine and adaptive consensus logic implemented (Rust)
- Fee calculation and split logic included
- Working prototypes available under the broader Gideon Mesh project
- Not production-audited — formal security review required before real-value deployment

Live prototype:https://codepen.io/editor/Sibongakonke-Masondo/pen/01a164da-6a8a-77a0-a7e4-7e2c3b2d5257

---

## Important Notes

- This is foundational architecture, not production-ready software.
- Real deployment requires cryptographic audits, proper key management, and legal review.
- Human authority remains a hard requirement in extreme isolation mode.

---

##Author

Sibongakonke Masondo
Systems Architect  
South Africa

---

##License

MIT License
