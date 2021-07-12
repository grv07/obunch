use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    email: String,
    phone: String,
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
