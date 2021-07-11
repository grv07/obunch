use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    lastname: String,
    firstname: String,
    email: String,
    phone: String,
    addr: String,
}

struct Order {}
