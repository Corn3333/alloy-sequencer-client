use alloy_primitives::{U256, Bytes, TxKind, B256};
use alloy_rlp::{RlpEncodable, RlpDecodable};
use alloy_rpc_types::AccessList;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub version: i64,
    pub messages: Vec<BroadcastFeedMessage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BroadcastFeedMessage {
    pub sequence_number: i64,
    pub message: MessageWithMetadata,
    // pub signature: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageWithMetadata {
    pub message: L1IncomingMessageHeader,
    pub delayed_messages_read: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessageHeader {
    pub header: Header,
    #[serde(rename = "l2Msg")]
    pub l2msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub kind: i64,
    pub sender: String,
    pub block_number: i64,
    pub timestamp: i64,
    pub request_id: Value,
    pub base_fee_l1: Value,
}


#[derive(Debug)]
pub enum TypedTransaction {
    Legacy(TxLegacy),
    Eip2930(TxEip2930),
    Eip1559(TxEip1559),
}

impl TypedTransaction {
    pub fn into_info(self) -> TransactionInfo {
        match self {
            Self::Legacy(tx_legacy) => TransactionInfo {
                        to: tx_legacy.to,
                        value: tx_legacy.value,
                        data: tx_legacy.data,
                        tx_hash: tx_legacy.tx_hash.unwrap()
            },
            Self::Eip2930(tx_eip2930) => TransactionInfo {
                        to: tx_eip2930.to,
                        value: tx_eip2930.value,
                        data: tx_eip2930.data,
                        tx_hash: tx_eip2930.tx_hash.unwrap()
            },
            Self::Eip1559(tx_eip1559) => TransactionInfo {
                        to: tx_eip1559.to,
                        value: tx_eip1559.value,
                        data: tx_eip1559.data,
                        tx_hash: tx_eip1559.tx_hash.unwrap()
            },
        }
    }

    pub fn into_legacy(self) -> Option<TxLegacy> {
        match self {
            Self::Legacy(legacy) => Some(legacy),
            _ => None
        }
    }

    pub fn into_eip2930(self) -> Option<TxEip2930> {
        match self {
            Self::Eip2930(eip2930) => Some(eip2930),
            _ => None
        }
    }

    pub fn into_eip1559(self) -> Option<TxEip1559> {
        match self {
            Self::Eip1559(eip1559) => Some(eip1559),
            _ => None
        }
    }        
}




#[derive(Debug)]
pub struct TransactionInfo {
    pub to: TxKind,
    pub value: U256,
    pub data: Bytes,
    pub tx_hash: B256,
}



#[derive(Debug, RlpEncodable, RlpDecodable, PartialEq)]
#[rlp(trailing)]
pub struct TxLegacy {    
    pub nonce: u64,
    pub gas_price: u128,
    pub gas_limit: u64,
    pub to: TxKind,
    pub value: U256,
    pub data: Bytes,
    pub v: u64,
    pub r: U256,
    pub s: U256,    
    pub tx_hash: Option<B256>,
}



#[derive(Debug, RlpEncodable, RlpDecodable, PartialEq)]
#[rlp(trailing)]
pub struct TxEip1559 {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: u128,
    pub max_fee_per_gas: u128,
    pub gas: u64,    
    pub to: TxKind,    
    pub value: U256,    
    pub data: Bytes,
    pub access_list: AccessList,    
    pub v: u64,
    pub r: U256,
    pub s: U256,
    pub tx_hash: Option<B256>,
}


#[derive(Debug, RlpEncodable, RlpDecodable, PartialEq)]
#[rlp(trailing)]
pub struct TxEip2930 {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u128,
    pub gas: u64,    
    pub to: TxKind,    
    pub value: U256,    
    pub data: Bytes,
    pub access_list: AccessList,    
    pub v: u64,
    pub r: U256,
    pub s: U256,
    pub tx_hash: Option<B256>,
}
