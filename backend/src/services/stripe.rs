use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BillingMessage {
    pub message: &'static str,
}

pub fn not_configured() -> BillingMessage {
    BillingMessage {
        message: "billing is not configured yet",
    }
}
