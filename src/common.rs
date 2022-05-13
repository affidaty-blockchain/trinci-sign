use std::str::FromStr;

use clap::{Arg, Command};
use serde_json::Value;

use crate::types::{self, AppCommand, AppOperation, Arguments, Result, UnitTxArgs};

/// Convert a base58 string into a vec
pub fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

fn create_app() -> Command<'static> {
    Command::new("Trinci Blockchain Transaction Sign")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("command")
                .long("command")
                .short('c')
                .help("Specify the command: { create_unit_tx | submit_unit_tx | to_message_pack }")
                .value_name("COMMAND")
                .required(true),
        )
        .arg(
            Arg::new("hex")
                .long("hex")
                .short('h')
                .help("Arguments in messagepacked HEX")
                .value_name("HEX")
                .required_unless_present_any(&["json", "bs58", "string", "jsonstruct"]),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .short('j')
                .help("Arguments in json String")
                .value_name("JSON")
                .required_unless_present_any(&["hex", "bs58", "string", "jsonstruct"]),
        )
        .arg(
            Arg::new("bs58")
                .long("bs58")
                .short('b')
                .help("Arguments in messagepacked base58")
                .value_name("BASE58")
                .required_unless_present_any(&["json", "hex", "string", "jsonstruct"]),
        )
        .arg(
            Arg::new("string")
                .long("string")
                .help("String to convert in MessagePack")
                .value_name("STRING")
                .required_unless_present_any(&["json", "hex", "bs58", "jsonstruct"]),
        )
        .arg(
            Arg::new("jsonstruct")
                .long("jsonstruct")
                .help("Json structure to convert in MessagePack")
                .value_name("jsonstruct")
                .required_unless_present_any(&["json", "hex", "bs58", "string"]),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .short('u')
                .help("Trinci Node url")
                .value_name("URL")
                .required_if_eq("command", "submit_unit_tx"),
        )
}

pub fn get_args() -> Option<AppCommand> {
    let matches = create_app().get_matches();

    let operation = matches.value_of("command").unwrap_or_default();
    let operation = AppOperation::from_str(operation);
    let operation = match operation {
        Ok(op) => op,
        Err(_) => return None,
    };

    let to_msgpack_string = matches.value_of("string").unwrap_or_default();
    let to_msgpack_struct = matches.value_of("jsonstruct").unwrap_or_default();

    let url = matches.value_of("url").unwrap_or_default();

    match operation {
        AppOperation::CreateUnitTx | AppOperation::SubmitUnitTx => {
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
                    operation,
                    args: Arguments::UnitTxArgsType(args),
                    url: url.to_string(),
                }),
                None => {
                    eprintln!("Invalid command");
                    None
                }
            }
        }
        AppOperation::ToMessagePack => {
            if !to_msgpack_string.is_empty() {
                Some(AppCommand {
                    operation,
                    args: types::Arguments::MsgPackString(to_msgpack_string.to_string()),
                    url: "".to_string(),
                })
            } else if !to_msgpack_struct.is_empty() {
                match Value::from_str(to_msgpack_struct) {
                    Ok(val) => Some(AppCommand {
                        operation,
                        args: types::Arguments::MsgPackStruct(val),
                        url: "".to_string(),
                    }),
                    Err(_) => {
                        eprintln!("Invalid json structure!");
                        None
                    }
                }
            } else {
                eprintln!("Invalid string!");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unit_tx_json_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "create_unit_tx",
            "--json",
            "any",
        ]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_create_unit_tx_bs58_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "create_unit_tx",
            "--bs58",
            "any",
        ]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_create_unit_tx_hex_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "create_unit_tx",
            "--hex",
            "any",
        ]);
        assert!(command.is_ok())
    }

    #[test]
    fn test_submit_unit_tx_json_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "submit_unit_tx",
            "--json",
            "any",
            "--url",
            "any_url",
        ]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_submit_unit_tx_bs58_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "submit_unit_tx",
            "--bs58",
            "any",
            "--url",
            "any_url",
        ]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_submit_unit_tx_hex_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "submit_unit_tx",
            "--hex",
            "any",
            "--url",
            "any_url",
        ]);
        assert!(command.is_ok())
    }

    #[test]
    fn test_to_message_pack_json_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "to_message_pack",
            "--jsonstruct",
            "any",
        ]);
        println!("command: {:?}", command);
        assert!(command.is_ok())
    }
    #[test]
    fn test_to_message_pack_string_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
            "--command",
            "to_message_pack",
            "--string",
            "any",
        ]);
        assert!(command.is_ok())
    }
}
