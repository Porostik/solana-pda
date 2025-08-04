CLI_DIR = cli

.PHONY: all build deploy cli clean

program-build:
	anchor build

program-deploy:
	solana program deploy target/deploy/pda_wallet.so --program-id key/pda_wallet-keypair.json

cli-build:
	cd $(CLI_DIR) && cargo build
	mkdir target/debug/key/ && cp cli/key/user_keypair.json target/debug/key/
