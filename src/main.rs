/*

To run:
cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10

To decrypt:
cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx drowcovfoc"  --decrypt --shift 10

*/

mod libs;

//use caeser_cipher_cli::{decrypt, encrypt};
use clap::Parser;
use crate::libs::{decrypt, encrypt};

// Cli tool to encrypt and decrypt messages using the caesar cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// Decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to user for the cipher
    /// Must be between 1 and 25, the default value is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

fn main() {
    let args = Args::parse();
    if args.encrypt{
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else {
        println!("Please specify either --encrypt or --decrypt");
    }

}
