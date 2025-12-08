// 告訴 Rust 專案有一個模組叫 php
mod php;

fn main() {
    let ts = php::time();
    let mut a = String::from("WTFFFFFF....你好");
    a.push_str("!!!");
    println!("Hello, world! {} {}", a, ts);

    println!("Ymd = {}", php::date("Ymd", None));
    println!("Y-m-d H:i:s = {}", php::date("Y-m-d H:i:s", None));
}
