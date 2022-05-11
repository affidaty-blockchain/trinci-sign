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

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub struct AppCommand {
    pub operation: AppOperation,
    pub args: Arguments,
    pub url: String,
}

pub enum Arguments {
    UnitTxArgsType(UnitTxArgs),
    MsgPackString(String),
    MsgPackStruct(Value),
}

pub enum AppOperation {
    CreateUnitTx,
    SubmitUnitTx,
    ToMessagePack,
}

impl FromStr for AppOperation {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "create_unit_tx" => Ok(AppOperation::CreateUnitTx),
            "submit_unit_tx" => Ok(AppOperation::SubmitUnitTx),
            "to_message_pack" => Ok(AppOperation::ToMessagePack),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UnitTxArgs {
    pub target: String,
    pub network: String,
    pub nonce: String, // base58 of a bytes array
    pub fuel: u64,
    pub contract: String,
    pub method: String,
    pub args: serde_value::Value,
    pub private_key: String, // base58 of a bytes array
}

impl UnitTxArgs {
    pub fn from_json_string(json_text: &str) -> Option<Self> {
        match serde_json::from_str::<UnitTxArgs>(json_text) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    pub fn from_hex_string(hex_text: &str) -> Option<Self> {
        match hex::decode(hex_text) {
            Ok(buf) => match rmp_serde::from_slice::<UnitTxArgs>(&buf) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }

    pub fn from_bs58_string(bs58_text: &str) -> Option<Self> {
        match bs58::decode(bs58_text).into_vec() {
            Ok(buf) => match rmp_serde::from_slice::<UnitTxArgs>(&buf) {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const ARGS_HEX: &str = "DF00000008A6746172676574A8234143434F554E54A76E6574776F726BA6534B594E4554A56E6F6E6365AB343363394A475973715971A46675656CCD2710A8636F6E7472616374D9443132323035616330636666313839653232373934623834373638373537386566343731346337646131306665396536663665333133363439323836333631623038323766A66D6574686F64AE6D795F636F6F6C5F6D6574686F64A461726773B1617267735F666F725F636F6E7472616374AB707269766174655F6B6579D9AF696E76616C696467744A4B68346533637742446D675348414E5862376872786D523456654A556B774C627A6B41745A626D6D635065534C426D33476B7272524E7235587A7233766A597335737845795571704546376232636B67436A43507045703577564C41746375555A4B69576B385A33374C3342776975584B57364A5759737650434A4148665970474A376D58725169505062324767506E3970774D4654533538317459796138356374357738";
    const ARGS_JSON: &str = "{\"target\":\"#ACCOUNT\",\"network\":\"SKYNET\",\"nonce\":\"43c9JGYsqYq\",\"fuel\":10000,\"contract\":\"12205ac0cff189e22794b847687578ef4714c7da10fe9e6f6e313649286361b0827f\",\"method\":\"my_cool_method\",\"args\":\"args_for_contract\",\"private_key\":\"invalidgtJKh4e3cwBDmgSHANXb7hrxmR4VeJUkwLbzkAtZbmmcPeSLBm3GkrrRNr5Xzr3vjYs5sxEyUqpEF7b2ckgCjCPpEp5wVLAtcuUZKiWk8Z37L3BwiuXKW6JWYsvPCJAHfYpGJ7mXrQiPPb2GgPn9pwMFTS581tYya85ct5w8\"}";
    const ARGS_BS58: &str = "2RruV5gEwt2u4GHYWKgAuaSuzHvp2XzmxmP2Cakys4sfcf6i1LHuYsuVthfS3CspELEJZA1moQDsmdkgv9kNcc5NudoKz97C2jNRgXT4supyHGCsTtNL4xKxQcgZcnHsEdKCtbSRagBvbwr6uiRfhoQ29c1Vn3auujet25qKfYqubeMFAb2YC6QR9bJiqA4Bz1wVFRr2BvBSqAP34B4MUYkdVXPbmokEe9PTa99d63n85mR9QHqcKnv4Bs7qHPrf9YrpHZgGguF49q9zfBgkGbw7YnVr9gAAM2v5nznWAb52cM6uyNcMWSjStHFXp6VSyedtWvaAr76hMsMbbcE3Go3RykXZNHEHEGaaGgAMudU28ZWkHEEuTCeXA1AizG2JsG1wsbFdcBYKeockRrKCdrnxNdvUAUsejBBFAx1AeE9wNHRR2DJFeanxWnfHpFbwz9jRr7qhNP7zdqxScCxDeNbZwF2VXmUkDxpN3TViBDZv4xSQW9grmVhhk5CMymyD1";

    fn create_unit_tx_args() -> UnitTxArgs {
        UnitTxArgs {
            target: String::from("#ACCOUNT"),
            network: String::from("SKYNET"),
            nonce: String::from("43c9JGYsqYq"),
            fuel: 10000u64,
            contract: String::from("12205ac0cff189e22794b847687578ef4714c7da10fe9e6f6e313649286361b0827f"),
            method: String::from("my_cool_method"),
            args: serde_value::value!("args_for_contract"),
            private_key: String::from("invalidgtJKh4e3cwBDmgSHANXb7hrxmR4VeJUkwLbzkAtZbmmcPeSLBm3GkrrRNr5Xzr3vjYs5sxEyUqpEF7b2ckgCjCPpEp5wVLAtcuUZKiWk8Z37L3BwiuXKW6JWYsvPCJAHfYpGJ7mXrQiPPb2GgPn9pwMFTS581tYya85ct5w8"),
        }
    }

    #[test]
    fn unit_tx_args_from_hex() {
        let expected = create_unit_tx_args();

        let res = UnitTxArgs::from_hex_string(ARGS_HEX).unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    fn unit_tx_args_from_json() {
        let expected = create_unit_tx_args();

        let res = UnitTxArgs::from_json_string(ARGS_JSON).unwrap();

        assert_eq!(res, expected);
    }

    #[test]
    fn unit_tx_args_from_bs58() {
        let expected = create_unit_tx_args();

        let res = UnitTxArgs::from_bs58_string(ARGS_BS58).unwrap();

        assert_eq!(res, expected);
    }
}
