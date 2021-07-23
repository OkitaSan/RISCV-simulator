pub mod parser;

pub mod registers;

pub mod instructions;

/// memory size
pub const MEMORY_SIZE:usize = 134217728;
/// memory(byte array)
pub static MEMORY:[u8;MEMORY_SIZE] = [0;MEMORY_SIZE];