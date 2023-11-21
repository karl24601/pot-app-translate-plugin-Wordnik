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

    fn parse_result(res: Value, client: Client) -> Option<Value> {
        let body = res.as_array()?.get(0)?.as_object()?;
        let (phonetics, meanings) = (body.get("phonetics")?, body.get("meanings")?);

        let (mut pronunciations, mut explanations, mut associations, mut sentence) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for item in phonetics.as_array()? {
            let audio_url = item.get("audio")?.as_str()?;
            let (region, voice);

            if audio_url.is_empty() {
                region = "";
                voice = vec![];
            } else {
                region = audio_url.get((audio_url.len() - 6)..(audio_url.len() - 4))?;
                voice = client.get(audio_url).send().ok()?.bytes().ok()?.to_vec();
            };

            let symbol = if let Some(text) = item.get("text") {
                text.as_str()?
            } else {
                ""
            };

            pronunciations.push(json!({
            "region": region,
            "symbol": symbol,
            "voice": voice,
            }));
        }

        for item in meanings.as_array()? {
            let _trait = item.get("partOfSpeech")?.as_str()?;
            let mut explains = Vec::new();
            let definitions = item.get("definitions")?.as_array()?;

            for definition in definitions {
                explains.push(definition.get("definition")?.as_str()?);

                if let Some(example) = definition.get("example") {
                    sentence.push(json!({
                    "source": example.as_str()?,
                    "target": "",
                    }));
                };
            }

            explanations.push(json!({
            "trait": _trait,
            "explains": explains,
            }));

            let synonyms = item
                .get("synonyms")?
                .as_array()?
                .into_iter()
                .map(|x| x.as_str().unwrap())
                .collect::<Vec<_>>();

            associations.extend(synonyms);
        }

        Some(json!({
            "pronunciations": pronunciations,
            "explanations": explanations,
            "associations": associations,
            "sentence": sentence,
        }))
    }

    if let Some(result) = parse_result(res, client) {
        return Ok(result);
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
