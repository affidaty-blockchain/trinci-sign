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
use serde_value::Value;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub struct AppCommand {
    pub operation: AppOperation,
    pub args: TxArguments,
    pub url: String,
}

pub enum TxArguments {
    UnitTxArgsType(UnitTxArgs),
}

pub enum AppOperation {
    CreateUnitTx,
    SubmitUnitTx,
}

impl FromStr for AppOperation {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "create_unit_tx" => Ok(AppOperation::CreateUnitTx),
            "submit_unit_tx" => Ok(AppOperation::SubmitUnitTx),
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
    pub args: Value,
    pub public_key: String,  // base58 of a bytes array
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

    const ARGS_HEX: &str = "DF00000009A6746172676574A8234143434F554E54A76E6574776F726BA6534B594E4554A56E6F6E6365AB343363394A475973715971A46675656CCD2710A8636F6E7472616374D9443132323035616330636666313839653232373934623834373638373537386566343731346337646131306665396536663665333133363439323836333631623038323766A66D6574686F64AE6D795F636F6F6C5F6D6574686F64A461726773B57B2761273A3132332C2762273A2768656C6C6F277DAA7075626C69635F6B6579D92C32376D3556674A59324A5062775548356E50627139644E705251597539506459327146514150343279727A35AB707269766174655F6B6579D957754C376B3473704B413276686658314E6464373470555739554E7A44476D4362584868725A6B354C734C414B6366566F6B356762723667596F53374671436766525738666B483559664A34645941436B72743848543668";
    const ARGS_JSON: &str = "{\"target\":\"#ACCOUNT\",\"network\":\"SKYNET\",\"nonce\":\"43c9JGYsqYq\",\"fuel\":10000,\"contract\":\"12205ac0cff189e22794b847687578ef4714c7da10fe9e6f6e313649286361b0827f\",\"method\":\"my_cool_method\",\"args\":\"{'a':123,'b':'hello'}\",\"public_key\":\"27m5VgJY2JPbwUH5nPbq9dNpRQYu9PdY2qFQAP42yrz5\",\"private_key\":\"uL7k4spKA2vhfX1Ndd74pUW9UNzDGmCbXHhrZk5LsLAKcfVok5gbr6gYoS7FqCgfRW8fkH5YfJ4dYACkrt8HT6h\"}";
    const ARGS_BS58: &str = "3PuEMd5N4u3L6WzutXHrpYEtXFPu2gmoLKQudpt2jzrHhb9o2K5FNp5qDVK1mV35bKUhbbR4Uy12FtxjHGJLEe4uwTnuGGmyTtXuBs5TWXGgrvC87h286CNpz5FWrzVjZVPMJNb71aFhWRqT4AxBG9yiX6Lvz7WvwR6RunhzieAtMpLdGgvWuMWCoaCrUDCN28QLPLFKgPkcB6D24qDMxM1MPz5bFzAP3U5N2hoSmEduaBh8dFrFUZpARpZ4txF9it5khmwrkPEirwPDw4kpvF5GhqdM2Rxn3NfRnomzDGocxHftgf4Z77SSe3xb8APmtDWw5vWT7orKKTVoCs4TjYMQRAwz4Hh7L9TVdGoVyhG1UifCKZ4TT95Wuf6YaMon8SmyS6ULFyDbX1YCrRefgTLVGkAsReUeGhST1Atv9ntn64swnHpVeNgytycW5nJ4kCsf5kNaLbW5qYsdZ7trQUPZd5Q3";

    fn create_unit_tx_args() -> UnitTxArgs {
        UnitTxArgs {
            target: String::from("#ACCOUNT"),
            network: String::from("SKYNET"),
            nonce: String::from("43c9JGYsqYq"),
            fuel: 10000u64,
            contract: String::from("12205ac0cff189e22794b847687578ef4714c7da10fe9e6f6e313649286361b0827f"),
            method: String::from("my_cool_method"),
            args: serde_value::value!("{'a':123,'b':'hello'}"),
            public_key: String::from("27m5VgJY2JPbwUH5nPbq9dNpRQYu9PdY2qFQAP42yrz5"),
            private_key: String::from("uL7k4spKA2vhfX1Ndd74pUW9UNzDGmCbXHhrZk5LsLAKcfVok5gbr6gYoS7FqCgfRW8fkH5YfJ4dYACkrt8HT6h"),
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
