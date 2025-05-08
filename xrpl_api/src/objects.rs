mod account_root;
mod offer;
mod ripple_state;
mod ticket;

pub use account_root::*;
pub use offer::*;
pub use ripple_state::*;
use serde::Deserialize;
use ticket::Ticket;

/// Any ledger object. See <https://xrpl.org/ledger-object-types.html>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "LedgerEntryType")]
pub enum LedgerObject {
    AccountRoot(AccountRoot),
    // TODO add model for remaining obejcts
    Amendments,
    Check,
    DepositPreauth,
    DirectoryNode,
    Escrow,
    FeeSettings,
    LedgerHashes,
    NegativeUNL,
    NFTokenOffer,
    NFTokenPage,
    Offer(Offer),
    PayChannel,
    RippleState(RippleState),
    SignerList,
    Ticket(Ticket),
}
