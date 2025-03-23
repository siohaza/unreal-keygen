use crate::ut2k_common;

pub fn generate_key() -> String {
    ut2k_common::generate_key_with_magic("This is going to change")
}
