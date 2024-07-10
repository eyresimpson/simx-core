use crate::entity::function::{Function, FunctionTypes};

mod handler;
mod tools;
pub mod entity;

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn list() -> Vec<Function> {
    return vec![
        Function{
            name: "test".to_string(),
            description: "sss".to_string(),
            params: vec![],
            return_type: FunctionTypes::Str,
            is_async: false,
            is_static: false,
        }
    ];
}
