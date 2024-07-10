pub struct Function {
    pub name: String,
    pub description: String,
    pub params: Vec<Param>,
    pub return_type: FunctionTypes,
    pub is_async: bool,
    pub is_static: bool,
}

pub struct Param {
    pub name: String,
    pub types: FunctionTypes
}

pub enum FunctionTypes {
    Str,
    Int,
    Float,
    Bool,
    Vec,
    Object,
    Null,
    Any,
    Function,
    Enum,
    Unknown,
}