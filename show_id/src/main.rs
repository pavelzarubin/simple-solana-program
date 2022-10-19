use solana_sdk::signer::keypair::{read_keypair_file};
use solana_sdk::signer::Signer;
use solana_sdk::pubkey::Pubkey;



fn read_config(keypair_path:&str) -> Result<Pubkey,String>{
    let program_keypair = read_keypair_file(keypair_path);
    match program_keypair {
        Ok(result) => return Ok(result.pubkey()),
        _ => Err(String::from("Something wrong"))

    }

        }

fn main() {
    
    
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <path to solana hello world example program keypair>",
            args[0]
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];
    match read_config(keypair_path) {
        Ok(s) => println!("{}",s),
        _ => eprintln!("eroror")
    }

}
