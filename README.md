Full Deploy Cycle Example
Take a program and deploy it to Atlas. You can try out the deploy cycle by running the following commands:
```
# 1. Airdrop some testnet tokens to your filesystem keypair
solana airdrop 0.01 -u "https://testnet.atlas.xyz"

# 2. Clone the SPL Token Program from Solana mainnet
solana program dump -um TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA program.so

# 3. Generate a new program ID for Atlas
solana-keygen new -f -o program-keypair.json -s --no-bip39-passphrase

# 4. Deploy the cloned program to Atlas testnet at the generated address
solana program deploy -u "https://testnet.atlas.xyz" program.so --program-id program-keypair.json
```
