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
    res::Res, utils::check_admin,
};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CreateMenu {
    name: String,
    pic: String,
    price: i32,
    description: String,
    rank: Option<i32>,
}

pub async fn create_menu(
    claims: Claims,
    menu: Json<CreateMenu>,
    Extension(pool): Extension<Pool<MySql>>,
) -> Res<u64> {
    let mut res = Res::default();

    // 从token中获取用户编号
    let user_id: i32 = claims.sub.parse().unwrap();

    // 管理员才可以添加菜单
    let is_admin = check_admin(user_id, &pool).await;

    if !is_admin {
        res.set_code(-10);
        res.set_msg("只有管理员可以添加菜");
        return res;
    }

    log::debug!("{:#?}", menu);

    // 添加厨师数据到数据库
    let id = match sqlx::query!(
        r#"insert into 
        menus(
           name,pic, price, description, rank
        ) 
        values(?,?,?,?,?)"#,
        &menu.name,
        &menu.pic,
        &menu.price,
        &menu.description,
        &menu.rank
    )
    .execute(&pool)
    .await
    {
        Ok(v) => {
            // 插入后的用户编号
            v.last_insert_id()
        }
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                res.set_code(-1);
                res.set_msg("已经添加过厨师信息");
                return res;
            }

            log::error!("添加厨师数据出错:{}", e);
            res.set_code(-2);
            res.set_msg("服务端出错，请联系管理");
            return res;
        }
    };

    res.set_data(id);
    res
}
