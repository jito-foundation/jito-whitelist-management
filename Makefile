.PHONY: build-sbf clean test clippy fmt idl-build client-build

build-sbf:
	cargo-build-sbf --manifest-path program/Cargo.toml

# IDL Build
build-idl:
	cargo r -p jito-shank-cli -- \
        --program-env-path ./config/program.env \
        --output-idl-path ./program/idl/ \
        generate \
        --program-id-key "JITO_WHITELIST_MANAGEMENT_PROGRAM_ID" \
        --idl-name jito_whitelist_management \
        --module-paths "sdk" \
        --module-paths "program" \
        --module-paths "core"

# Client Build
build-client:
	pnpm codama run --all

# Test
test:
	make build-sbf && \
	cp ./target/sbpf-solana-solana/release/jito_whitelist_management_program.so ./program/tests/fixtures/ && \
	SBF_OUT_DIR=$(pwd)/target/sbpf-solana-solana/release cargo nextest run
