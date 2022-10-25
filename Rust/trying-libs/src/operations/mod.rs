pub fn list_operations() {
    println!("Divide\nAdd\nSubtract\nMultiply")
}

pub fn ret_operations() -> Vec<&'static str> {
    let vec1 = vec!["Divide", "Add", "Subtract", "Multiply"];
    vec1
}
pub mod divide;
pub mod add;
pub mod subtract;
pub mod multiply;