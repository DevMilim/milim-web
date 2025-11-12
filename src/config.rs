#[derive(Debug, Clone, Default)]
pub struct Config {
    pub max_headers: u32,
    pub max_header_len_kb: usize,
    pub max_body_kb: usize,
    pub read_timeout_s: u32,
    pub keep_alive_s: u32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_headers: 32,
            max_header_len_kb: 8,
            max_body_kb: 1024,
            read_timeout_s: 5,
            keep_alive_s: 5,
        }
    }
    pub fn get_kb_value(value_kb: usize) -> usize {
        value_kb * 1024
    }
}
