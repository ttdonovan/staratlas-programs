# NOTES

```
$ cargo build
$ cargo doc --workspace --no-deps
$ cargo test
```

## Solana CLI

```
source ~/.profile
solana --version
solana-cli 2.1.5 (src:4da190bd; feat:288566304, client:Agave)

solana config get
...

mkdir tmp
solana-keygen new -o ./tmp/id.json
solana config set --keypair ./tmp/id.json

# solana config set --url mainnet-beta
solana config set --url localhost

solana config get

solana-test-validator --limit-ledger-size 1000
solana airdrop 2

solana address
solana balance

# https://explorer.solana.com
```

## Star Atlas Programs

```
$ solana program dump Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk programs/cargo/Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk.so
$ solana program dump CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ programs/crew/CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ.so
$ solana program dump pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9 programs/player-profile/pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9.so
$ solana program dump pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq programs/profile-faction/pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq.so
$ solana program dump SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE programs/sage/SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE.so
```

### Claude

* https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview

## Research

* https://github.com/codama-idl/codama
* https://github.com/solana-program/libraries
