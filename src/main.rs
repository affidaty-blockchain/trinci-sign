use std::io::{self, Write};

use clap::{Arg, Command};
use http_channel::HttpChannel;
use trinci_core::{
    base::{
        schema::{SignedTransaction, TransactionData},
        serialize::{rmp_deserialize, rmp_serialize, MessagePack},
    },
    crypto::{
        ecdsa::{CurveId, KeyPair as EcdsaKeyPair},
        Hash,
    },
    KeyPair, Message, TransactionDataV1,
};
use types::{AppCommand, Arguments, Result, UnitTxArgs};

mod http_channel;
mod types;

fn get_args() -> Option<AppCommand> {
    let matches = Command::new("Trinci Blockchain Transaction Sign")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("command")
                .long("command")
                .short('c')
                .help("Specify the command: { create_unit_tx | submit_unit_tx }")
                .value_name("COMMAND")
                .required(true)
                .requires_if("submit_unit_tx", "url"),
        )
        .arg(
            Arg::new("hex")
                .long("hex")
                .short('h')
                .help("Arguments in messagepacked HEX")
                .value_name("HEX")
                .required_unless_present("json")
                .required_unless_present("bs58"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .short('j')
                .help("Arguments in json String")
                .value_name("JSON")
                .required_unless_present("hex")
                .required_unless_present("bs58"),
        )
        .arg(
            Arg::new("bs58")
                .long("bs58")
                .short('b')
                .help("Arguments in messagepacked base58")
                .value_name("BASE58")
                .required_unless_present("json")
                .required_unless_present("hex"),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .short('u')
                .help("Trinci Node url")
                .value_name("URL"),
        )
        .get_matches();

    let operation = matches.value_of("command").unwrap_or_default();
    let url = matches.value_of("url").unwrap_or_default();

    match operation {
        "create_unit_tx" | "submit_unit_tx" => {
            let inner_args = if let Some(hex_text) = matches.value_of("hex") {
                UnitTxArgs::from_hex_string(hex_text)
            } else if let Some(json_text) = matches.value_of("json") {
                UnitTxArgs::from_json_string(json_text)
            } else if let Some(bs58_text) = matches.value_of("bs58") {
                UnitTxArgs::from_bs58_string(bs58_text)
            } else {
                eprintln!("Args error");
                None
            };
            match inner_args {
                Some(args) => Some(AppCommand {
                    operation: operation.to_string(),

                    args: Arguments::UnitTxArgsType(args),

                    url: url.to_string(),
                }),
                None => {
                    eprintln!("Invalid command");
                    None
                }
            }
        }
        _ => {
            eprintln!("Invalid operation: {}", operation);
            None
        }
    }
}

/// Convert a base58 string into a vec
fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

fn submit_unit_tx(input_args: Arguments, url: String) -> Result<Vec<u8>> {
    let tx = create_unit_tx(input_args)?;
    let mut http_channel = HttpChannel::new(url);
    http_channel.send(tx)?;
    let buf = http_channel.recv()?;

    let msg = rmp_deserialize::<Message>(&buf)?;

    let result = match msg {
        Message::PutTransactionResponse { hash } => {
            hex::encode(hash.as_bytes()).as_bytes().to_vec()
        }
        Message::Exception(e) => {
            eprintln!("Node Answer: {:?}", e.kind);
            vec![]
        }
        _ => {
            eprintln!("Node Error: {:?}", msg);
            vec![]
        }
    };

    Ok(result)
}

fn create_unit_tx(input_args: Arguments) -> Result<Vec<u8>> {
    match input_args {
        Arguments::UnitTxArgsType(input_args) => {
            let contract = if input_args.contract.is_empty() {
                None
            } else {
                Hash::from_hex(&input_args.contract).ok()
            };

            let private_bytes = bs58_into_vec(&input_args.private_key)?;
            let public_bytes = bs58_into_vec(&input_args.public_key)?;

            let kp = EcdsaKeyPair::new(CurveId::Secp384R1, &private_bytes, &public_bytes)?;

            let args = rmp_serialize(&input_args.args)?;

            let nonce = bs58_into_vec(&input_args.nonce)?;

            let data = TransactionDataV1 {
                account: input_args.target,
                fuel_limit: input_args.fuel,
                nonce,
                network: input_args.network,
                contract,
                method: input_args.method,
                caller: trinci_core::PublicKey::Ecdsa(kp.public_key()),
                args,
            };

            let data = TransactionData::V1(data);
            let bytes = data.serialize();
            let signature = KeyPair::Ecdsa(kp).sign(&bytes)?;

            let sign_tx = SignedTransaction { data, signature };

            let tx = trinci_core::Transaction::UnitTransaction(sign_tx);

            let message = Message::PutTransactionRequest { confirm: true, tx };

            // Message pack of the transaction
            let buf = rmp_serialize(&message)?;

            Ok(buf)
        }
    }
}

fn main() {
    let args = get_args();
    let result = match args {
        Some(cmd) => match cmd.operation.as_str() {
            "create_unit_tx" => create_unit_tx(cmd.args).expect("Error creating unit tx message"),
            "submit_unit_tx" => submit_unit_tx(cmd.args, cmd.url).expect("Error sending unit tx"),
            _ => {
                eprintln!("Invalid command");
                vec![]
            }
        },
        None => {
            eprintln!("Error reading args!");
            vec![]
        }
    };
    io::stdout().write_all(&result).unwrap();
}
