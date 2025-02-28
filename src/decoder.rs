use base64::{engine::general_purpose, Engine as _};
use alloy_primitives::keccak256;
use alloy_rlp::Decodable;
use crate::types::{L1IncomingMessageHeader, TxLegacy, TxEip1559, TxEip2930, TypedTransaction};

const MAX_L2_MESSAGE_SIZE: usize = 256 * 1024;
const L1_MESSAGE_TYPE_L2_MESSAGE: i64 = 3;



impl L1IncomingMessageHeader {
    pub fn decode(&self) -> Option<Vec<TypedTransaction>> {
        if self.l2msg.len() > MAX_L2_MESSAGE_SIZE {
            return None;
        }


        if self.header.kind == L1_MESSAGE_TYPE_L2_MESSAGE {
            return general_purpose::STANDARD
                .decode(self.l2msg.as_bytes())
                .ok()
                .and_then(Self::parse_l2_message);
        }
        
        None
    }

    
    fn parse_l2_message(data: Vec<u8>) -> Option<Vec<TypedTransaction>> {
        let kind = *data.first()?;
        if kind == 4 { // L2MessageKind_SignedTx
            let mut buf_after = data.get(1..)?;
            let pointer = *buf_after.first()?;
            // Regular transactions
            if pointer > 0x7f {
                // Legacy tx
                let new_tx_hash = keccak256(buf_after);       
                let mut decoded = TxLegacy::decode(&mut buf_after).ok()?;   
                decoded.tx_hash = Some(new_tx_hash);                 
                return Some(vec![TypedTransaction::Legacy(decoded)]);  

            } else if pointer == 1 {
                // EIP-2930
                let new_tx_hash = keccak256(buf_after);
                let mut decoded = TxEip2930::decode(&mut &buf_after[1..]).ok()?;    
                decoded.tx_hash = Some(new_tx_hash);                            
                return Some(vec![TypedTransaction::Eip2930(decoded)]);  

            } else if pointer == 2 {
                // EIP 1559
                let new_tx_hash = keccak256(buf_after);
                let mut decoded = TxEip1559::decode(&mut &buf_after[1..]).ok()?;                
                decoded.tx_hash = Some(new_tx_hash);                   
                return Some(vec![TypedTransaction::Eip1559(decoded)]);

            } else {
                // let bytes_data = Bytes::copy_from_slice(&data);
                // println!("else: {:?}", bytes_data);    
                // todo!()
            }
        }
        // else if kind == 3 { // L2MessageKind_Batch
        //     let buffer = data.get(1..)?;

        //     let mut vec_tx = Vec::new();

        //     let mut offset: usize = 0;
        //     let len = buffer.len();
        //     for _ in 0..128 {
        //         let msg_length = as_usize(&buffer[offset..]);
        //         offset += 8_usize;
                
        //         let mut new_data = &buffer[offset + 1..];             
        //         let pointer = *new_data.first()?;
        //         // Regular transactions
        //         if pointer > 0x7f {
        //             let new_tx_hash = keccak256(new_data);       
        //             if let Ok(mut decoded) = TxLegacy::decode(&mut new_data) {
        //                 decoded.tx_hash = Some(new_tx_hash); 
        //                 vec_tx.push(TypedTransaction::Legacy(decoded));    
        //             }               
        
        //         } else if pointer == 1 {
        //             let new_tx_hash = keccak256(new_data);
        //             if let Ok(mut decoded) = TxEip2930::decode(&mut &new_data[1..]) {
        //                 decoded.tx_hash = Some(new_tx_hash);     
        //                 vec_tx.push(TypedTransaction::Eip2930(decoded));  
        //             }               
  
        //         } else if pointer == 2 {
        //             let new_tx_hash = keccak256(new_data);
        //             if let Ok(mut decoded) = TxEip1559::decode(&mut &new_data[1..]) {
        //                 decoded.tx_hash = Some(new_tx_hash);  
        //                 vec_tx.push(TypedTransaction::Eip1559(decoded));     
        //             }             
               
        //         }                
        
        //         offset += msg_length;
        //         if offset + 9 > len {
        //             break;
        //         }        
        //     }     

        //     return Some(vec_tx);       
        // } else {
        //     // println!("unknown kind: {}", kind);
        // }
        None
    }
}


// #[inline(always)]
// fn as_usize(buf: &[u8]) -> usize {
//     // OPTIMIZATION: nothing sensible should ever be longer than 2 ** 16 so we ignore the other bytes
//     // ((unsafe { *buf.get_unchecked(28) } as usize) << 24)
//     //     + ((unsafe { *buf.get_unchecked(29) } as usize) << 16)
//     ((unsafe { *buf.get_unchecked(5) } as usize) << 16)
//         + ((unsafe { *buf.get_unchecked(6) } as usize) << 8)
//         + unsafe { *buf.get_unchecked(7) } as usize
// }