pub fn build_address(target: &str, port: u16) -> String {
    format!("{}:{}", target, port)
}