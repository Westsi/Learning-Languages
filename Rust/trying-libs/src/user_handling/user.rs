#[derive(Debug)]
#[derive(PartialEq)]
pub struct User {
    name: String,
    password: String,
    age: u8,
    active: bool,
    sign_in_count: u64,
}

impl User {
    pub fn new(name: &str, password: &str, age: u8) -> User {
        User {
            name: String::from(name),
            password: String::from(password),
            age: age,
            active: true,
            sign_in_count: 1,
        }
    }

    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }
    
    pub fn get_password(&self) -> String {
        String::from(&self.password)
    }

}
