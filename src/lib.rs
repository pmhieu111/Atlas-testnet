use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, World! from atlas-testnet-hieuwb");

    // Here you would process the instruction data or interact with accounts
    // This is just a placeholder for demonstration
    Ok(())
}

// Test module (for unit tests)
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "atlas-testnet-hieuwb",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        // Create a test account
        let account = Keypair::new();
        let mut transaction = Transaction::new_with_payer(
            &[solana_sdk::system_instruction::create_account(
                &payer.pubkey(),
                &account.pubkey(),
                1,
                0,
                &program_id,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &account], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Process an instruction
        let instruction = solana_program::instruction::Instruction::new_with_borsh(
            program_id,
            &(),
            vec![AccountMeta::new(account.pubkey(), false)],
        );
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );
        banks_client.process_transaction(transaction).await.unwrap();

        // Here you would assert some condition to check if the program works as expected
    }
}
