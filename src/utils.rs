use sqlx::Pool;
use sqlx::MySql;


/// 根据uid判断是否是管理员
pub async fn check_admin(user_id: i32, pool: &Pool<MySql>) -> bool {
    match sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(pool)
        .await
    {
        Ok(v) => match v.is_admin {
            Some(v) => v != 0,
            None => false,
        },
        Err(e) => {
            log::error!("获取个人信息出错:{},id:{}", e, user_id);
            false
        }
    }
}
