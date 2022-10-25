pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod operations;

#[cfg(test)]
// run cargo test to run these
mod tests {
    use super::*;
    use operations::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

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
}
