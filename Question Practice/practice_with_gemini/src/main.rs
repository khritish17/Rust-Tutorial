mod lesson1_editor;
mod power_plant;
mod warehouse;
use warehouse::{Product, ItemType, print_details};

fn main() {
    // for lesson1_editors.rs
    // lesson1_editor::editor();

    // for power_plant.rs
    // let plant = power_plant::Plant{
    //     name: String::from("Lily"),
    //     status: power_plant::Status::Warning(300),
    // };
    // println!("Created plant {}", plant.name);
    // power_plant::report_status(&plant);


    // for warehouse.rs
    // let product = warehouse::Product{
    //     name: String::from("Interstellar"),
    //     kind: warehouse::ItemType::Digital(700),
    // };
    // warehouse::print_details(&product);

    let product = Product{
        name: String::from("Interstellar"),
        kind: ItemType::Digital(700),
    };
    print_details(&product);

}