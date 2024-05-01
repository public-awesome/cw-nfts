use cosmwasm_std::{Addr, attr, Event};
use crate::TokenAmount;


/// Tracks token transfer actions
pub struct TransferEvent {
    pub sender: Addr,
    pub recipient: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl TransferEvent {
    pub fn new(sender: &Addr, recipient: &Addr, tokens: Vec<TokenAmount>) -> Self {
        Self {
            sender: sender.clone(),
            recipient: recipient.clone(),
            tokens,
        }
    }
}

impl From<TransferEvent> for Event {
    fn from(event: TransferEvent) -> Self {
        Event::new("transfer_tokens").add_attributes(
            vec![
                attr("sender", event.sender.as_str()),
                attr("recipient", event.recipient.as_str()),
                attr("tokens", event.tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",")),
            ]
        )
    }
}

/// Tracks token mint actions
pub struct MintEvent {
    pub recipient: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl MintEvent {
    pub fn new(recipient: &Addr, tokens: Vec<TokenAmount>) -> Self {
        Self {
            recipient: recipient.clone(),
            tokens,
        }
    }
}

impl From<MintEvent> for Event {
    fn from(event: MintEvent) -> Self {
        Event::new("mint_tokens").add_attributes(
            vec![
                attr("recipient", event.recipient.as_str()),
                attr("tokens", event.tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",")),
            ]
        )
    }
}

/// Tracks token burn actions
pub struct BurnEvent {
    pub sender: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl BurnEvent {
    pub fn new(sender: &Addr, tokens: Vec<TokenAmount>) -> Self {
        Self {
            sender: sender.clone(),
            tokens,
        }
    }
}

impl From<BurnEvent> for Event {
    fn from(event: BurnEvent) -> Self {
        Event::new("burn_tokens").add_attributes(
            vec![
                attr("sender", event.sender.as_str()),
                attr("tokens", event.tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(",")),
            ]
        )
    }
}

/// Tracks approve_all status changes
pub struct ApproveAllEvent {
    pub sender: Addr,
    pub operator: Addr,
}

impl ApproveAllEvent {
    pub fn new(sender: &Addr, operator: &Addr) -> Self {
        Self {
            sender: sender.clone(),
            operator: operator.clone(),
        }
    }
}

impl From<ApproveAllEvent> for Event {
    fn from(event: ApproveAllEvent) -> Self {
        Event::new("approve_all").add_attributes(
            vec![
                attr("sender", event.sender.as_str()),
                attr("operator", event.operator.as_str()),
            ]
        )
    }
}

/// Tracks revoke_all status changes
pub struct RevokeAllEvent {
    pub sender: Addr,
    pub operator: Addr,
}

impl RevokeAllEvent {
    pub fn new(sender: &Addr, operator: &Addr) -> Self {
        Self {
            sender: sender.clone(),
            operator: operator.clone(),
        }
    }
}

impl From<RevokeAllEvent> for Event {
    fn from(event: RevokeAllEvent) -> Self {
        Event::new("revoke_all").add_attributes(
            vec![
                attr("sender", event.sender.as_str()),
                attr("operator", event.operator.as_str()),
            ]
        )
    }
}
