# php.rs Rust 工具庫

一個模仿 PHP 標準函式庫的 Rust 工具庫，包含：

- 字串操作：`strlen`, `substr`, `strtoupper`, `strtolower`, `str_replace`, `trim`, `is_string_like`  
- JSON 處理：`json_encode`, `json_decode`, `json_format`, `json_format_utf8`  
- Base64 / Hex 編碼  
- 檔案操作：`file_get_contents`, `file_put_contents`, `is_file`, `is_dir`, `copy`, `rename`  
- SHA256  
- 時間：`time`, `date`  

---

## 開發環境

- Rust 1.78+
- VS Code

---

## VS Code 插件

- **rust-analyzer** → Rust 智慧補全、檢查、Go to Definition  
- **Dependi** → 方便管理 Cargo.toml 依賴  
- **Even Better TOML** → Cargo.toml 語法高亮與驗證  
- **CodeLLDB** → Rust 調試 (可選)  
- **Prettier - Code formatter** → 統一格式（可選）  

---

## VS Code 設定建議

- 字體放大：`Ctrl +`  
- Rust 使用 `rust-analyzer` 自動補全與格式化  
- 可以用 `Shift + Alt + F` 或 `rust-analyzer` 的自動格式化  
- 編譯 release：`cargo build --release`  

## 範例

mod php;

fn main() {
    // 字串測試
    let s = php::str_replace("Rust", "PHP", "Hello Rust!");
    println!("{}", s);

    // Base64 UTF-8
    let encoded = php::base64_encode_utf8("Hello PHP 🌟");
    println!("{}", encoded);
    let decoded = php::base64_decode_utf8(&encoded).unwrap();
    println!("{}", decoded);

    // is_file / is_dir
    println!("is_file: {}", php::is_file("Cargo.toml"));
    println!("is_dir: {}", php::is_dir("src"));
}