pub mod operations;

pub mod user_handling;

#[cfg(test)]
// run cargo test to run these
mod tests {
    use super::*;
    use operations::*;
    use user_handling::user::*;

    #[test]
    fn operations() {
        list_operations();
        let result2 = divide::divide(100, 10);
        assert_eq!(result2, 10.);
        let result3 = add::add(100, 10);
        assert_eq!(result3, 110);
        let result4 = subtract::subtract(100, 10);
        assert_eq!(result4, 90.);
        let result5 = multiply::multiply(100, 10);
        assert_eq!(result5, 1000);
    }
    #[test]
    fn users() {
        let us = user_handling::user_handling();
        assert_eq!(us, String::from("This mod is for handling users."));

        let new_user = unsafe {new_user("westsi", "iamwestsi", 25 as u8)};
        let created_user = User {
            name: String::from("westsi"),
            password: String::from("iamwestsi"),
            age: 25 as u8,
            active: true,
            sign_in_count: 1,
        };
        assert_eq!(new_user, created_user);
        assert_eq!(new_user.name, created_user.name);
        assert_eq!(new_user.password, created_user.password);
        assert_eq!(new_user.age, created_user.age);
        assert_eq!(new_user.active, created_user.active);
        assert_eq!(new_user.sign_in_count, created_user.sign_in_count);
    }

}
