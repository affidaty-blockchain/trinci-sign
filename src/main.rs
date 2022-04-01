use clap::{Arg, Command};
use trinci_core::{
    base::{
        schema::{SignedTransaction, TransactionData},
        serialize::{rmp_serialize, MessagePack},
    },
    crypto::{
        ecdsa::{CurveId, KeyPair as EcdsaKeyPair},
        Hash,
    },
    KeyPair, Message, TransactionDataV1,
};
use types::{AppCommand, Arguments, Result, UnitTxArgs};

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
                .help("Specify the command: { create_unit_tx }")
                .value_name("COMMAND")
                .required(true),
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
        .get_matches();

    let operation = matches.value_of("command").unwrap_or_default();

    let args = match operation {
        "create_unit_tx" => {
            let inner_args = if let Some(hex_text) = matches.value_of("hex") {
                UnitTxArgs::from_hex_string(hex_text)
            } else if let Some(json_text) = matches.value_of("json") {
                UnitTxArgs::from_json_string(json_text)
            } else if let Some(bs58_text) = matches.value_of("bs58") {
                UnitTxArgs::from_bs58_string(bs58_text)
            } else {
                None
            };
            match inner_args {
                Some(args) => Arguments::UnitTxArgsType(args),
                None => return None,
            }
        }
        _ => return None,
    };

    Some(AppCommand {
        operation: operation.to_string(),
        args,
    })
}

/// Convert a base58 string into a vec
fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

fn create_unit_tx(input_args: Arguments) -> Result<()> {
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

            // Message pack of the transaction and save as .bin
            let buf = rmp_serialize(&message)?;

            std::fs::write("transaction.bin", buf).unwrap();

            println!("Trinci Transaction saved into `transaction.bin`");
            Ok(())
        }
    }
}

fn main() {
    let args = get_args();
    match args {
        Some(val) => match val.operation.as_str() {
            "create_unit_tx" => {
                create_unit_tx(val.args).expect("Error creating unit tx message");
            }
            _ => {
                println!("Command error!")
            }
        },
        None => println!("Arguments error!"),
    }
}
