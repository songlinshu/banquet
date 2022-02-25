use axum::extract::{Extension, Path};
use chrono::Local;
use jsonwebtoken::{encode, Header};
use log::{debug, info};
use redis::{Client, Commands};
use reqwest;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env::var};

use crate::{
    jwt::{create_token, AuthError, Claims, KEYS},
    res::Res,
};

#[derive(Debug, Serialize, Deserialize)]
struct TokenRes {
    access_token: String,
    expires_in: i32,
}

pub async fn refresh_token(client: Client) -> Res {
    let url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        var("MP_APPID").expect("获取 MP_APPID 错误"),
        var("MP_SECRET").expect("获取 MP_SECRET 错误"),
    );

    debug!("请求地址:{}", url);

    let res = match reqwest::get(url).await {
        Ok(v) => v,
        Err(e) => {
            let mut res = Res::default();
            res.set_code(10000).set_msg(e.to_string().as_str());
            return res;
        }
    };
    let data = match res.json::<TokenRes>().await {
        Ok(v) => v,
        Err(e) => {
            let mut res = Res::default();
            res.set_code(10001).set_msg(e.to_string().as_str());
            return res;
        }
    };

    let mut conn = match client.get_connection() {
        Ok(v) => v,
        Err(e) => {
            let mut res = Res::default();
            res.set_code(10002).set_msg(e.to_string().as_str());
            return res;
        }
    };

    let _: () = match conn.set_ex("token", &data.access_token, data.expires_in as usize) {
        Ok(v) => v,
        Err(e) => {
            let mut res = Res::default();
            res.set_code(10003).set_msg(e.to_string().as_str());
            return res;
        }
    };

    return Res::default();
}

/// 创建临时二维码返回的信息
#[derive(Deserialize, Serialize, Debug, Default)]
struct QrCode {
    // 根据ticket可以获取到未失效的二维码
    ticket: String,
    // 过期时间
    expire_seconds: i32,
    // 根据这个url生成二维码
    url: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct QrLoginResult {
    // 过期时间
    expire_seconds: i32,
    // 根据这个url生成二维码
    url: String,
    // 登录状态场景码，用于返回给客户端查询扫码状态
    scene: String,
}

pub async fn phone_login(Extension(client): Extension<Client>) -> Res<String> {
    let mut res = Res::default();

    log::info!("登录");
    let token = create_token("aaa").unwrap();

    res.set_data(token);
    return res;
}

/// 生成关注登录的临时二维码
pub async fn create_wx_login_qrcode(Extension(client): Extension<Client>) -> Res<QrLoginResult> {
    // 生成临时登录场景字符串

    let id: usize = rand::random::<usize>() + 10000;

    let mut scene = format!("wx_login_{}", id);

    info!("scene:{}", scene);

    let mut res = Res::default();

    let mut conn = match client.get_connection() {
        Ok(v) => v,
        Err(e) => {
            res.set_code(10001).set_msg(e.to_string().as_str());
            return res;
        }
    };

    // 防止重复的场景码，如果存在了，就在末尾追加一个字符
    let ret: bool = match conn.exists(&scene) {
        Ok(v) => v,
        Err(e) => {
            res.set_code(10001).set_msg(e.to_string().as_str());
            return res;
        }
    };
    if ret {
        scene += "x";
    }

    // 保存scene,用于登录判断
    let _: () = conn.set_ex(&scene, false, 300).unwrap();

    // 获取token
    let token: String = match conn.get("token") {
        Ok(v) => v,
        Err(e) => {
            res.set_code(10001).set_msg(e.to_string().as_str());
            return res;
        }
    };

    let qrcode = match gen_qrcode(&token, scene.as_str(), 60 * 5).await {
        Ok(v) => v,
        Err(e) => {
            res.set_code(10001).set_msg(e.to_string().as_str());
            return res;
        }
    };

    let qr_result = QrLoginResult {
        expire_seconds: qrcode.expire_seconds,
        url: qrcode.url,
        scene: scene.to_string(),
    };
    res.set_data(qr_result);
    return res;
}

/// 生成带场景参数的临时二维码
/// token: 请求微信接口必须的参数
/// scene: 场景参数，用来区分不同的人
/// timeout: 二维码过期时间，单位是秒
async fn gen_qrcode(token: &str, scene: &str, timeout: usize) -> Result<QrCode, reqwest::Error> {
    // 生成二维码
    let url = format!(
        "https://api.weixin.qq.com/cgi-bin/qrcode/create?access_token={}",
        token
    );

    let client = reqwest::Client::new();
    let res = client.post(url)
        .header("content-type", "application/json")
        .body(
            format!(r#"{{"expire_seconds": {}, "action_name": "QR_STR_SCENE", "action_info": {{"scene": {{"scene_str": "{}"}}}}}}"#,
            timeout,
            scene
    ))
        .send().await.unwrap();

    return res.json::<QrCode>().await;
}
