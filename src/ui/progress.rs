/// Phase 1: println!ベースのメッセージ表示
pub fn info(msg: &str) {
    println!("{}", msg);
}

pub fn success(msg: &str) {
    println!("✓ {}", msg);
}

pub fn warn(msg: &str) {
    println!("⚠ {}", msg);
}
