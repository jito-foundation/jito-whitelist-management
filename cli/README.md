# Jito Whitelist Management CLI

## Whitelist Management

### Whitelist

#### Get

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    get \
    --base 6Ji1UAx1y6MKFAR3JKgxkVJYubXahexc5e5CYEHzfqBK \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```

#### Initialize

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    initialize \
    --base ./target/deploy/base.json \
    --initial-admin BBBATax9kikSHQp8UTcyQL3tfU3BmQD9yid5qhC7QEAA \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```


#### Add Admin

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    add-admin \
    --base 6Ji1UAx1y6MKFAR3JKgxkVJYubXahexc5e5CYEHzfqBK \
    --new-admin B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```

#### Remove Admin

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    remove-admin \
    --base 6Ji1UAx1y6MKFAR3JKgxkVJYubXahexc5e5CYEHzfqBK \
    --admin-to-remove B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```

#### Add To Whitelist

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    add-to-whitelist \
    --base 6Ji1UAx1y6MKFAR3JKgxkVJYubXahexc5e5CYEHzfqBK \
    --signer-to-add B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-bam-boost-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```

#### Remove From Whitelist

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    remove-from-whitelist \
    --base 6Ji1UAx1y6MKFAR3JKgxkVJYubXahexc5e5CYEHzfqBK \
    --signer-to-remove B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id 8ZHoBFEvf1cNR1ewSDkYcKEp4tMvjTXDkRAADPsBoBDq
```
