pythonSetup:
	sudo apt-get update && sudo apt-get install ffmpeg libsm6 libxext6  -y

setup:
	rustup target add wasm32-unknown-unknown
	cargo install --locked --version 20.0.0-rc2 soroban-cli --features opt
	echo "source <(soroban completion --shell bash)" >> ~/.bashrc

sorobanInit:
	soroban config network add testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"
	# soroban config identity generate alice

build:
	soroban contract build && soroban contract optimize --wasm target/wasm32-unknown-unknown/release/contracts.wasm

deploy:
	soroban contract deploy --wasm target/wasm32-unknown-unknown/release/hello_contracts.optimized.wasm --source alice --network testnet > ../.contractId