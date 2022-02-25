use axum::extract::{ContentLengthLimit, Multipart};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir, File};
use std::{
    env::var,
    fs::create_dir_all,
    io::{ErrorKind, Write},
    path::Path,
};
use uuid::Uuid;

use crate::res::Res;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileInfo {
    src: String,
    alt: String,
    is_image: bool,
}

pub async fn upload_file(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> Res<FileInfo> {
    let mut res = Res::<FileInfo>::default();

    if let Some(field) = multipart.next_field().await.unwrap() {
        // let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let upload_base_dir = var("UPLOAD_DIR").expect("获取 UPLOAD_DIR 错误");

        // 根据 文件类型和时间 生成 目录
        let now: DateTime<Local> = Local::now();

        let is_image = content_type.starts_with("image");

        let upload_dir = format!(
            "/{}/{}{:02?}/",
            if is_image { "images" } else { "files" },
            now.year(),
            now.month()
        );

        let path = Path::new(&upload_dir);
        println!("{}", upload_dir);

        // 目录不存在，就创建
        if !path.exists() {
            if let Err(e) = create_dir_all(&path) {
                log::error!("创建文件夹失败:{}", e);
                res.set_code(-1);
                res.set_msg("创建文件夹失败");
                return res;
            }
        }

        // 获取文件后缀
        let p = Path::new(&file_name);
        let ext = match p.extension() {
            Some(v) => v.to_str().unwrap_or_default(),
            None => {
                log::error!("获取文件后缀失败:{}", file_name);
                res.set_code(-1);
                res.set_msg("获取文件后缀失败");
                return res;
            }
        };

        // 文件相对路径，最终返回给用户的路径
        let mut file_path;
        // 完成的文件路径：保存文件的根目录 + 文件相对路径
        let mut all_file_path;

        // 防止文件名重复，一直生成新的文件名，直到该文件名不存在为止
        loop {
            file_path = path
                .join(format!("{}.{}", Uuid::new_v4(), ext))
                .to_str()
                .unwrap_or_default()
                .to_string();

            all_file_path = Path::new(&upload_base_dir).join(&file_path);

            // 文件不存在就不用循环了
            if !all_file_path.exists() {
                break;
            }
        }

        // 创建文件
        let mut file = match File::create(all_file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("写入文件失败:{}", e);
                res.set_code(-1);
                res.set_msg("写入文件失败");
                return res;
            }
        };

        // 上传内容写入到文件
        if let Err(e) = file.write(&data) {
            log::error!("写入文件失败:{}", e);
            res.set_code(-1);
            res.set_msg("写入文件失败");
            return res;
        };

        if let Err(e) = file.flush() {
            log::error!("刷新文件失败:{}", e);
            res.set_code(-1);
            res.set_msg("刷新文件失败");
            return res;
        };

        // 构造返回给前端  的内容
        let file_info = FileInfo {
            src: file_path,
            alt: file_name,
            is_image,
        };
        res.set_data(file_info);
    }

    res
}
