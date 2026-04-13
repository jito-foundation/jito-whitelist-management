# Jito Whitelist Management Program

## Overview

Jito Whitelist Management is a Solana on-chain program that controls access via an address whitelist.
It was built as part of the [Jito x Coinbase collaboration (JIP-33)](https://forum.jito.network/t/jip-33-a-jito-x-coinbase-collaboration/923/6) to authorize specific signers for deposit-stake operations.

**Key capabilities:**
- Initialize a whitelist PDA supporting up to 64 whitelisted addresses and 8 admins
- Admin instructions: `AddAdmin`, `RemoveAdmin`, `AddToWhitelist`, `RemoveFromWhitelist`

## Getting Started

### Build IDL

```bash
make build-idl
```

### Build Client

```bash
pnpm install

make build-client
```

### Build Program

```bash
make build-sbf
```

### Test

```bash
make test
```

## Security Audits

| Group    | Date       | Commit                                                                        |
|----------|------------|-------------------------------------------------------------------------------|
| Certora  | 2026-03-20 | [9ca381a](security_audits/certora_jito_coinbase_integration_audit_v0.1.pdf)   |

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](./LICENSE) file for details.


## References

- https://forum.jito.network/t/jip-33-a-jito-x-coinbase-collaboration/923/6
