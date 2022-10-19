use client as cl;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <path to solana program keypair>",
            args[0]
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];

    let connection = cl::client::establish_connection().unwrap();
    println!(
        "Connected to remote solana node running version ({}).",
        connection.get_version().unwrap()
    );

    let user = cl::utils::get_user().unwrap();

    let program = cl::client::get_program(keypair_path, &connection).unwrap();

    cl::client::say_hello(&user, &program, &connection).unwrap();

}