use crate::msg::TokenAmount;
use cosmwasm_std::{attr, Addr, Attribute, MessageInfo, Uint128};

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

impl IntoIterator for TransferEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut attrs = vec![
            event_action("transfer", &self.tokens),
            attr("owner", self.owner.as_str()),
            attr("sender", self.sender.as_str()),
            attr("recipient", self.recipient.as_str()),
        ];
        attrs.extend(token_attributes(self.tokens));
        attrs.into_iter()
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

impl IntoIterator for MintEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut attrs = vec![
            event_action("mint", &self.tokens),
            attr("sender", self.sender.as_str()),
            attr("recipient", self.recipient.as_str()),
        ];
        attrs.extend(token_attributes(self.tokens));
        attrs.into_iter()
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

impl IntoIterator for BurnEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut attrs = vec![
            event_action("burn", &self.tokens),
            attr("owner", self.owner.as_str()),
            attr("sender", self.sender.as_str()),
        ];
        attrs.extend(token_attributes(self.tokens));
        attrs.into_iter()
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

impl IntoIterator for ApproveEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            attr("action", "approve_single"),
            attr("sender", self.sender.as_str()),
            attr("operator", self.operator.as_str()),
            attr("token_id", self.token_id),
            attr("amount", self.amount.to_string()),
        ]
        .into_iter()
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

impl IntoIterator for RevokeEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            attr("action", "revoke_single"),
            attr("sender", self.sender.as_str()),
            attr("operator", self.operator.as_str()),
            attr("token_id", self.token_id),
            attr("amount", self.amount.to_string()),
        ]
        .into_iter()
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

impl IntoIterator for ApproveAllEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            attr("action", "approve_all"),
            attr("sender", self.sender.as_str()),
            attr("operator", self.operator.as_str()),
        ]
        .into_iter()
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

impl IntoIterator for RevokeAllEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            attr("action", "revoke_all"),
            attr("sender", self.sender.as_str()),
            attr("operator", self.operator.as_str()),
        ]
        .into_iter()
    }
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

impl IntoIterator for UpdateMetadataEvent {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            attr("action", "update_metadata"),
            attr("token_id", self.token_id),
            attr("token_uri", self.token_uri),
            attr("extension_update", self.extension_update.to_string()),
        ]
        .into_iter()
    }
}

pub fn event_action(action: &str, tokens: &[TokenAmount]) -> Attribute {
    let action = format!(
        "{}_{}",
        action,
        if tokens.len() == 1 { "single" } else { "batch" }
    );
    attr("action", action)
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
