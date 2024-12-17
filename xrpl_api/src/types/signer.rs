use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Signer {
    pub account: String,
    pub signing_pub_key: String,
    pub txn_signature: String,
}
