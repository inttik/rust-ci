use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}
fn main() {
    let p = Person {
        name: "men".to_string(),

        age: 24,
        phones: vec!["7-909-...".to_string(), "8-925-...".to_string()],
    };

    println!("{}", serde_json::to_string_pretty(&p).unwrap());
}
