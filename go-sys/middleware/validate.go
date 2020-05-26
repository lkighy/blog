package middleware

import "github.com/kataras/iris"

// 登录验证中间件
// 没有登录
var NotLogin = iris.Map{"code": 305, "msg": "请先登录"}

// 登录过期
var ExceedLogin = iris.Map{"code": 306, "msg": "登录过期"}

// 500
var ServerError = iris.Map{"code": 500, "msg": "服务器错误"}

// 504
var ValidateError = iris.Map{"code": 500, "msg": "验证码错误"}

// 05


