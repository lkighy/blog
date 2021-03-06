package app

import (
	"github.com/kataras/iris/v12"
	"github.com/kataras/iris/v12/middleware/logger"
	"github.com/kataras/iris/v12/middleware/recover"
	"github.com/kataras/iris/v12/mvc"
	"go-sys/app/admin"
	"go-sys/app/login"
)

func Router() *iris.Application {
	app := iris.New()
	app.Use(recover.New())
	app.Use(logger.New())
	app.Handle("POST", "/send-ckm", login.SendCkm)
	app.Handle("POST", "/ckm-login", login.CkmLogin)
	mvc.Configure(app.Party("/admin"), admin.Manage)
	//app.Handle("GET", "/test", home.Test)
	return app
}