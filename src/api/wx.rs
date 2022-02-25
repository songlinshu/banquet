use std::{collections::HashMap, env::var};

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
};

pub async fn wx_redirect() -> impl IntoResponse {
    // https://open.weixin.qq.com/connect/oauth2/authorize?appid=APPID&redirect_uri=REDIRECT_URI&response_type=code&scope=SCOPE&state=STATE#wechat_re
    let auth_url = format!("https://open.weixin.qq.com/connect/qrconnect?appid={}&redirect_uri={}&response_type=code&scope={}&state={}#wechat_redirect",
    var("WX_APPID").expect("获取 WX_APPID 错误"),
    urlencoding::encode(&var("WX_REDIRECT_URI").expect("获取 WX_REDIRECT_URI 错误")),
    "snsapi_userinfo",
    "STATE");
    Redirect::temporary(auth_url.to_string().parse().unwrap())
}

pub async fn wx_login(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    return "";
}
