# 博客服务器端

## 进度

- [x] redis 中间件
- [x] mongodb 中间件
- [x] smtp 发送邮件
- [x] 使用 .env 代替 config.json
- [ ] app 结构目录更改
- [ ] 将前台和后台的前端以及服务器端整合到一个项目中
- [ ] 登录
- [ ] 登录验证
- [ ] 文章添加
- [ ] 文章修改
- [ ] 文章删除

## 结构

### rust 库

- ~~rocket -- http 框架~~
- actix-web -- http 框架

### 数据库

- ~~postgreSQL - 关系数据库, 数据存储~~
- mongodb 决定用 mongodb 作为存储的主要数据库， 放飞自我
- redis - 非关系数据库, 缓存

#### mongodb 数据表以及结构

- 用户表
- 设置选项表
- 邮箱信息表 email_info
    - email 邮箱 唯一
    - password 
    - smtp_addr smtp服务器地址
    - port 端口号
    - from_name 发件人名称
- 文章表

### todo

#### 前台

- 数据库所有表格定义
- redis 以及其作用
- 陆游
- 