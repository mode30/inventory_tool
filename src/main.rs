use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

#[allow(dead_code)]
struct Inventory {
    inventory: Vec<HashMap<String, u16>>,
}

fn main() {
    let mut new_object: HashMap<String, u16> = HashMap::new();
    new_object.insert(String::from("battery"), 12);

    let _object_bucket = vec![new_object];
    // println!
}

#[allow(dead_code)]
fn user_query(prompt: &str) -> Result<String, io::Error> {
    print!("prompt:{}", prompt);
    std::io::stdout().flush()?;
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_string())
}
