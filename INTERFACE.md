# YOD4COINS Interface Documentation

## Purpose
This interface is the human-facing layer of YOD4COINS. It allows users to experience the core properties of the protocol without needing to interact with the Rust backend directly.

## What the Interface Demonstrates

1. **Adaptive Consensus Modes**
   - Users can switch between Normal (3f+1), Partitioned (2f+1), and Extreme (1f+1).
   - The number of required local confirmations changes automatically.

2. **Offline Transaction Flow**
   - Create wallets with different risk tiers and offline spending limits.
   - Send value while “offline”.
   - Add local confirmations until the required threshold is met.
   - Settle the transaction once local finality is reached.

3. **Transparent Protocol Fee**
   - Every transaction shows the 0.1% fee in real time.
   - The fee is automatically split between the Founder allocation and the Host allocation.

4. **Risk Controls**
   - Offline spending limits differ by wallet tier (Standard / Business / Enterprise).
   - The interface prevents transactions that would exceed the offline limit.

## Design Principles
- Clarity over decoration
- Immediate feedback on every action
- Clear visual distinction between network modes
- Mobile-friendly so it can be shown on any device

## Future Direction
Once the protocol generates real usage and revenue, later versions of the interface will explore additional monetary stability features, including tools designed to help communities manage and reduce the effects of hyperinflation through disciplined local settlement and velocity-aware mechanisms.

## Current Status
This is a functional prototype interface. It simulates the protocol logic in the browser for demonstration and educational purposes. Production versions will connect to the actual Rust core and real cryptographic primitives.
