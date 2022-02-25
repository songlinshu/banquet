use crypto::{digest::Digest, sha1::Sha1};

/// 根据收到的数据，生成签名，判断是否来自微信服务器
pub fn get_signature(timestamp: &str, nonce: &str, token: &str) -> String {
    let mut data = vec![
        token.to_owned().clone(),
        timestamp.to_string(),
        nonce.to_owned(),
    ];
    data.sort();
    let data_str = data.join("");

    // sha1
    let mut hasher = Sha1::new();

    // write input message
    hasher.input_str(&data_str);

    // read hash digest
    hasher.result_str()
}
