# Trinci Sign Dynamic-link Library
Trinci Dynamic-link Library to submit signed transactions to TRINCI BLOCKCHAIN TRINCI2 technology.

# Compile the dll (from windows)
```bash
cargo build --release
```
The compiled dll can be found in the directory `target\release\trinci_sign.dll` and needs to be copied on the `c#` project directory.

# **C#** usage example
```cs
using System.Runtime.InteropServices;

// Used to convert a json string into a message packed bytes array (as string: "[110,...,123]")
// The `text` argument must be a pointer to a valid json string: eg: "{\"name1\":\"value1\",\"name2\":123}"
[DllImport("trinci_sign.dll")]
static extern IntPtr convert_json_to_msgpack(IntPtr text);

// Submit a transaction to a trinci endpoint
// The `json` and `url` must be pointers to valid json and url strings
// json : 
//  {
//      "target": account-id, // eg: "#ACCOUNT"
//      "network": string,    // eg: "QmNiibPaxdU61jSUK35dRwVQYjF9AC3GScWTRzRdFtZ4vZ",
//      "fuel": integer,      // eg: 1000
//      "contract": string,   // eg: contract hash as base58 (could be empty)
//      "method": string,     // "transfer",
//      "args": json_string,  // contract_args, eg: {"arg1":123, "arg2":"account1"}
//      "private_key": string // private key used to sign (base58)
//  }
// Note: this must be a valid string, so the double quotes need to be escaped: " -> \"
[DllImport("trinci_sign.dll")]
static extern IntPtr submit_unit_tx(IntPtr json, IntPtr url); 


// This is used to free the memory used by the imported function to return a string
[DllImport("trinci_sign.dll")]
static extern void free_string();


bool submit(Int64 units, String purpose, String data, String url, String pvt_key)
{
    var payment_data = "[]";

    // Prepare data args
    if (data != "")
    {
        IntPtr intPtr_data = Marshal.StringToHGlobalAnsi(data);
        IntPtr data_result = convert_json_to_msgpack(intPtr_data);
        var payment_data_tmp = Marshal.PtrToStringAnsi(data_result);
        free_string();
        if (payment_data_tmp != null && payment_data_tmp[0] == '[')
        {
            payment_data = payment_data_tmp;
        }
        else
        {
            return false;
        }
    }

    String contract_args = "{\"asset\":\"#EURS\",\"currency\":\"eur\",\"units\":" + units + ",\"purpose\":\"" + purpose + "\",\"payment_data\":" + payment_data + "}";
    String json_text = "{\"target\":\"#EURS\",\"network\":\"QmNiibPaxdU61jSUK35dRwVQYjF9AC3GScWTRzRdFtZ4vZ\",\"fuel\":1000,\"contract\":\"\",\"method\":\"mint_from_provider\",\"args\":" + contract_args + ",\"private_key\":\"" + pvt_key + "\"}";

    IntPtr intPtr_json = Marshal.StringToHGlobalAnsi(json_text);
    IntPtr intPtr_url = Marshal.StringToHGlobalAnsi(url);

    IntPtr submit_result_ptr = submit_unit_tx(intPtr_json, intPtr_url);
    var submit_result = Marshal.PtrToStringAnsi(submit_result_ptr);
    free_string();
    Console.WriteLine(submit_result);
    if (submit_result == "OK|Valid Transaction!")
    {
        return true;
    }


    return false;
}

String data = "{\"timestamp\":\"2020-05-06T10:22:15\"}";
String purpose = "akdhvkasdvasdv:++EURS";
Int64 units = 1242;


// **Note**: This is a private key used only for testing purpose, don't use it in a production environment
String pvt_key = "9XwbySgVsf1qZvErcMkdGtzDnrDVoRfL6AxQGQ35A2bnCstKZ3pve1ziT4qskoUDZQeMQ6AJZx14hrvPqZeCWw3bNf2thSkxRmuGu7XsnkMDMqJGq7hkA14DffxjQdkqQrg6Aws8SHwXzrZUkFFTL7QK9jcFsT9DHejEwLCwepjJi4MdpzFmiwLySALnMKHm6itCQK9N1HNoc4FL9MJf7mFiQaEi3oG6ufdcyTecPTkuAoi3TpTqtL1MJSDT6kuFH9B9K29yM";
String url = "https://localhost:8000";


if (submit(units, purpose, data, url, pvt_key) == true)
{
    Console.WriteLine("OK");
}
else
{
    Console.WriteLine("ERROR");
}

```
