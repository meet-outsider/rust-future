#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub introduce: String,
}

impl User {
    pub fn new(name: String, age: u8, introduce: String) -> User {
        User {
            name,
            age,
            introduce,
        }
    }
    pub fn print_self(self) {
        println!("{:#?}", self);
    }
}
