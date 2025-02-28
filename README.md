## Status
Supports Legacy, Eip1559 and Eip2930 decoding. 
Returns only L2MessageKind_SignedTx messages. 

Also supports L2MessageKind_Batch, but by default it is not provided, because I don't know what these transactions are, they don't get into the blockchain. If you know what to do with them -> open issues.

## Installation

Add `alloy-sequencer-client` to your `Cargo.toml`.

```toml
alloy-sequencer-client = {git = "https://github.com/Corn3333/alloy-sequencer-client"}
```

## Quick Start
To use this sequencer-client, you'll need Tokio as your main runtime.

Here is a basic example.
```Rust
use alloy_sequencer_client::feed_clients::RelayClients;
use tokio::sync::mpsc::channel;


#[tokio::main]
async fn main() {
    // Create a channel to receive messages from the feed client
    let (sender, mut receiver) = channel(4096);
    // Create a new relay client and start background maintenance
    let relay_client = RelayClients::new("wss://arb1.arbitrum.io/feed", 3, 1, sender)
        .await
        .expect("Failed to create relay client");
    tokio::spawn(RelayClients::start_reader(relay_client));

    // To prevent duplicate messages
    let mut highest_seq_number: i64 = 0;
    
    while let Some(data) = receiver.recv().await {
        
        let msg_seq_num = data.messages[0].sequence_number;
            
        if highest_seq_number >= msg_seq_num {
            continue;
        }

        highest_seq_number = msg_seq_num;

        for feed_msg in data.messages.iter() {

            if let Some(vec_typed) = feed_msg.message.message.decode() {
                for typed_tx in vec_typed {
                    let tx_info = typed_tx.into_info();
                    println!("{:?}", tx_info);
                }
            }
        }        
    }
}
```

## Credits

- [alloy]
- [fulcrum]
- [sequencer-client-rs]

[alloy]: https://github.com/alloy-rs
[fulcrum]: https://github.com/jordy25519/fulcrum
[sequencer-client-rs]: https://github.com/duoxehyon/sequencer-client-rs