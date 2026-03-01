use crate::core::red_hare::RedHare;

impl RedHare {
    pub fn h_set_string(&self, k: String, field: String, v: Vec<u8>) {}

    pub fn h_set_string_with_expire(&self, k: String, field: String, v: Vec<u8>) {}

    pub fn h_get_string(&self, k: String, field: String) {}
    pub fn h_get_all_string(&self, k: String) {}
}