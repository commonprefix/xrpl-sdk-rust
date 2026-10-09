use crate::{Amount, TransactionCommon};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::PaymentFlags;

/// An `Payment` transaction <https://xrpl.org/payment.html>
#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<PaymentFlags>,
    pub amount: Amount,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_max: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_min: Option<Amount>,
}

#[cfg(test)]
mod test {
    use crate::PaymentTransaction;
    use xrpl_types::PaymentFlags;

    #[test]
    fn test_payment_deserialize() {
        let json = r#"
{
  "TransactionType" : "Payment",
  "Account" : "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
  "Destination" : "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
  "Amount" : {
     "currency" : "USD",
     "value" : "1",
     "issuer" : "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"
  },
  "Fee": "12",
  "Flags": 2147483648,
  "Sequence": 2
}
        "#;

        let _: PaymentTransaction = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn test_inner_batch_payment_deserialize() {
        // mainnet B17E9C28D28FD708CF13AFA5193EF9B4A9B0175D168AED5C0C9036F55BA4BEBE (ledger 107541479):
        // inner Batch transaction (XLS-56) with tfInnerBatchTxn | tfPartialPayment | tfNoRippleDirect.
        let json = r#"
{
  "TransactionType": "Payment",
  "Account": "rBqUZzPRcVus1phirwuPd7f6QJQQsCdyax",
  "Destination": "rBqUZzPRcVus1phirwuPd7f6QJQQsCdyax",
  "Amount": {
    "currency": "5245414C00000000000000000000000000000000",
    "issuer": "rKVyXn1AhqMTvNA9hS6XkFjQNn2VE8Nz88",
    "value": "9999999999999990e-1"
  },
  "SendMax": "1273679",
  "Fee": "0",
  "Flags": 1073938432,
  "Sequence": 0,
  "SigningPubKey": "",
  "TicketSequence": 107285947
}
        "#;

        let tx: PaymentTransaction = serde_json::from_str(json).unwrap();
        assert!(tx.flags.contains(PaymentFlags::InnerBatchTxn));
        assert!(tx.flags.contains(PaymentFlags::PartialPayment));
        assert!(tx.flags.contains(PaymentFlags::NoDirectRipple));
    }
}
