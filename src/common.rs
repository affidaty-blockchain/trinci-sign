use std::str::FromStr;

use clap::{Arg, Command};

use crate::types::{AppCommand, AppOperation, Result, TxArguments, UnitTxArgs};

/// Convert a base58 string into a vec
pub fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

pub fn get_args() -> Option<AppCommand> {
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
                    operation: match AppOperation::from_str(operation) {
                        Ok(val) => val,
                        Err(_) => return None,
                    },

                    args: TxArguments::UnitTxArgsType(args),

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
