use std::{
    collections::HashMap,
    io::{self, Write},
};

#[allow(dead_code)]
struct Inventory {
    inventory: Vec<HashMap<String, i32>>,
}

fn main() {
    let mut new_object: HashMap<String, i32> = HashMap::new();
    new_object.insert(String::from("battery"), 12);

    let _object_bucket = vec![new_object.clone()];

    let product = user_query("enter product:").unwrap_or("null".to_string());
    let quantity = atoi("enter quantity").unwrap_or_default();
    let new_product = product;
    let new_quantity = quantity;
    new_object.insert(new_product, new_quantity);
}

#[allow(dead_code)]
fn user_query(prompt: &str) -> Result<String, io::Error> {
    print!("prompt:{}", prompt);
    std::io::stdout().flush()?;
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_string())
}

fn atoi(prompt: &str) -> Result<i32, io::Error> {
    let buffer = user_query(prompt)?;
    let buffer: i32 = buffer
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "nan"))?;
    Ok(buffer)
}
