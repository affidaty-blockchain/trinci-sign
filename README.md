# Trinci Sign

Binary utility executable to build private key signed transactions, compatible with TRINCI2 technology.

This utility is written in rust, but it can be used and compiled on which is an operating system to allow the use of signatures in the web server, downloaded from the language and the technology stack used for the backend

```bash
Trinci Blockchain Transaction Sign 0.1.0

USAGE:
    trinci-sign [OPTIONS] --command <COMMAND>

OPTIONS:
    -b, --bs58 <BASE58>        Arguments in message packed base58
    -c, --command <COMMAND>    Specify the command: { create_unit_tx }
    -h, --hex <HEX>            Arguments in message packed HEX
        --help                 Print help information
    -j, --json <JSON>          Arguments in json String
    -V, --version              Print version information
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
`$ cargo run -- --command create_unit_tx --json <JSON>`

 - `<HEX>` must be the message pack of the structure below.
 - `<BASE58>` must be the message pack of the structure below.
 - `<JSON>` must be the structure below passed as String. 

```json
args: 
{
    "target": String,       // Target account
    "network": String,      // Blockchain Network (it is in Multihash format)
    "nonce": String,        // base58 of a bytes array
    "fuel": integer,        // Max fuel allowed
    "contract": String,     // Multihash of the contract, empty String if not specified
    "method": String,       // Method to call
    "args": json,           // key/value json
    "public_key": String,   // base58 of the public key bytes array
    "private_key":String,   // base58 of the private key bytes array
}
```

Example:
```json
{
    "target":"#MYACCOUNT",
    "network":"QmNiibPaxdU61jSUK35dRwVQYjF9AC3GScWTRzRdFtZ4vZ",
    "nonce":"VgsL75FnH3X",
    "fuel":1000,
    "contract":"12205bdca17463a5fbb92d461b61ec5b502ab2645c3487c94862f9b18c37bc01c118",
    "method":"transfer",
    "args":{"from":"QmamzDVuZqkUDwHikjHCkgJXXXXXXXVDTvTYb2aq6qfLbY","to":"#ANYACCOUNT","units":100},
    "public_key":"88GH8txjkGw4jZUhUaYrZCfTzHNPfxLSX3QzhgXXXXXXXXXXXXXXXXXXXXXXAH4nC61uGVA6SusX7AvVGNnZqNQwBZqzuZnDBcWsu5kMd9KrngyMg3ikrKUKMdTxXQ9MXqgj",
    "private_key":"3wNt6sUs4jDqgN72ZfX7XWVXXXXXXXXXXXXXXXXXXXXgdE376at1XmgECygypDwiQf",
}
```