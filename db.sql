create table users(
    id int auto_increment primary key,
    phone varchar(20) not null comment "手机号",
    nickname varchar(20) null default '' comment "昵称",
    is_auth tinyint null default 0 comment "是否认证为厨师",
    is_admin tinyint null default 0 comment "是否是管理员",
    `password` varchar(100) null default '' comment "是否密码登录，如果为空不允许密码登录",
    pic varchar(200) null default '' comment "头像，可空",
    address varchar(500) null default "" comment "地址",
    is_cook tinyint null default 0 comment "是否是厨师，0 不是， 1 是",
    created_at timestamp default CURRENT_TIMESTAMP() comment "添加时间"
) ENGINE = InnoDB DEFAULT CHARSET = utf8 AUTO_INCREMENT = 1;

# 厨师信息
create table chefs(
    id int auto_increment primary key,
    user_id int not null comment "users.id 外键",
    rank int null default 0 comment "优先级,默认为0，越大越靠前",
    name varchar(10) not null comment "姓名",
    phone varchar(20) not null comment "手机号",
    sex tinyint null default 0 comment "0 未知， 1 男，2 女",
    marry_status tinyint null default 0 comment "婚姻状态：0 未知，1 未婚，2 已婚",
    origin_address varchar(200) null default "" comment "户籍所在地",
    address varchar(200) null default "" comment "常驻地址",
    photo varchar(200) null default "" comment "真人照片",
    identify_card1 varchar(200) not null comment "身份证正面",
    identify_card2 varchar(200) not null comment "身份证反面",
    residence_permit varchar(200) null default "" comment "居住证",
    description varchar(255) null default "" comment "履历，描述",
    foods varchar(500) null default "" comment "食物列表，json数组,例如： [{'pic':'1.png'},{'pic':'2.png'}]",
    created_at timestamp default CURRENT_TIMESTAMP() comment "添加时间",
    foreign key(user_id) references users(id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 AUTO_INCREMENT = 1;

# 空闲时间段
create table spare_times(
    user_id int not null comment "users.id 外键",
    start_time int not null comment "开始有空的时间，存储方式为0点开始所经历的分钟数",
    end_time int not null comment "有空结束的时间，存储方式为0点开始所经历的分钟数",
) ENGINE = InnoDB DEFAULT CHARSET = utf8 AUTO_INCREMENT = 1;

# 套餐
create table menus(
    id int auto_increment primary key,
    name varchar(50) not null comment "套餐名称",
    pic varchar(200) not null comment "套餐封面",
    price int not null comment "套餐价格，单位:角",
    description text not null comment "详情页信息",
    rank int null default 0 comment "优先级,默认为0，越大越靠前",
    created_at timestamp default CURRENT_TIMESTAMP() comment "添加时间"
) ENGINE = InnoDB DEFAULT CHARSET = utf8 AUTO_INCREMENT = 1;

# 订单信息
create table orders(
    id int auto_increment primary key,
    menu_id int not null comment "菜品编号",
    name varchar(100) not null comment "套餐名称",
    price int not null comment "套餐价格，单位:角",
    pic varchar(200) not null comment "套餐封面",
    count int null default 1 comment "份数默认1",
    status tinyint null default 0 comment "订单状态，-1， 已取消，0 已下单，1 已接单，10 已完成",
    address varchar(255) not null comment "地址",
    phone varcahr(20) not null comment "电话",
    created_at timestamp default CURRENT_TIMESTAMP() comment "添加时间",
    foreign key(menu_id) references menus(id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8 AUTO_INCREMENT = 1;