#[derive(Debug, Clone)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    products: Vec<Product>,
}

impl Product {
    fn new(name: &str, description: &str, price: f64, quantity: u32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            price,
            quantity,
        }
    }
}

impl Inventory {
    fn new() -> Self {
        Self { products: Vec::new() }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn edit_product(&mut self, name: &str, new_product: Product) {
        if let Some(index) = self.products.iter().position(|p| p.name == name) {
            self.products[index] = new_product;
        }
    }

    fn delete_product(&mut self, name: &str) {
        self.products.retain(|p| p.name != name);
    }

    fn list_products(&self) {
        for product in &self.products {
            println!("{:?}", product);
        }
    }
}

use std::io::{self, Write};

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("Inventory Management System");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Products");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let product = prompt_product_details();
                inventory.add_product(product);
            },
            "2" => {
                let (name, product) = prompt_edit_product_details();
                inventory.edit_product(&name, product);
            },
            "3" => {
                let name = prompt_product_name();
                inventory.delete_product(&name);
            },
            "4" => {
                inventory.list_products();
            },
            "5" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn prompt_product_details() -> Product {
    let mut name = String::new();
    let mut description = String::new();
    let mut price = String::new();
    let mut quantity = String::new();

    println!("Enter product name:");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter product description:");
    io::stdin().read_line(&mut description).expect("Failed to read line");
    println!("Enter product price:");
    io::stdin().read_line(&mut price).expect("Failed to read line");
    println!("Enter product quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read line");

    // Convert price and quantity to their appropriate types
    let price: f64 = price.trim().parse().expect("Please type a number.");
    let quantity: u32 = quantity.trim().parse().expect("Please type a number.");

    Product::new(&name.trim(), &description.trim(), price, quantity)
}

fn prompt_edit_product_details() -> (String, Product) {
    println!("Enter the name of the product you wish to edit:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // Reuse prompt_product_details to get the new product details
    let new_product = prompt_product_details();

    (name.trim().to_string(), new_product)
}

fn prompt_product_name() -> String {
    println!("Enter the name of the product:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}
