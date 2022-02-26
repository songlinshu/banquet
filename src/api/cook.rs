use axum::{
    extract::{Extension, Path},
    Json,
};
use hyper::rt::Executor;
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::{jwt::Claims, res::Res};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCook {
    name: String,
    phone: String,
    sex: u8,
    marry_status: u8,
    origin_address: String,
    address: String,
    photo: String,
    identify_card1: String,
    identify_card2: String,
    residence_permit: String,
    description: String,
    foods: String,
}

pub async fn create_cook(
    claims: Claims,
    chef: Json<CreateCook>,
    Extension(pool): Extension<Pool<MySql>>,
) -> Res<u64> {
    let mut res = Res::default();

    // 从token中获取用户编号
    let user_id: i32 = claims.sub.parse().unwrap();

    // 添加厨师数据到数据库
    let id = match sqlx::query!(
        r#"insert into 
        chefs(
            user_id, name,phone,sex,marry_status, origin_address, address,
            photo, identify_card1, identify_card2, 
            residence_permit, description, foods
        ) 
        values(?, ?,?,?,?,?,?,?,?,?,?,?,?)"#,
        &user_id,
        &chef.name,
        &chef.phone,
        &chef.sex,
        &chef.marry_status,
        &chef.origin_address,
        &chef.address,
        &chef.photo,
        &chef.identify_card1,
        &chef.identify_card2,
        &chef.residence_permit,
        &chef.description,
        &chef.foods
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

    log::debug!("{:#?}", chef);

    res.set_data(id);
    res
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResultCook {
    id: i32,
    name: String,
    phone: String,
    sex: Option<i8>,
    marry_status: Option<i8>,
    origin_address: Option<String>,
    address: Option<String>,
    photo: Option<String>,
    identify_card1: String,
    identify_card2: String,
    residence_permit: Option<String>,
    description: Option<String>,
    foods: Option<String>,
}

/// 认证厨师列表
pub async fn list_cooks(Extension(pool): Extension<Pool<MySql>>) -> Res<Vec<ResultCook>> {
    let mut res = Res::default();
    let rs = match sqlx::query!("SELECT * FROM chefs").fetch_all(&pool).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("获取用户列表出错:{}", e);
            res.set_code(-1);
            res.set_msg("获取用户列表出错");
            return res;
        }
    };

    let mut users = vec![];
    for v in rs {
        users.push(ResultCook {
            id: v.id,
            name: v.name,
            phone: v.phone,
            sex: v.sex,
            marry_status: v.marry_status,
            origin_address: v.origin_address,
            address: v.address,
            photo: v.photo,
            identify_card1: v.identify_card1,
            identify_card2: v.identify_card2,
            residence_permit: v.residence_permit,
            description: v.description,
            foods: v.foods,
        });
    }

    res.set_data(users);

    res
}

/// 删除厨师认证信息
pub async fn delete_cook(Path(id): Path<i32>, Extension(pool): Extension<Pool<MySql>>) -> Res<u64> {
    let mut res = Res::default();
    log::debug!("id:{}", id);
    let rows_affected = match sqlx::query!("delete from chefs where id=?", id)
        .execute(&pool)
        .await
    {
        Ok(v) => v.rows_affected(),
        Err(e) => {
            log::error!("删除厨师出错:{}", e);
            res.set_code(-1);
            res.set_msg("删除厨师出错");
            return res;
        }
    };
    res.set_data(rows_affected);
    res
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SpareTime {
    start_time: String,
    end_time: String,
}

/// 设置厨师空闲时间
pub async fn set_spare_time(
    claims: Claims,
    spare_times: Json<Vec<SpareTime>>,
    Extension(pool): Extension<Pool<MySql>>,
) -> Res {
    let mut res = Res::default();

    let user_id: i32 = claims.sub.parse().unwrap();

    log::debug!("{:#?}", spare_times);

    // 添加前先删除当前厨师的所有空闲时间记录
    if let Err(e) = sqlx::query!("delete from spare_times where user_id=?", user_id)
        .execute(&pool)
        .await
    {
        log::error!("清空厨师空闲时间出错:{}", e);
        res.set_code(-1);
        res.set_msg("设置厨师空闲时间出错");
        return res;
    }

    // 把字符串格式的时间转换成分钟表示的整数，小时*60+分钟
    for t in spare_times.0 {
        log::debug!(
            "start:{:?}\tend:{:?}",
            str_to_minute(&t.start_time),
            str_to_minute(&t.end_time)
        );

        let start_time = match str_to_minute(&t.start_time) {
            Some(v) => v,
            None => {
                res.set_code(-4);
                res.set_msg("设置厨师空闲时间出错");
                return res;
            }
        };
        let end_time = match str_to_minute(&t.end_time) {
            Some(v) => v,
            None => {
                res.set_code(-6);
                res.set_msg("设置厨师空闲时间出错");
                return res;
            }
        };

        // 插入数据库
        if let Err(e) = sqlx::query!(
            "insert into spare_times(user_id, start_time, end_time) value(?,?,?)",
            user_id,
            start_time,
            end_time,
        )
        .execute(&pool)
        .await
        {
            log::error!("设置厨师空闲时间出错:{}", e);
            res.set_code(-8);
            res.set_msg("设置厨师空闲时间出错");
            return res;
        }
    }
    res
}

/// 时间转分钟数 8:30 => 510  转换规则 8*60+30
fn str_to_minute(time_str: &str) -> Option<i32> {
    let mut r = time_str.split(":");
    let hour: i32 = match r.next() {
        Some(v) => match v.parse() {
            Ok(v) => v,
            Err(e) => {
                log::error!("时间转换失败:{}", e);
                return None;
            }
        },
        None => {
            return None;
        }
    };

    let minute: i32 = match r.next() {
        Some(v) => match v.parse() {
            Ok(v) => v,
            Err(e) => {
                log::error!("时间转换失败:{}", e);
                return None;
            }
        },
        None => {
            return None;
        }
    };

    Some(hour * 60 + minute)
}
