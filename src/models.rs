use serde::Deserialize;

fn default_fk() -> String {
    "NA".to_string()
}

fn default_phone() -> String {
    "NA".to_string()
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    email: String,
    #[serde(default = "default_phone")]
    phone: String,
    #[serde(default = "default_fk")]
    addr_id: String,
}

struct Address {
    text: String,
}

struct Item {
    title: String,
    price: f64,
    is_available: bool,
}

enum PaymentType {
    CashOnDelivery,
    PrePaid,
}

struct Order {
    item_id: String,
    total_price: i64,
    owner: String,
    payment_type: PaymentType,
}
