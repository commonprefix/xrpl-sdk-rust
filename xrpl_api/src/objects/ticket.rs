use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};

use super::RippleStateFlags;

/// A ticket in the ledger.
///
/// <https://xrpl.org/docs/references/protocol/ledger-data/ledger-entry-types/ticket>
///
/// {
///     "Account": "rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de",
///     "Flags": 0,
///     "LedgerEntryType": "Ticket",
///     "OwnerNode": "0000000000000000",
///     "PreviousTxnID": "F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1",
///     "PreviousTxnLgrSeq": 4,
///     "TicketSequence": 3
/// }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    pub account: String,
    pub flags: BitFlags<RippleStateFlags>,
    pub owner_node: String,
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: String,
    pub previous_txn_lgr_seq: u32,
    pub ticket_sequence: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_ticket() {
        let json = r#"
{
  "Account": "rEhxGqkqPPSxQ3P25J66ft5TwpzV14k2de",
  "Flags": 0,
  "LedgerEntryType": "Ticket",
  "OwnerNode": "0000000000000000",
  "PreviousTxnID": "F19AD4577212D3BEACA0F75FE1BA1644F2E854D46E8D62E9C95D18E9708CBFB1",
  "PreviousTxnLgrSeq": 4,
  "TicketSequence": 3
}
"#;

        let _ticket: Ticket = serde_json::from_str(json).unwrap();
    }
}
