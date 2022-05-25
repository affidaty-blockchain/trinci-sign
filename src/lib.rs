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

use http_channel::HttpChannel;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
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
use types::{Result, UnitTxArgs};

mod http_channel;
mod types;

static mut RESULT_STRING_PTR: *mut c_char = 0 as *mut c_char;

/// Convert a base58 string into a vec
pub fn bs58_into_vec(bs58_text: &str) -> Result<Vec<u8>> {
    bs58::decode(bs58_text).into_vec().map_err(|e| e.into())
}

macro_rules! unwrap_or_return {
    ( $e:expr , $val:expr) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $val,
        }
    };
}

fn store_string_on_heap(string_to_store: String) -> *mut c_char {
    // fn store_string_on_heap(string_to_store: &'static str) -> *mut c_char {
    //create a new raw pointer
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    //store it in our static variable (REQUIRES UNSAFE)
    unsafe {
        RESULT_STRING_PTR = pntr;
    }
    //return the c_char
    return pntr;
}

#[no_mangle]
pub extern "C" fn free_string() {
    unsafe {
        let _ = CString::from_raw(RESULT_STRING_PTR);
        RESULT_STRING_PTR = 0 as *mut c_char;
    }
}

fn create_unit_tx_as_vec(input_args: UnitTxArgs) -> Result<Vec<u8>> {
    let contract = if input_args.contract.is_empty() {
        None
    } else {
        Hash::from_hex(&input_args.contract).ok()
    };

    let private_bytes = bs58_into_vec(&input_args.private_key)?;

    let kp = EcdsaKeyPair::from_pkcs8_bytes(CurveId::Secp384R1, &private_bytes)?;

    let args = rmp_serialize(&input_args.args)?;

    let nonce = rand::random::<u64>().to_be_bytes().to_vec();

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

#[no_mangle]
pub extern "C" fn convert_json_to_msgpack(input_args: *mut c_char) -> *mut c_char {
    println!("args: {:?}", input_args);

    let c_str: &CStr = unsafe { CStr::from_ptr(input_args) };
    let str_slice: &str = c_str.to_str().unwrap();
    let input_args: String = str_slice.to_owned(); // if necessary

    match serde_json::from_str::<Value>(&input_args) {
        Ok(val) => match rmp_serialize(&val) {
            Ok(buf) => {
                let res = format!("{:?}", buf).replace(' ', "");
                store_string_on_heap(res)
            }
            Err(_) => store_string_on_heap("KO|serialization error".to_string()),
        },
        Err(_) => store_string_on_heap("KO|bad input".to_string()),
    }
}

#[no_mangle]
fn submit_unit_tx(input_args: String, url: String) -> String {
    // FIXME add pub extern "C"
    if let Some(input_args) = UnitTxArgs::from_json_string(&input_args) {
        let tx = unwrap_or_return!(
            create_unit_tx_as_vec(input_args),
            String::from("KO|error creating unit tx")
        );
        let mut http_channel = HttpChannel::new(url);
        unwrap_or_return!(
            http_channel.send(tx),
            String::from("KO|error sending unit tx")
        );
        let buf = unwrap_or_return!(http_channel.recv(), String::from("KO|error on recv"));

        let output = if String::from_utf8_lossy(&buf) == *"true" {
            String::from("OK|Valid Transaction!")
        } else if String::from_utf8_lossy(&buf) == *"false" {
            String::from("KO|Invalid Transaction!")
        } else {
            let msg = unwrap_or_return!(
                rmp_deserialize::<Message>(&buf),
                String::from("KO|error on message deserialization")
            );

            match msg {
                Message::PutTransactionResponse { hash } => {
                    format!("OK|{}", hex::encode(hash.as_bytes()))
                }
                Message::Exception(e) => {
                    format!("KO|{:?}", e.kind)
                }
                _ => {
                    format!("KO|{:?}", msg)
                }
            }
        };
        format!("OK|{}", output)
    } else {
        "KO|Bad input args".to_string()
    }
}

#[cfg(test)]
mod tests {
    // TODO
    #[test]
    fn test_convert_json_to_msgpack() {}

    // TODO
    #[test]
    fn create_unit_tx_as_vec() {}
}
