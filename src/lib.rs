
mod handler;
mod tools;
pub mod entity;

#[no_mangle]
pub extern "C" fn test(left: usize, right: usize) -> usize {
    println!("aaabbbcccdddeeeeeeee");
    left + right
}
