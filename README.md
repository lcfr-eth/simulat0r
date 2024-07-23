## revm + foundry + alloy simulation demo
using latest foundry + revm.  

## usage:

```bash
Usage: simulat0r [OPTIONS] --to <TO>

Options:
      --rpc-url <RPC_URL>            RPC URL [default: https://eth.llamarpc.com]
      --block-number <BLOCK_NUMBER>  Block Number
      --from <FROM>                  From address
      --to <TO>                      To address
      --data <DATA>                  Data (hex string)
      --value <VALUE>                Value
  -h, --help                         Print help
  -V, --version                      Print version
  ```  

## Example 
call ```skim()``` on ```0x48ae077696196cd604c0AF89c098Effd285ccf01``` at block ```20065826``` and check the execution result & logs produced:

```bash
cargo run -- --to 0x48ae077696196cd604c0AF89c098Effd285ccf01 --data 0xbc25cf7700000000000000000000000048ae077696196cd604c0af89c098effd285ccf01 --block-number 20065826

Success {
    reason: Stop,
    gas_used: 74729,
    gas_refunded: 2800,
    logs: [
        Log {
            address: 0xd46ba6d942050d489dbd938a2c909a5d5039a161,
            data: LogData {
                topics: [
                    0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef,
                    0x00000000000000000000000048ae077696196cd604c0af89c098effd285ccf01,
                    0x00000000000000000000000048ae077696196cd604c0af89c098effd285ccf01,
                ],
                data: 0x0000000000000000000000000000000000000000000000000000000000000000,
            },
        },
        Log {
            address: 0xdac17f958d2ee523a2206206994597c13d831ec7,
            data: LogData {
                topics: [
                    0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef,
                    0x00000000000000000000000048ae077696196cd604c0af89c098effd285ccf01,
                    0x00000000000000000000000048ae077696196cd604c0af89c098effd285ccf01,
                ],
                data: 0x0000000000000000000000000000000000000000000000000000000000000000,
            },
        },
    ],
    output: Call(
        0x,
    ),
}

```

if no --block-number is specified it uses the latest/current block.