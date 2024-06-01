use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let api_key = needs.get("apiKey").ok_or("sk-m58ayNrRswFSeFOgdQVjjz3TlsbxHsn6ogsYSehFtbNcTef6")?;
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let url = "https://api.moonshot.cn";

    let request_body = json!({
        "prompt": format!("Translate the following text from {} to {}: {}", from, to, text),
        "max_tokens": 60
    });

    let res: Value = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        res.get("choices")?
            .get(0)?
            .get("text")?
            .as_str()
            .map(|s| s.trim().to_string())
    }

    if let Some(result) = parse_result(res) {
        return Ok(Value::String(result));
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("apiKey".to_string(), "your_openai_api_key".to_string());
        let result = translate("你好 世界！", "zh", "en", "zh_cn", needs).unwrap();
        println!("{result}");
    }
}
