use crate::msg::TokenAmount;
use cosmwasm_std::{attr, Addr, Attribute, Event, MessageInfo, Uint128};

/// Tracks token transfer actions
pub struct TransferEvent {
    pub owner: Addr,
    pub sender: Addr,
    pub recipient: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl TransferEvent {
    pub fn new(
        info: &MessageInfo,
        from: Option<Addr>,
        recipient: &Addr,
        tokens: Vec<TokenAmount>,
    ) -> Self {
        Self {
            owner: from.unwrap_or_else(|| info.sender.clone()),
            sender: info.sender.clone(),
            recipient: recipient.clone(),
            tokens,
        }
    }
}

impl From<TransferEvent> for Event {
    fn from(event: TransferEvent) -> Self {
        Event::new(format!(
            "transfer_{}",
            if event.tokens.len() == 1 {
                "single"
            } else {
                "batch"
            }
        ))
        .add_attributes(vec![
            attr("owner", event.owner.as_str()),
            attr("sender", event.sender.as_str()),
            attr("recipient", event.recipient.as_str()),
        ])
        .add_attributes(token_attributes(event.tokens))
    }
}

/// Tracks token mint actions
pub struct MintEvent {
    pub sender: Addr,
    pub recipient: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl MintEvent {
    pub fn new(info: &MessageInfo, recipient: &Addr, tokens: Vec<TokenAmount>) -> Self {
        Self {
            sender: info.sender.clone(),
            recipient: recipient.clone(),
            tokens,
        }
    }
}

impl From<MintEvent> for Event {
    fn from(event: MintEvent) -> Self {
        Event::new(format!(
            "mint_{}",
            if event.tokens.len() == 1 {
                "single"
            } else {
                "batch"
            }
        ))
        .add_attribute("sender", event.sender.as_str())
        .add_attribute("recipient", event.recipient.as_str())
        .add_attributes(token_attributes(event.tokens))
    }
}

/// Tracks token burn actions
pub struct BurnEvent {
    pub owner: Addr,
    pub sender: Addr,
    pub tokens: Vec<TokenAmount>,
}

impl BurnEvent {
    pub fn new(info: &MessageInfo, from: Option<Addr>, tokens: Vec<TokenAmount>) -> Self {
        Self {
            owner: from.unwrap_or_else(|| info.sender.clone()),
            sender: info.sender.clone(),
            tokens,
        }
    }
}

impl From<BurnEvent> for Event {
    fn from(event: BurnEvent) -> Self {
        Event::new(format!(
            "burn_{}",
            if event.tokens.len() == 1 {
                "single"
            } else {
                "batch"
            }
        ))
        .add_attribute("owner", event.owner.as_str())
        .add_attribute("sender", event.sender.as_str())
        .add_attributes(token_attributes(event.tokens))
    }
}

/// Tracks approve status changes
pub struct ApproveEvent {
    pub sender: Addr,
    pub operator: Addr,
    pub token_id: String,
    pub amount: Uint128,
}

impl ApproveEvent {
    pub fn new(sender: &Addr, operator: &Addr, token_id: &str, amount: Uint128) -> Self {
        Self {
            sender: sender.clone(),
            operator: operator.clone(),
            token_id: token_id.to_string(),
            amount,
        }
    }
}

impl From<ApproveEvent> for Event {
    fn from(event: ApproveEvent) -> Self {
        Event::new("approve_single").add_attributes(vec![
            attr("sender", event.sender.as_str()),
            attr("operator", event.operator.as_str()),
            attr("token_id", event.token_id),
            attr("amount", event.amount.to_string()),
        ])
    }
}

/// Tracks revoke status changes
pub struct RevokeEvent {
    pub sender: Addr,
    pub operator: Addr,
    pub token_id: String,
    pub amount: Uint128,
}

impl RevokeEvent {
    pub fn new(sender: &Addr, operator: &Addr, token_id: &str, amount: Uint128) -> Self {
        Self {
            sender: sender.clone(),
            operator: operator.clone(),
            token_id: token_id.to_string(),
            amount,
        }
    }
}

impl From<RevokeEvent> for Event {
    fn from(event: RevokeEvent) -> Self {
        Event::new("revoke_single").add_attributes(vec![
            attr("sender", event.sender.as_str()),
            attr("operator", event.operator.as_str()),
            attr("token_id", event.token_id),
            attr("amount", event.amount.to_string()),
        ])
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
        Event::new("approve_all").add_attributes(vec![
            attr("sender", event.sender.as_str()),
            attr("operator", event.operator.as_str()),
        ])
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
        Event::new("revoke_all").add_attributes(vec![
            attr("sender", event.sender.as_str()),
            attr("operator", event.operator.as_str()),
        ])
    }
}

pub fn token_attributes(tokens: Vec<TokenAmount>) -> Vec<Attribute> {
    vec![
        attr(
            format!("token_id{}", if tokens.len() == 1 { "" } else { "s" }),
            tokens
                .iter()
                .map(|t| t.token_id.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
        attr(
            format!("amount{}", if tokens.len() == 1 { "" } else { "s" }),
            tokens
                .iter()
                .map(|t| t.amount.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
    ]
}

pub struct UpdateMetadataEvent {
    pub token_id: String,
    pub token_uri: String,
    pub extension_update: bool,
}

impl UpdateMetadataEvent {
    pub fn new(token_id: &str, token_uri: &str, extension_update: bool) -> Self {
        Self {
            token_id: token_id.to_string(),
            token_uri: token_uri.to_string(),
            extension_update,
        }
    }
}

impl From<UpdateMetadataEvent> for Event {
    fn from(event: UpdateMetadataEvent) -> Self {
        Event::new("update_metadata").add_attributes(vec![
            attr("token_id", event.token_id),
            attr("token_uri", event.token_uri),
            attr("extension_update", event.extension_update.to_string()),
        ])
    }
}
