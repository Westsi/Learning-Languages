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

        let new_user = User::new("westsi", "iamwestsi", 25 as u8);
        assert_eq!(new_user.get_name(), String::from("westsi"));
    }

}
