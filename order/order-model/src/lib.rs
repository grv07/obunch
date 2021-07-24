use serde::Deserialize;
use uuid::Uuid;

fn default_fk() -> String {
    "NA".to_string()
}

fn default_phone() -> String {
    "NA".to_string()
}

fn default_owner() -> String {
    "NA".to_string()
}

#[derive(Deserialize, Debug)]
enum PaymentType {
    Undefined,
    CashOnDelivery,
    PrePaid,
}

impl PaymentType {
    fn default() -> PaymentType {
       PaymentType::Undefined 
    }
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub id: Uuid,
    pub item_ids: Vec<String>,
    pub total_price: i64,
    #[serde(default = "default_owner")]
    owner_id: String,
    #[serde(default = "PaymentType::default")]
    payment_type: PaymentType,
    pub is_active: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
