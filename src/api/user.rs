use axum::{
    extract::{Extension, Path},
    Json,
};
use chrono::Local;
use jsonwebtoken::{encode, Header};
use log::{debug, info};
use redis::{Client, Commands};
use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use std::{collections::HashMap, env::var};

use crate::{
    jwt::{create_token, AuthError, Claims, KEYS},
    res::Res,
};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PhoneLogin {
    phone: String,
    code: String,
}

pub async fn phone_login(
    login: Json<PhoneLogin>,
    Extension(mut client): Extension<Client>,
    Extension(pool): Extension<Pool<MySql>>,
) -> Res<String> {
    let mut res = Res::default();

    if login.phone.is_empty() || login.code.is_empty() {
        res.set_code(-1);
        res.set_msg("手机号和验证码不能为空");
        return res;
    }

    // 获取验证码
    let code: String = match client.get(format!("{}_login", &login.phone)) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("获取验证码失败，可能是超时了:{},手机号:{}", e, &login.phone);
            res.set_code(-2);
            res.set_msg("验证码超时，请重新获取");
            return res;
        }
    };

    // 比较验证码是否正确
    if !code.eq(&login.code) {
        log::debug!("验证码错误，手机号:{}", &login.phone);
        res.set_code(-3);
        res.set_msg("验证码错误");
        return res;
    }

    // 用户编号，用于生成 token
    let id;
    // 如果数据库用户表不存在该手机号，就添加
    match sqlx::query!("SELECT * FROM users WHERE phone = ?", &login.phone)
        .fetch_one(&pool)
        .await
    {
        Ok(v) => {
            // 查询到的用户编号
            id = v.id as u64;
        }
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    // 添加手机到数据库
                    match sqlx::query!("insert into users(phone) values(?)", &login.phone)
                        .execute(&pool)
                        .await
                    {
                        Ok(v) => {
                            // 插入后的用户编号
                            id = v.last_insert_id()
                        }
                        Err(e) => {
                            log::error!("添加用户到数据库出错:{}", e);
                            res.set_code(-5);
                            res.set_msg("服务端出错，请联系管理");
                            return res;
                        }
                    };
                }
                _ => {
                    log::error!("查询数据库出错:{:#?}", e);
                    res.set_code(-6);
                    res.set_msg("服务端出错，请联系管理");
                    return res;
                }
            }
        }
    };

    // 创建token
    let token = create_token(id.to_string().as_str()).unwrap();

    // 删除redis中的验证码
    client
        .del::<String, ()>(format!("{}_login", &login.phone))
        .unwrap();

    res.set_data(token);
    return res;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    phone: String,
}

/// 获取手机验证码
pub async fn get_phone_code(phone: Json<Phone>, Extension(mut client): Extension<Client>) -> Res {
    log::debug!("{:#?}", phone);

    let mut res = Res::default();

    // 如果 redis 中已存在验证码，就不能重发
    if let Ok(_) = client.get::<String, String>(format!("{}_login", &phone.phone)) {
        res.set_code(-2);
        res.set_msg("验证码已发送，无需重新发送");
        return res;
    }

    // 生成随机验证码
    let n: f32 = rand::random();
    let code = format!("{:06?}", (n * 1000000f32) as u32);

    let _: () = match client.set_ex(format!("{}_login", &phone.phone), &code, 300) {
        Ok(v) => v,
        Err(e) => {
            log::error!("发送登录短信出错:{},手机号:{}", e, &phone.phone);
            res.set_code(-1);
            res.set_msg("短信验证码发送出错,请联系管理员");
            return res;
        }
    };
    log::debug!("code:{}", code);

    // 发送短信验证码给用户手机
    let id = var("ACCESS_KEY_ID").unwrap();
    let secret = var("ACCESS_SECRET").unwrap();
    let sms_sign_name = var("SMS_SIGN_NAME").unwrap();
    let sms_code = var("LOGIN_SMS_CODE").unwrap();

    // let aliyun = Aliyun::new(&id, &secret);
    // if let Err(e) = aliyun
    //     .send_sms(&phone.phone, &sms_sign_name, &sms_code, &code)
    //     .await
    // {
    //     log::error!("发送登录短信出错:{},手机号:{}", e, &phone.phone);
    //     res.set_code(-2);
    //     res.set_msg("短信验证码发送出错,请联系管理员");
    //     return res;
    // }

    res.set_data(code);
    res
}

/**
*
* "phone": "13788883545",
   "is_auth": true, // 是否是认证厨师
   "pic": "1.jpg" // 头像相对地址
   "address": "地址",
   "is_cook": true, // 是否是厨师

   */

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResultMe {
    id: i32,
    phone: String,
    is_auth: bool,
    pic: Option<String>,
    address: Option<String>,
    is_cook: bool,
    is_admin: bool,
}
/// 获取自己的信息
pub async fn me(claims: Claims, Extension(pool): Extension<Pool<MySql>>) -> Res<ResultMe> {
    let mut res = Res::<ResultMe>::default();

    log::debug!("登录用户id:{}", &claims.sub);

    let id: u64 = claims.sub.parse().unwrap();
    match sqlx::query!("SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&pool)
        .await
    {
        Ok(v) => {
            log::debug!("{:#?}", v);
            let me = ResultMe {
                id: v.id,
                phone: v.phone,
                is_auth: if let Some(v) = v.is_auth {
                    v == 1
                } else {
                    false
                },
                pic: v.pic,
                address: v.address,
                is_cook: if let Some(v) = v.is_cook {
                    v == 1
                } else {
                    false
                },
                is_admin: if let Some(v) = v.is_admin {
                    v == 1
                } else {
                    false
                },
            };
            res.set_data(me);
            return res;
        }
        Err(e) => {
            log::error!("获取个人信息出错:{},id:{}", e, id);
            res.set_code(-5);
            res.set_msg("获取个人信息出错");
            return res;
        }
    }
    res
}
