pub enum Status{
    Active,
    Inactive,
    Warning(u32),
}

pub struct Plant{
    pub name: String,
    pub status: Status,
}

pub fn report_status(p: &Plant){
    match p.status {
        Status::Active => println!("{} Its active", p.name),
        Status::Inactive => println!("{} Its inactive",p.name),
        Status::Warning(code) => println!("{} Alert Error: {}",p.name, code),
    }

}