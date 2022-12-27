#![allow(unused)]
pub struct  Advanced{

}

macro_rules! yo {
    ($name:expr) => {
        println!("Yo {}!",$name)
    };
}
impl Advanced {
    pub fn marco(){
     yo!("Fini")
    }
}
