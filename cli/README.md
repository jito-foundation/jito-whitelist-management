# Jito Whitelist Management CLI

## Whitelist Management

### Whitelist

#### Get

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    get \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```

#### Initialize

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    initialize \
    --initial-admin BBBATax9kikSHQp8UTcyQL3tfU3BmQD9yid5qhC7QEAA \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```


#### Add Admin

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    add-admin \
    --new-admin B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```

#### Remove Admin

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    remove-admin \
    --admin-to-remove B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```

##### Squads

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    remove-admin \
    --admin 9eZbWiHsPRsxLSiHxzg2pkXsAuQMwAjQrda7C7e21Fw6 \
    --admin-to-remove BBBATax9kikSHQp8UTcyQL3tfU3BmQD9yid5qhC7QEAA \
    --print-tx \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```

#### Add To Whitelist

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    add-to-whitelist \
    --signer-to-add B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```

#### Remove From Whitelist

```bash
cargo r -p jito-whitelist-management-cli -- \
    whitelist-management \
    whitelist \
    remove-from-whitelist \
    --signer-to-remove B8cPx1rmsbhjc6yE1u47kNhgkkva14a1TpXemHYquGny \
    --rpc-url https://api.devnet.solana.com \
    --signer ~/.config/solana/id.json \
    --commitment confirmed \
    --jito-whitelist-management-program-id Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G
```
