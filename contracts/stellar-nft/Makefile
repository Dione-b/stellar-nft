DIONE_ADDRESS ?= GC3U3JGCPJPPQEY6PY6DMFMJBQ7D4B5YPWGQR4JVYRNW5KZSJWVBXLWV
CONTRACT_WASM_PATH = ../../target/wasm32-unknown-unknown/release/stellar_nft.wasm

# Default values for parameters
TOKEN_ID ?= 0
OWNER ?= $(STELLAR_SOURCE_ACCOUNT)
BASE_URI ?= $(STELLAR_NFT_CONTRACT_BASE_URI)
CONTRACT_ID ?= $(STELLAR_NFT_CONTRACT_ID)
STELLAR_NFT_CONTRACT_MAX_SUPPLY ?= 1000
STELLAR_NFT_CONTRACT_NAME ?= NFT
STELLAR_NFT_CONTRACT_SYMBOL ?= NFT
STELLAR_NFT_CONTRACT_BASE_URI ?= https://example.com

debug:
	@echo "=== Environment Variables ==="
	@echo "NETWORK: $(NETWORK)"
	@echo "SOURCE: $(SOURCE)"
	@echo "CONTRACT_NAME: $(STELLAR_NFT_CONTRACT_NAME)"
	@echo "CONTRACT_SYMBOL: $(STELLAR_NFT_CONTRACT_SYMBOL)"
	@echo "CONTRACT_URI: $(STELLAR_NFT_CONTRACT_URI)"
	@echo "CONTRACT_MAX_SUPPLY: $(STELLAR_NFT_CONTRACT_MAX_SUPPLY)"
	@echo "CONTRACT_WASM_PATH: $(CONTRACT_WASM_PATH)"
	@echo "TOKEN_ID: $(TOKEN_ID)"
	@echo "OWNER: $(OWNER)"
	@echo "BASE_URI: $(BASE_URI)"
	@echo "CONTRACT_ID: $(CONTRACT_ID)"
	@echo "=============================="

default: build

all: test

test: build
	cargo test

build:
	stellar contract build

fmt:
	cargo fmt --all

clean:
	cargo clean

upload:
	stellar contract upload \
		--network $(NETWORK) \
		--source-account $(SOURCE) \
		--wasm $(CONTRACT_WASM_PATH)

deploy:
	@CONTRACT_ID=$$(stellar contract deploy \
		--network $(NETWORK) \
		--source-account $(SOURCE) \
		--wasm $(CONTRACT_WASM_PATH) \
		-- \
		--owner $(DIONE_ADDRESS) \
		--max-supply $(STELLAR_NFT_CONTRACT_MAX_SUPPLY) \
		--metadata '{"name":"$(STELLAR_NFT_CONTRACT_NAME)","symbol":"$(STELLAR_NFT_CONTRACT_SYMBOL)","base_uri":"$(STELLAR_NFT_CONTRACT_BASE_URI)"}' | tr -d '\n'); \
	echo "Deployed contract ID: $$CONTRACT_ID";

set-metadata-uri:
	stellar contract invoke \
		--id $(CONTRACT_ID) \
		--source-account $(SOURCE) \
		--network $(NETWORK) \
		-- \
		set-metadata-uri \
		--base-uri $(BASE_URI)