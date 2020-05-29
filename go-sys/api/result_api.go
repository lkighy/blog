package api

import (
	"fmt"
	"github.com/kataras/iris/v12"
)

func apiOutput(ctx iris.Context, code int, msg string, data interface{} ) {
	ctx.JSON(iris.Map{"code": code, "msg": msg, "data": data})
}

// ok 200
func OK(ctx iris.Context, msg string, data interface{}) {
	apiOutput(ctx, 200, msg, data)
}

// 401
func Unauthorized(ctx iris.Context, msg string) {
	apiOutput(ctx, 401, msg, "")
}

func WaringFailUnauthorized(ctx iris.Context, msg string, err error) {
	apiOutput(ctx, 401, fmt.Sprintf("%s:%v", msg, err), "")
}

func ErrorFailUnauthorized(ctx iris.Context, msg string, err error) {
	apiOutput(ctx, 401, fmt.Sprintf("%s:%v", msg, err), "")
}

// error 500
func Fail(ctx iris.Context, msg string) {
	apiOutput(ctx, 500, msg, "")
}

func WaringFail(ctx iris.Context, msg string, err error) {
	apiOutput(ctx, 500, fmt.Sprintf("%s:%v", msg, err), "")
}

func ErrorFail(ctx iris.Context, msg string, err error) {
	apiOutput(ctx, 500, fmt.Sprintf("%s:%v", msg, err), "")
}

// 这些日志的 api 暂时不考虑
// 将值返回，同时记录一般信息
type InfoApi struct {}
// 将值返回，同时记录警告信息
type WaringApi struct {}
// 将值返回，同时记录严重bug
type ErrorApi struct {}

// 关于日志输出详情
// 通过配置文件控制，是否将错误信息返回给前端 debug 模式以及
// 所以其接受的值


func (e ErrorApi) OK() {}
func (e ErrorApi) Unauthorized() {}
//func (e ErrorApi) Error() {}
func (e ErrorApi) Error (ctx iris.Context, err error, msg string, data interface{}) {
	//ErrorLog.fmt(err) // 记录日志 [ERROR] 时间/访问ip/url/错误原因
	//if (conf.debug) { // 判定的是否在 debue
	//	msg = fmt.Sprintf("%s:%t", msg, err)
	//}
	//apiOutput(ctx, 500, msg, data)
}

