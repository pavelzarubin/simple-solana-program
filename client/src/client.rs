use crate::utils;
use crate::{Error, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{Instruction};
use solana_sdk::message::Message;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use solana_sdk::transaction::Transaction;


pub fn establish_connection() -> Result<RpcClient> {
    let rpc_url = utils::get_rpc_url()?;
    Ok(RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    ))
}

pub fn get_program(keypair_path: &str, connection: &RpcClient) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).map_err(|e| {
        Error::InvalidConfig(format!(
            "failed to read program keypair file ({}): ({})",
            keypair_path, e
        ))
    })?;

    let program_info = connection.get_account(&program_keypair.pubkey())?;
    if !program_info.executable {
        return Err(Error::InvalidConfig(format!(
            "program with keypair ({}) is not executable",
            keypair_path
        )));
    }

    Ok(program_keypair)
}


pub fn say_hello(user: &Keypair, program: &Keypair, connection: &RpcClient) -> Result<()> {
    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[],
        vec![],
    );
    let message = Message::new(&[instruction], Some(&user.pubkey()));
    let transaction = Transaction::new(&[user], message, connection.get_latest_blockhash().unwrap());

    connection.send_and_confirm_transaction(&transaction)?;

    Ok(())
}