use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
// [dependencies]
// #chrono = "0.4"
use chrono::{NaiveDateTime, Local};
use serde::{Serialize, Deserialize};
use serde_json::{Value, to_string, to_string_pretty, from_str, from_slice, Deserializer, Error as SerdeError};
use std::fs;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

#[allow(dead_code)]
pub fn time() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_secs()  // 回傳秒數
}

#[allow(dead_code)]
pub fn substr(s: &str, start: isize, length: Option<usize>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len() as isize;
    let mut start = if start < 0 { len + start } else { start };
    start = start.max(0).min(len);

    let end = match length {
        Some(l) => (start + l as isize).min(len),
        None => len,
    };

    chars[start as usize..end as usize].iter().collect()
}

#[allow(dead_code)]
pub fn strlen(s: &str) -> usize {
    s.chars().count()
}

/// strtoupper
#[allow(dead_code)]
pub fn strtoupper(s: &str) -> String {
    s.to_uppercase()
}

/// strtolower
#[allow(dead_code)]
pub fn strtolower(s: &str) -> String {
    s.to_lowercase()
}

/// explode
#[allow(dead_code)]
pub fn explode(delimiter: &str, s: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

/// implode
#[allow(dead_code)]
pub fn implode(glue: &str, arr: &[String]) -> String {
    arr.join(glue)
}


/// PHP-like date(format, timestamp)
/// format: PHP 風格字串，例如 "Y-m-d H:i:s"
/// timestamp: UNIX timestamp，如果 None，使用現在時間
#[allow(dead_code)]
pub fn date(format: &str, timestamp: Option<u64>) -> String {
    // 轉成 chrono NaiveDateTime
    let dt = match timestamp {        
        Some(ts) => NaiveDateTime::from_timestamp(ts as i64, 0),
        None => {
            let now = Local::now();
            now.naive_local()
        }
    };

    // 將 PHP format 轉換成 strftime compatible
    // 部分常用 PHP 格式對應：
    // Y -> %Y, m -> %m, d -> %d, H -> %H, i -> %M, s -> %S
    let fmt = format
        .replace("Y", "%Y")
        .replace("m", "%m")
        .replace("d", "%d")
        .replace("H", "%H")
        .replace("i", "%M")
        .replace("s", "%S");

    dt.format(&fmt).to_string()
}

/// json_encode: 將任何可序列化物件轉成 JSON 字串
#[allow(dead_code)]
pub fn json_encode<T: Serialize>(data: &T) -> String {
    to_string(data).unwrap_or_else(|_| "null".to_string())
}

/// json_decode: 將 JSON 字串解析成 serde_json::Value
#[allow(dead_code)]
pub fn json_decode(s: &str) -> Option<Value> {
    let mut de = Deserializer::from_str(s);
    Value::deserialize(&mut de).ok()
}

/// json_format: 格式化 JSON 字串，漂亮輸出
#[allow(dead_code)]
pub fn json_format<T: Serialize>(data: &T) -> String {
    to_string_pretty(data).unwrap_or_else(|_| "null".to_string())
}

/// json_format_utf8: 格式化 JSON 並保持 Unicode 原樣（不 escape）
#[allow(dead_code)]
pub fn json_format_utf8<T: Serialize>(data: &T) -> String {
    match to_string_pretty(data) {
        Ok(s) => s,
        Err(_) => "null".to_string(),
    }
}

/// ---------------- Byte 版本 ----------------

/// base64_encode_bytes: 支援任意 bytes
#[allow(dead_code)]
pub fn base64_encode_bytes(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// base64_decode_bytes: 支援任意 bytes，回傳 Vec<u8>
#[allow(dead_code)]
pub fn base64_decode_bytes(data: &str) -> Option<Vec<u8>> {
    general_purpose::STANDARD.decode(data).ok()
}

/// Base64 encode UTF-8 字串
#[allow(dead_code)]
pub fn base64_encode_utf8(s: &str) -> String {
    general_purpose::STANDARD.encode(s.as_bytes())
}

/// Base64 decode UTF-8 字串
#[allow(dead_code)]
pub fn base64_decode_utf8(s: &str) -> Option<String> {
    general_purpose::STANDARD.decode(s).ok().and_then(|bytes| String::from_utf8(bytes).ok())
}

/// ---------------- UTF-8 版本 ----------------
/*
// 1️⃣ byte 版本
    let text = "Hello PHP 🌟";

    let encoded = php::base64_encode_utf8(text);
    println!("Base64 encode: {}", encoded);

    let decoded = php::base64_decode_utf8(&encoded).unwrap();
    println!("Base64 decode: {}", decoded);

    // bytes 版
    let data_bytes = text.as_bytes();
    let encoded_bytes = php::base64_encode_bytes(data_bytes);
    println!("Base64 bytes encode: {}", encoded_bytes);
*/

/// file_get_contents
#[allow(dead_code)]
pub fn file_get_contents(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// file_put_contents
#[allow(dead_code)]
pub fn file_put_contents(path: &str, content: &str) -> bool {
    fs::write(path, content).is_ok()
}

/// ---------------- bytes 版 ----------------
#[allow(dead_code)]
pub fn file_get_contents_bytes(path: &str) -> Option<Vec<u8>> {
    fs::read(path).ok()
}

#[allow(dead_code)]
pub fn file_put_contents_bytes(path: &str, data: &[u8]) -> bool {
    fs::write(path, data).is_ok()
}

/// sha256: 計算字串 SHA256
#[allow(dead_code)]
pub fn sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// sha256_file: 計算檔案 SHA256
#[allow(dead_code)]
pub fn sha256_file(path: &str) -> Option<String> {
    let data = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Some(hex::encode(result))
}

/// 判斷是否為檔案
#[allow(dead_code)]
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

/// 判斷是否為目錄
#[allow(dead_code)]
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// str_replace: 將所有搜尋字串替換成目標字串
#[allow(dead_code)]
pub fn str_replace(search: &str, replace: &str, subject: &str) -> String {
    subject.replace(search, replace)
}

/// is_string_like: 支援 % 通配符
#[allow(dead_code)]
pub fn is_string_like(subject: &str, pattern: &str) -> bool {
    if pattern == "%" {
        return true;
    }

    let parts: Vec<&str> = pattern.split('%').collect();

    if parts.len() == 1 {
        // 沒有 %，完全匹配
        return subject == pattern;
    }

    // 開頭是否有文字（pattern 沒有 % 開頭）
    let starts_with_wildcard = pattern.starts_with('%');
    let ends_with_wildcard = pattern.ends_with('%');

    let mut pos = 0;
    for (_i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        match subject[pos..].find(part) {
            Some(idx) => pos += idx + part.len(),
            None => return false,
        }
    }

    // 檢查開頭結尾是否符合 %
    if !starts_with_wildcard && !subject.starts_with(parts[0]) {
        return false;
    }
    if !ends_with_wildcard && !subject.ends_with(parts[parts.len()-1]) {
        return false;
    }

    true
}

/// trim: 去掉字串前後空白
#[allow(dead_code)]
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

/// copy 檔案
#[allow(dead_code)]
pub fn copy(src: &str, dst: &str) -> bool {
    fs::copy(src, dst).is_ok()
}

/// rename 檔案或目錄
#[allow(dead_code)]
pub fn rename(src: &str, dst: &str) -> bool {
    fs::rename(src, dst).is_ok()
}