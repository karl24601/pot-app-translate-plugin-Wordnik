use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use urlencoding::encode;

#[no_mangle]
pub fn translate(
    text: &str, // 待翻译文本
    from: &str, // 源语言
    to: &str,   // 目标语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    detect: &str, // 检测到的语言 (若使用 detect, 需要手动转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    let res: Value = client
        .get(format!("https://api.wordnik.com/v4/words.json/{word}?api_key=sy04qr76gpejdxv3dxfyqvfgrmjwja3jygi59vjdcmmdp5z5g"))
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<String> {
        let result = res.as_object()?.get("translation")?.as_str()?.to_string();

        Some(result.replace("@@", "/"))
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
        println!("{result}");
    }
}
