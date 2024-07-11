
mod handler;
mod tools;
pub mod entity;

#[no_mangle]
pub extern "C" fn test(left: usize, right: usize) -> usize {
    println!("aaabbbccc");
    left + right
}
