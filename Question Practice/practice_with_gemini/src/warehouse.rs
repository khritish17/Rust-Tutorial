pub enum ItemType {
    Physical(f64),
    Digital(u64)
}

pub struct Product {
    pub name: String,
    pub kind: ItemType
}

pub fn print_details(product: &Product){
    match &product.kind {
        ItemType::Physical(weight) => println!("The Item {} weighs {}", product.name, weight),
        ItemType::Digital(size) => println!("The Item {} is {} MB", product.name, size),
        

    }
}