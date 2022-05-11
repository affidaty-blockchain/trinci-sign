// This file is part of TRINCI.
//
// Copyright (C) 2021 Affidaty Spa.
//
// TRINCI is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// TRINCI is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License
// for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with TRINCI. If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write};

use common::{bs58_into_vec, get_args};
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
use types::{AppOperation, Arguments, Result};

mod common;
mod http_channel;
mod types;

fn submit_unit_tx(input_args: Arguments, url: String) -> Result<()> {
    let tx = create_unit_tx_as_vec(input_args)?;
    let mut http_channel = HttpChannel::new(url);
    http_channel.send(tx)?;
    let buf = http_channel.recv()?;

    let msg = rmp_deserialize::<Message>(&buf)?;

    let output = match msg {
        Message::PutTransactionResponse { hash } => {
            format!("OK|{}", hex::encode(hash.as_bytes()))
        }
        Message::Exception(e) => {
            format!("KO|{:?}", e.kind)
        }
        _ => {
            format!("KO|{:?}", msg)
        }
    };

    io::stdout()
        .write_all(output.as_bytes())
        .unwrap_or_default();

    Ok(())
}

fn create_unit_tx_as_vec(input_args: Arguments) -> Result<Vec<u8>> {
    match input_args {
        Arguments::UnitTxArgsType(input_args) => {
            let contract = if input_args.contract.is_empty() {
                None
            } else {
                Hash::from_hex(&input_args.contract).ok()
            };

            let private_bytes = bs58_into_vec(&input_args.private_key)?;

            let kp = EcdsaKeyPair::from_pkcs8_bytes(CurveId::Secp384R1, &private_bytes)?;

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
        Arguments::MsgPackString(_) => panic!("unexpected value"),
    }
}

fn convert_string_to_msgpack(input_args: Arguments) -> Result<()> {
    match input_args {
        Arguments::MsgPackString(args) => {
            let value = format!("{:?}", args).replace(' ', "");
            io::stdout().write_all(value.as_bytes()).unwrap_or_default()
        }
        _ => panic!("unexpected value"),
    }

    Ok(())
}

fn create_unit_tx(input_args: Arguments) -> Result<()> {
    let tx = create_unit_tx_as_vec(input_args)?;
    io::stdout().write_all(&tx).unwrap_or_default();
    Ok(())
}

fn main() {
    let args = get_args();
    match args {
        Some(cmd) => match cmd.operation {
            AppOperation::CreateUnitTx => {
                if let Err(e) = create_unit_tx(cmd.args) {
                    io::stdout()
                        .write_all(format!("KO|Error creating unit tx message {:?}", e).as_bytes())
                        .unwrap_or_default();
                }
            }
            AppOperation::SubmitUnitTx => {
                if let Err(e) = submit_unit_tx(cmd.args, cmd.url) {
                    io::stdout()
                        .write_all(format!("KO|Error sending unit tx message {:?}", e).as_bytes())
                        .unwrap_or_default();
                }
            }
            AppOperation::ToMessagePack => {
                if let Err(e) = convert_string_to_msgpack(cmd.args) {
                    io::stdout()
                        .write_all(
                            format!("KO|converting the string into msgpack {:?}", e).as_bytes(),
                        )
                        .unwrap_or_default();
                }
            }
        },
        None => {
            eprintln!("Error reading args!");
            io::stdout()
                .write_all("KO|Error reading args!".as_bytes())
                .unwrap_or_default();
        }
    };
}
