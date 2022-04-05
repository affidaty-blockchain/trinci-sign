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
use types::{AppOperation, Result, TxArguments};

mod common;
mod http_channel;
mod types;

fn submit_unit_tx(input_args: TxArguments, url: String) -> Result<Vec<u8>> {
    let tx = create_unit_tx(input_args)?;
    let mut http_channel = HttpChannel::new(url);
    http_channel.send(tx)?;
    let buf = http_channel.recv()?;

    let msg = rmp_deserialize::<Message>(&buf)?;

    let result = match msg {
        Message::PutTransactionResponse { hash } => hash.as_bytes().to_vec(),
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

fn create_unit_tx(input_args: TxArguments) -> Result<Vec<u8>> {
    match input_args {
        TxArguments::UnitTxArgsType(input_args) => {
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
        Some(cmd) => match cmd.operation {
            AppOperation::CreateUnitTx => {
                create_unit_tx(cmd.args).expect("Error creating unit tx message")
            }
            AppOperation::SubmitUnitTx => {
                submit_unit_tx(cmd.args, cmd.url).expect("Error sending unit tx")
            }
        },
        None => {
            eprintln!("Error reading args!");
            vec![]
        }
    };
    io::stdout().write_all(&result).unwrap();
}
