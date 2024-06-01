pub struct Bird {
    pub name: String,
    pub attack: u16,
}

impl Bird {
    pub fn print_name(&self) {
        println!("{} and attack is {}", self.name, self.attack);
    }
}
