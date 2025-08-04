# Simple CLI for Solana PDA.

### This is a pet-project for explore possibilities of Solana PDA.

### Goals:

[x] - Create simple Solana Program for working with PDA.

[x] - Create cli interface for working with Solana Program.

[] - Create api interface for working with Solana Program.

[] - Add database.

[] - Add error-handling for Program and cli/api interfaces.

# CLI.

Before use cli:

Create your own keypair:

```bash
cd cli
```

```bash
solana-keygen new -o key/user_keypair.json
```

or change path to your existed solana keypair:

```rust
pub const KEYPAIR_FILE_PATH: &'static str = "*your path*";
```

## NOTE!

You must have some Sol on your account than you will use in your "key/user_keypair.json".

Commands:

Generate wallet:

```
cargo run generate "test"
```

Show wallets list:

```
cargo run list
```

Show wallet balance:

```
cargo run balance "{wallet_name}"
```

Send Sol from your wallet:

```
cargo run -- send --name "{wallet_name}" --recipient "{recipient_address}" --amount {amount}
```
