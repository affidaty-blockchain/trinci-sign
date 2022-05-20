# Trinci Sign Lib

### OBSOLETE: TO CHECK

Binary utility executable to build private key signed transactions, compatible with TRINCI2 technology.

This utility is written in rust, but it can be used and compiled on which is an operating system to allow the use of signatures in the web server, downloaded from the language and the technology stack used for the backend

```bash
Trinci Blockchain Transaction Sign 0.1.0

USAGE:
    trinci-sign [OPTIONS] --command <COMMAND>

OPTIONS:
    -b, --bs58 <BASE58>              Arguments in messagepacked base58
    -c, --command <COMMAND>          Specify the command: { create_unit_tx | submit_unit_tx |
                                     to_message_pack }
    -h, --hex <HEX>                  Arguments in messagepacked HEX
        --help                       Print help information
    -j, --json <JSON>                Arguments in json String
        --jsonstruct <jsonstruct>    Json structure to convert in MessagePack
        --string <STRING>            String to convert in MessagePack
    -u, --url <URL>                  Trinci Node url
    -V, --version                    Print version information
```

The output is a bytes array with the transaction to send to the TRINCI blockchain, eg with `curl`:
```bash
$ cargo run -- --command create_unit_tx --bs58 <BS58DATA> | \ 
    curl -X POST --header "Content-Type:application/octet-stream" \ 
    --data-binary @- http://localhost:8000/api/v1/message
```

### `create_unit_tx`

`$ cargo run -- --command create_unit_tx --hex <HEX>`
`$ cargo run -- --command create_unit_tx --bs58 <BASE58>`
`$ cargo run -- --command create_unit_tx --json '<JSON>'`

 - `<HEX>` must be the message pack of the structure below.
 - `<BASE58>` must be the message pack of the structure below.
 - `<JSON>` must be the structure below passed as String. 

```json
args: 
{
    "target": String,       // Target account
    "network": String,      // Blockchain Network (it is in Multihash format)
    "fuel": integer,        // Max fuel allowed
    "contract": String,     // Multihash of the contract, empty String if not specified
    "method": String,       // Method to call
    "args": json,           // key/value json
    "private_key":String,   // base58 of the private key bytes array in pkcs8
}
```

Example:
```json
{
    "target":"#MYACCOUNT",
    "network":"QmNiibPaxdU61jSUK35dRwVQYjF9AC3GScWTRzRdFtZ4vZ",
    "fuel":1000,
    "contract":"12205bdca17463a5fbb92d461b61ec5b502ab2645c3487c94862f9b18c37bc01c118",
    "method":"transfer",
    "args":{"from":"QmamzDVuZqkUDwHikjHCkgJXXXXXXXVDTvTYb2aq6qfLbY","to":"#ANYACCOUNT","units":100},
    "private_key":"Invalid3wNt6sUs4jDqgN72ZfX7XWV88GH8txjkGw4jZUhUaYrZCfTzHNPfxLSX3Qzhu5kMd9KrngyMg3ikrKUKMdTxXQ9MXqgj376at1XmgECygypDwiQf",
}
```

### `submit_unit_tx`

`$ cargo run -- --command submit_unit_tx --json '<JSON>' --url <URL>`
`$ cargo run -- --command submit_unit_tx --hex <HEX> --url <URL>`
`$ cargo run -- --command submit_unit_tx --bs58 <BASE58> --url <URL>` 

 - The `<HEX>`, `<BASE58>` `<JSON>` arguments are the same of the `create_unit_tx` functionality.
 - the `<URL>` argument is the url (comprehensive of port and path) of the Trinci Node, eg: `http://localhost:8000/api/v1`

 - In case of success returns the HEX of the transaction receipt, eg:
   ```bash
   OK|12208496dac2cd6cbb56378d12fef825c5d3a1235ebdf72de33153d6d157d8b383ba
   ```
 - In case of error print the node answer, eg:
   ```bash
   KO|DuplicatedConfirmedTx
   KO|Error reading args!
   KO|Error sending unit tx message Error { kind: MalformedData, source: Some(KeyRejected("InvalidComponent")) }
   ...
   ```

### MessagePack Conversion Utility: `to_message_pack`
#### `String`
`$ cargo run -- --command to_message_pack --string <STRING>`

Example
`$ cargo run -- --command to_message_pack --string "Hello, Trinci!"`

Result:
`[174,72,101,108,108,111,44,32,84,114,105,110,99,105,33]`


#### `Json Structures`
`$ cargo run -- --command to_message_pack --jsonstruct '<JSONSTRUCT>'`

Example
`$ cargo run -- --command to_message_pack --jsonstruct '{"a":1,"b":"text","c":[1,2,3]}'`

Result:
`[131,161,97,1,161,98,164,116,101,120,116,161,99,147,1,2,3]`
## Compilation

### Linux
```bash
cargo build --release
```

### Windows from Linux
```bash
cargo build --release --target x86_64-pc-windows-gnu
```
Note: you need to install the cross-compilation toolchain:

 - For Debian:
   ```bash
   sudo apt-get install gcc-mingw-w64-x86-64 g++-mingw-w64-x86-64
   ```
