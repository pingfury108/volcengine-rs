# volcengine-rs

[火山引擎](https://www.volcengine.com) Rust语言SDK，实现了HTTP请求签名，可用于调用火山引擎API

## Usage

`cargo add volcengine-rs`

```rust
use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Text2ImgData {
    image_urls: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Text2Img {
    code: i64,
    message: String,
    data: Option<Text2ImgData>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key = "AK***";
    let secret_key = "***==";

    let region = "cn-north-1";
    let endpoint = "https://visual.volcengineapi.com";
    let service = "cv";
    let content_type = "application/json";
    let method = "POST";

    // use BTreeMap to ensure the parameter order
    let mut query_params = BTreeMap::new();
    query_params.insert("Action", "CVProcess");
    query_params.insert("Version", "2022-08-31");

    let payload = json!({
        "req_key": "high_aes_general_v20",
        "prompt": "一只可爱的小猫",
        "model_version": "general_v2.0",
        "seed":-1,
        "scale":3.5,
        "ddim_steps":16,
        "width":512,
        "height":512,
        "use_sr":true,
        "return_url":true,
    });

    let result = volcengine-rs::send_request::<Text2Img>(
        access_key,
        secret_key,
        endpoint,
        region,
        service,
        method,
        content_type,
        query_params,
        payload,
    )
    .await?;

    println!(
        "code:{} message:{} images:{:?}",
        result.code,
        result.message,
        result.data.unwrap().image_urls
    );

    Ok(())
}
```
