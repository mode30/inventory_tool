use std::{
    collections::HashMap,
    io::{self, Write},
};

#[allow(dead_code)]
enum User {
    LoggedIn { user: String },
    EnterName { name: String },
    LoggedOut,
}
#[allow(dead_code)]
struct Inventory {
    inventory: Vec<HashMap<String, i32>>,
}

fn main() {
    let mut state = User::LoggedOut;

    loop {
        match &mut state {
            User::LoggedOut => {
                println!("welcome\n enter quit or login");
                println!("Enter:>:");
                let cmd = user_query("enter command:").unwrap_or_default();
                if cmd.as_str() == "login" {
                    state = User::EnterName {
                        name: String::new(),
                    }
                } else if cmd.as_str() == "quit" {
                    break;
                }
            }
            User::EnterName { name } => {
                if name.is_empty() {
                    // println!("enter your name:");
                    *name = user_query("enter name:").unwrap_or_default();

                    println!("welcome:{}", name);
                } else {
                    println!("welcome:{}", name);
                    state = User::LoggedIn { user: name.clone() }
                }
            }
            User::LoggedIn { user } => {
                println!("you are logged in as :{}", user);
                println!("enter 1:>add inventory\n2:>quit");
                // println!("enter 1:>add inventory\n2:>display all products\n3:>quit");
                let input = atoi(">:").unwrap_or_default();
                if input == 1 {
                    add_new_product_and_display("enter product to add followed by quantity");
                } else if input == 2 {
                    println!("thank,you come back later!!");
                    state = User::LoggedOut
                }
            } // _ => {
              //     println!("error incorrect entry");
              // }
        }
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

    // impl Inventory {
    //     // fn new()
    //     //
    //     fn diplap_inventory(&self,)
    // }
    //
    // #[allow(unused_variableslint)]
    #[allow(dead_code)]
    fn add_new_product_and_display(prompt: &str) {
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
}
