use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Shop {
    pub id: Uuid,
    pub name: String,
    pub menu_ids: Vec<String>,
    pub branch_name: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
