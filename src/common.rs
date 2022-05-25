use std::str::FromStr;

use clap::{Arg, ArgMatches, Command};
use serde_json::Value;

use crate::types::{AppCommand, AppOperation, Arguments, Result, UnitTxArgs};

/// Convert a base58 string into a vec
pub fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

fn create_app() -> Command<'static> {
    let hex_arg = Arg::new("hex")
        .long("hex")
        .help("Arguments in messagepacked HEX")
        .value_name("HEX");

    let json_arg = Arg::new("json")
        .long("json")
        .help("Arguments in json String")
        .value_name("JSON");

    let bs58_arg = Arg::new("bs58")
        .long("bs58")
        .help("Arguments in messagepacked base58")
        .value_name("BASE58");

    Command::new("Trinci Blockchain Transaction Sign")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("create_unit_tx")
                .about("Create a binary Trinci unit tx")
                .arg(
                    hex_arg
                        .clone()
                        .required_unless_present_any(&["json", "bs58"])
                        .conflicts_with_all(&["json", "bs58"]),
                )
                .arg(
                    bs58_arg
                        .clone()
                        .required_unless_present_any(&["json", "hex"])
                        .conflicts_with_all(&["json", "hex"]),
                )
                .arg(
                    json_arg
                        .clone()
                        .required_unless_present_any(&["hex", "bs58"])
                        .conflicts_with_all(&["hex", "bs58"]),
                ),
        )
        .subcommand(
            Command::new("submit_unit_tx")
                .about("Submit to the Trinci Blockchain a unit tx")
                .arg(
                    hex_arg
                        .clone()
                        .required_unless_present_any(&["json", "bs58"]),
                )
                .arg(
                    bs58_arg
                        .clone()
                        .required_unless_present_any(&["json", "hex"]),
                )
                .arg(
                    json_arg
                        .clone()
                        .required_unless_present_any(&["hex", "bs58"]),
                )
                .arg(
                    Arg::new("url")
                        .long("url")
                        .short('u')
                        .help("Trinci Node url")
                        .value_name("URL")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("to_message_pack")
                .about("Convert a string or a json into a byte array (returned as string)")
                .arg(json_arg.clone().required_unless_present_any(&["string"]))
                .arg(
                    Arg::new("string")
                        .long("string")
                        .help("String to convert in MessagePack")
                        .value_name("STRING")
                        .required_unless_present_any(&["json"]),
                ),
        )
}

fn get_inner_args(matches: &ArgMatches) -> Option<UnitTxArgs> {
    if let Some(hex_text) = matches.value_of("hex") {
        UnitTxArgs::from_hex_string(hex_text)
    } else if let Some(json_text) = matches.value_of("json") {
        UnitTxArgs::from_json_string(json_text)
    } else if let Some(bs58_text) = matches.value_of("bs58") {
        UnitTxArgs::from_bs58_string(bs58_text)
    } else {
        eprintln!("Args error");
        None
    }
}

pub fn get_args() -> Option<AppCommand> {
    let matches = create_app().get_matches();

    match matches.subcommand() {
        Some(("create_unit_tx", sub_matches)) => match get_inner_args(sub_matches) {
            Some(args) => Some(AppCommand {
                operation: AppOperation::CreateUnitTx,
                args: Arguments::UnitTxArgsType(args),
                url: String::new(),
            }),
            None => {
                eprintln!("Invalid command");
                None
            }
        },
        Some(("submit_unit_tx", sub_matches)) => {
            let url = match sub_matches.value_of("url") {
                Some(val) => val.to_string(),
                None => return None,
            };
            match get_inner_args(sub_matches) {
                Some(args) => Some(AppCommand {
                    operation: AppOperation::SubmitUnitTx,
                    args: Arguments::UnitTxArgsType(args),
                    url,
                }),
                None => {
                    eprintln!("Invalid command");
                    None
                }
            }
        }
        Some(("to_message_pack", sub_matches)) => {
            let msg_pack_args = if let Some(json_text) = sub_matches.value_of("json") {
                match Value::from_str(json_text) {
                    Ok(val) => Arguments::MsgPackStruct(val),
                    Err(_) => return None,
                }
            } else if let Some(string_text) = sub_matches.value_of("string") {
                Arguments::MsgPackString(string_text.to_string())
            } else {
                return None;
            };
            Some(AppCommand {
                operation: AppOperation::ToMessagePack,
                args: msg_pack_args,
                url: String::new(),
            })
        }

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unit_tx_json_command() {
        let command =
            create_app().try_get_matches_from(vec!["prog", "create_unit_tx", "--json", "any"]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_create_unit_tx_bs58_command() {
        let command =
            create_app().try_get_matches_from(vec!["prog", "create_unit_tx", "--bs58", "any"]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_create_unit_tx_hex_command() {
        let command =
            create_app().try_get_matches_from(vec!["prog", "create_unit_tx", "--hex", "any"]);
        assert!(command.is_ok())
    }

    #[test]
    fn test_submit_unit_tx_json_command() {
        let command = create_app().try_get_matches_from(vec![
            "prog",
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
        let command =
            create_app().try_get_matches_from(vec!["prog", "to_message_pack", "--json", "any"]);
        assert!(command.is_ok())
    }
    #[test]
    fn test_to_message_pack_string_command() {
        let command =
            create_app().try_get_matches_from(vec!["prog", "to_message_pack", "--string", "any"]);
        assert!(command.is_ok())
    }
}
