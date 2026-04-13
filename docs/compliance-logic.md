# Stellar RWA Guard: Compliance & Regulatory Framework

This document outlines how the **Stellar RWA Guard** smart contract integrates with existing Stellar Ecosystem Proposals (SEPs) to ensure institutional-grade compliance for tokenized assets.

## 1. SEP-12: Customer Information Integration
The RWA Guard acts as the on-chain "Gatekeeper" for SEP-12. 
- **The Flow:** Once a user completes KYC/AML via a regulated anchor (SEP-12), their public key is pushed to our `RwaGuard` smart contract.
- **The Result:** Only users with a `WL_DONE` (Whitelist Done) status in the contract storage can call the `transfer` function for a protected asset.

## 2. SEP-30: Account Recovery for Regulated Assets
For institutional assets, losing a private key shouldn't mean losing the asset.
- Our roadmap includes **SEP-30 support**, allowing compliance officers to re-issue tokens to a new verified wallet address if a user loses access, while maintaining a full audit trail on-chain.

## 3. Jurisdictional "Guardrails"
The contract is designed to support **Multi-Region Whitelisting**. 
- **Global Compliance:** We use Soroban's persistent storage to tag users by country code (e.g., US, UK, SG).
- **Rule Enforcement:** Issuers can set rules like "Only users with the 'SG' tag can trade this real estate token," preventing cross-border regulatory violations automatically.

## 4. Auditor Access & Transparency
To satisfy "Travel Rule" requirements:
- The contract includes a read-only `view_compliance_status` function.
- This allows third-party auditors (like Chainalysis or Elliptic) to verify that an issuer is enforcing their whitelist in real-time without needing private keys.
