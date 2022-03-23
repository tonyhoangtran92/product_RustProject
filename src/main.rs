//CRUD application - Create - Read - Update - Delete
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Product {
    productName: String,
    price: i32,
}

pub struct ProductList {
    class: HashMap<String, Product>,
}

impl ProductList {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add(&mut self, product: Product) {
        self.class.insert(product.productName.to_string(), product);
    }

    fn view_all(&self) -> Vec<&Product> {
        self.class.values().collect()
    }

    fn remove(&mut self, productName: &str) -> bool {
        self.class.remove(productName).is_some()
    }

    fn edit(&mut self, productName: &str, price: i32) -> bool {
        match self.class.get_mut(productName) {
            Some(products) => {
                products.price = price;
                true
            }
            None => false,
        }
    }
}

mod manager {
    use crate::*;
    pub fn add_product(products: &mut ProductList) {
        println!("Enter name of Product");
        let productName = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price = match get_int() {
            Some(number) => number,
            None => return,
        };
        let product = Product { productName, price };
        products.add(product)
    }

    pub fn view(products: &ProductList) {
        for product in products.view_all() {
            println!("{:?}", product);
        }
    }

    pub fn remove_product(products: &mut ProductList) {
        for product in products.view_all() {
            println!("{:?}", product);
        }
        println!("Please enter name of product you want to delete");
        let productName = match get_input() {
            Some(input) => input,
            None => return,
        };
        if products.remove(&productName) {
            println!("remove product");
        } else {
            println!("not found");
        }
    }

    pub fn edit_product(products: &mut ProductList) {
        for product in products.view_all() {
            println!("{:?}", product);
        }
        println!("Please enter name of product you want to edit");
        let productName = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price = match get_int() {
            Some(input) => input,
            None => return,
        };
        if products.edit(&productName, price) {
            println!("Product has been editted");
        } else {
            println!("not found");
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<i32> {
    println!("Enter Product price");
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

enum Manager {
    AddProduct,
    ViewProducts,
    EditProduct,
    DeleteProduct,
}

impl Manager {
    fn show() {
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Product");
        println!("2. View Products");
        println!("3. Edit Product");
        println!("4. Delete Product");
        println!("");
        println!("Please Enter A Number:");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddProduct),
            "2" => Some(Manager::ViewProducts),
            "3" => Some(Manager::EditProduct),
            "4" => Some(Manager::DeleteProduct),
            _ => None,
        }
    }
}

fn run_program() {
    let mut products = ProductList::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddProduct) => manager::add_product(&mut products),
            Some(Manager::ViewProducts) => manager::view(&products),
            Some(Manager::EditProduct) => manager::edit_product(&mut products),
            Some(Manager::DeleteProduct) => manager::remove_product(&mut products),
            None => break,
        }
    }
}

fn main() {
    run_program();
    println!("exit program");
}