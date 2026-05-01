use std::{
    collections::HashMap,
    io::{self, Write},
};

#[allow(dead_code)]
struct Inventory {
    inventory: Vec<HashMap<String, i32>>,
}

fn main() {}

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

// impl Inventory {
//     // fn new()
//     //
//     fn diplap_inventory(&self,)
// }
//
// #[allow(unused_variableslint)]
#[allow(dead_code)]
fn add_new_product(prompt: &str) {
    print!("prompt:{}", prompt);
    std::io::stdout().flush().unwrap_or_default();
    loop {
        let mut new_object: HashMap<String, i32> = HashMap::new();
        new_object.insert(String::from("battery"), 12);
        new_object.insert("keyboard".to_string(), 4);

        let product = user_query("enter product:").unwrap_or("null".to_string());
        let quantity = atoi("enter quantity").unwrap_or_default();
        let new_product = product;
        let new_quantity = quantity;

        new_object.insert(new_product.clone(), new_quantity);
        let object_bucket: Vec<HashMap<String, i32>> = vec![new_object];
        // let mut object_bucket: Vec<HashMap<String, i32>> = Vec::new();
        //
        // // object_bucket = vec![new_object.clone()];
        // object_bucket.push(new_object.clone());
        // new_object.insert(new_product, new_quantity);

        // Inventory:object_bucket.push(new_object.insert(new_product, new_quantity));
        // let object_file = Inventory {
        //     inventory: vec![
        //         // new_object.insert(new_product, new_quantity),
        //         HashMap::from(new_product, new_quantity),
        //     ], // Inventory:object_bucket.push(new_object.insert(new_product, new_quantity));
        // };

        for values in &object_bucket {
            println!("values:{:?}", values);
        }
    }
}
