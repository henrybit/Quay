use crate::error::{AppError, AppResult};

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn split_endpoints(raw: &str) -> Vec<String> {
    raw.split([',', '\n', ' ', ';'])
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

pub fn bytes_preview(bytes: Option<&[u8]>) -> String {
    match bytes {
        None => String::new(),
        Some(buf) if buf.is_empty() => String::new(),
        Some(buf) => match std::str::from_utf8(buf) {
            Ok(text) => text.to_string(),
            Err(_) => format!("hex:{}", to_hex(buf)),
        },
    }
}

pub fn to_hex(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(TABLE[(byte >> 4) as usize] as char);
        out.push(TABLE[(byte & 0x0f) as usize] as char);
    }
    out
}

pub fn require_non_empty(value: &str, label: &str) -> AppResult<()> {
    if value.trim().is_empty() {
        Err(AppError::msg(format!("{label} 不能为空")))
    } else {
        Ok(())
    }
}
