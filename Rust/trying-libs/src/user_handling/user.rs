
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct User {
    pub name: String,
    pub password: String,
    pub age: u8,
    pub active: bool,
    pub sign_in_count: u64,
}

static mut USERS: Vec<User> = Vec::new();

pub unsafe fn new_user(name: &str, password: &str, age: u8) -> User {
    let creating_user = User {
        name: String::from(name),
        password: String::from(password),
        age: age,
        active: true,
        sign_in_count: 1,
    };
    USERS.push(creating_user.clone());
    creating_user
}

pub unsafe fn get_users() -> Vec<User> {
    USERS.clone()
}