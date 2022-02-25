- [业务流程](#----)
  * [小程序端](#----)
      - [用户登录](#----)
      - [点餐下单](#----)
      - [定制预约](#----)
      - [厨师认证](#----)
      - [设置空闲时间](#------)
  * [后台](#--)
      - [厨师认证通过](#------)
      - [用户列表](#----)
      - [根据时间查询空闲厨师](#----------)
      - [套餐增删改查](#------)
      - [订单列表和编辑订单状态](#-----------)
      - [查看私人定制预约列表](#----------)
      - [编辑私人定制预约状态和处理结果](#---------------)
- [接口](#--)
  * [用户](#--)
    + [获取验证码](#-----)
    + [手机验证码登录](#-------)
    + [个人信息](#----)
    + [用户列表](#-----1)
    + [添加厨师（小程序端认证厨师）](#--------------)
    + [厨师列表](#----)
    + [删除厨师信息](#------)
    + [编辑(添加)空闲时间](#----------)
      - [查看指定时间空闲的厨师](#-----------)
  * [菜单](#--)
    + [添加套餐](#----)
    + [套餐列表](#----)
    + [编辑套餐](#----)
    + [删除套餐](#----)
  * [订单](#--)
    + [添加订单](#----)
    + [订单分类列表](#------)
    + [编辑订单](#----)
    + [删除订单](#----)
  * [预约](#--)
    + [私人定制预约](#------)
    + [预约列表](#----)
    + [管理员处理预约](#-------)
    
# 业务流程

## 小程序端

#### 用户登录

1. 输入手机号，点击获取验证码
2. 后端发送验证码到用户手机
3. 用户提交手机号+验证码到后端
4. 后端判断验证码正确就返回 token，并保存手机号到用户表（如果存在则忽略）
5. 错误则返回错误信息

#### 点餐下单

1. 前端首页点击套餐列表，进入具体套餐页面
2. 获取用户地址和手机号，输入姓名,(此时顺便保存姓名到用户表中)
3. 前端向后端发送请求创建订单
4. 后端返回下单情况，并通知管理员

#### 定制预约

1. 点击首页 私人定制
2. 跳转到表单页面填写个人联系方式，提交后端
3. 提示等待官方服务人员联系

#### 厨师认证

1. 输入基本信息，上传图片等
2. 提交保存
3. 返回处理结果

#### 设置空闲时间

1. 在 我的 页面 设置空闲时间

## 后台

#### 厨师认证通过

#### 用户列表

#### 根据时间查询空闲厨师

#### 套餐增删改查

#### 订单列表和编辑订单状态

#### 查看私人定制预约列表

#### 编辑私人定制预约状态和处理结果

# 接口

- 接口返回的格式统一为下面的格式，请求正常，状态码统一为 200。 如果请求需要认证的接口，可能会返回 400 或 401 错误，直接跳转到登录界面重新登录即可

```json
{
  "code": 0, // 非0表示出错
  "msg": "success", // code 非 0 时，此处包含具体错误信息，比如字段不合法，缺少字段，服务端处理出错等
  "data": "" // 没出错，就会包含具体的数据,可以是任意类型数据，视具体接口而定
}
```

## 用户

### 获取验证码

- Request
  `POST /login/phone/code`

  ```json
  {
    "phone": "13788888888"
  }
  ```

- Response

```json
{
  "code": 0, // 0 表示发送成功，失败请直接把msg字段的值提示给用户
  "msg": "success",
  "data": null
}
```

### 手机验证码登录

- Request
  `POST /login/phone`

  ```json
  {
    "phone": "13788888888",
    "code": "123456" // 手机收到的验证码
  }
  ```

- Response

```json
{
  "code": 0,
  "msg": "success",
  "data": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhYWEiLCJjb21wYW55Ijoibm94dWUiLCJleHAiOjE2NDYzOTgxMjZ9.v1Lo8blmD-zIzt7seQhmx2uxyNO-H9M6_uZLN3LVo3A"
}
```

- **请求需要认证的接口，通过 bearer 认证方式把 token 带在头信息中即可**

### 个人信息

- Request
  `GET /users/me`

- Response

```json
{
  "code": 0,
  "msg": "success",
  "data": {
    "phone": "13788883545",
    "is_auth": true, // 是否是认证厨师
    "pic": "1.jpg" // 头像相对地址
    "address": "地址",
    "is_cook": true, // 是否是厨师
  }
}
```

### 用户列表

- Request  
  `GET /users`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个用户信息
      {
        "id": 1,
        "nickname": "姓名",
        "phone": "13788883545",
        "is_admin": false, // 是否是管理员
        "is_auth": true, // 是否认证厨师
        "pic": "1.jpg" // 头像相对地址
        "address": "地址",
        "is_cook": true, // 是否是厨师
      }
    ]
  }
  ```

### 添加厨师（小程序端认证厨师）

- Request
  `POST /cooks`

  ```json
  {
    "name": "姓名",
    "phone": "手机号", // 注：前端界面上为了提高体验，可以自动把个人信息中的手机号填进去
    "sex": 1, //数字类型，可选，默认为 0 未知， 1 男，2 女
    "marry_status": 1, // 数字类型，可选， 婚姻状态：0 未知，1 未婚，2 已婚
    "origin_address": "户籍所在地(可选)",
    "address": "常驻地址(可选)",
    "photo": "真人照片", // 上传文件后返回的相对路径
    "identify_card1": "身份证正面", // 上传文件后返回的相对路径
    "identify_card2": "身份证反面", // 上传文件后返回的相对路径
    "residence_permit": "居住证", // 上传文件后返回的相对路径
    "description": "履历信息", // 可选，默认为空
    "foods": "[{'pic':'1.png'},{'pic':'2.png'}]" // 可选，厨师擅长的食物列表，组装成json数组，如果没有，传 null
  }
  ```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": null
  }
  ```

### 厨师列表

- Request  
  `GET /cooks`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个厨师信息
      {
        "id": 1, // 厨师信息编号
        "user_id": 2, // 用户编号
        "name": "姓名",
        "sex": 1,
        "photo": "1.jpg",
        "description": "四川xxx酒店大厨",
        "foods": "[{'pic':'1.png'},{'pic':'2.png'}]"
      }
    ]
  }
  ```

### 删除厨师信息

- Request  
  `DELETE /cooks/:id`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": null
  }
  ```

### 编辑(添加)空闲时间

- Request
  `POST /cooks/:id/spare_time` // 编辑 id 所指定的厨师空闲时间

  ```json
  [
    // 可以包含多段时间，时间使用 24 小时制
    {
      "start_time": "8:20", // 时间格式只能是 时:分
      "end_time": "12:30"
    },
    {
      "start_time": "13:20",
      "end_time": "15:30"
    }
  ]
  ```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": null
  }
  ```

#### 查看指定时间空闲的厨师

- Request  
  `GET /cooks/spare_time?time=11:12`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个厨师信息
      {
        "id": 1, // 厨师信息编号
        "user_id": 2, // 用户编号
        "name": "姓名",
        "sex": 1,
        "photo": "1.jpg",
        "description": "四川xxx酒店大厨",
        "foods": "[{'pic':'1.png'},{'pic':'2.png'}]",
        "spare_times": [
          // 一个或多个时间段
          {
            "start_time": 100, // 从0点开始所经过的分钟数
            "end_time": 200 // 从0点开始所经过的分钟数
          }
        ]
      }
    ]
  }
  ```

## 菜单

### 添加套餐

- Request
  `POST /menus`

```json
{
  "name": "套餐名",
  "pic": "图片",
  "price": 300, // 价格，单位：角
  "description": "套餐详情页",
  "rank": 0 // 套餐排序，越大越靠前
}
```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1 // 返回添加后的套餐编号
  }
  ```

### 套餐列表

- Request  
  `GET /menus`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个菜单信息
      {
        "id": 1, // 套餐编号
        "name": "套餐名",
        "pic": "图片",
        "price": 300, // 价格，单位：角
        "rank": 0
      }
    ]
  }
  ```

### 编辑套餐

- Request
  `POST /menus/:id`

```json
{
  "name": "套餐名",
  "pic": "图片",
  "price": 300, // 价格，单位：角
  "description": "套餐详情",
  "rank": 1 // 优先级，越大越靠前
}
```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1 // 编辑的套餐编号
  }
  ```

### 删除套餐

- Request
  `DELETE /menus/:id`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": null
  }
  ```

## 订单

### 添加订单

- Request
  `POST /orders`

```json
{
  "menu_id": 1, // 套餐编号
  "address": "地址",
  "phone": "手机号"
}
```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1 // 返回订单id
  }
  ```

### 订单分类列表

- Request  
  `GET /orders`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个菜单信息
      {
        "id": 1, // 订单编号
        "name": "套餐名",
        "pic": "图片",
        "price": 300, // 价格，单位：角
        "rank": 0
      }
    ]
  }
  ```

### 编辑订单

    修改订单不同状态，管理员才可以修改成接单和完成状态

    订单状态，-1， 已取消，0 等待下单相当于在购物车，1 已下单，2 已接单，10 已完成

    支持同时修改多个，通过逗号分隔订单编号

- Request
  `POST /orders/:id/status

  ```json
  {
    "status": 1 //可以是上面的任意状态
  }
  ```

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1 // 返回订单id
  }
  ```

### 删除订单

- Request
  `DELETE /orders/:id

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": null
  }
  ```

## 预约

### 私人定制预约

- Request

  `POST /reservations`

  ```json
  {
    "phone": "手机号",
    "name": "姓名",
    "address": "地址",
    "datetime": "就餐日期",
    "number_of_people": 15 // 就餐人数
  }
  ```

- Response

  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1 // 返回订单id
  }
  ```

### 预约列表

- Request  
  `GET /reservations`

- Response
  ```json
  {
    "code": 0,
    "msg": "success",
    "data": [
      // 包含多个预约信息
      {
        "id": 1,
        "phone": "手机号",
        "name": "姓名",
        "address": "地址",
        "datetime": "就餐日期",
        "number_of_people": 15, // 就餐人数
        "status": 1, // 预约状态，0 未处理，已处理
        "msg": "处理记录信息"
      }
    ]
  }
  ```

### 管理员处理预约

- Request

  `POST /reservations/:id`

  ```json
  {
    "status": 1,
    "msg": "处理记录信息"
  }
  ```

- Response

  ```json
  {
    "code": 0,
    "msg": "success",
    "data": 1
  }
  ```
