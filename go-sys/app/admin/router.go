package admin

import (
	"fmt"
	"github.com/dgrijalva/jwt-go"
	"github.com/kataras/iris"
	"github.com/kataras/iris/v12/mvc"
	"go-sys/middleware"
)

func Manage(app *mvc.Application) {
	// 使用中间件
	app.Router.Use(middleware.JWTMiddleware().Serve) // 绑定 jwt
	//app.Party("/article",)
	app.Handle(new(ArticleMVC)) // 路由 文章管理
}

//AfterActivation

//struct
type ArticleMVC struct {
	Token *jwt.Token // token
	//Ctx iris.Context
}

func (a *ArticleMVC) BeforeActivation(b mvc.BeforeActivation) {
	b.Handle("POST", "/add", "Add") // 设置路由
}

// 默认 /
func (a *ArticleMVC) Post() iris.Map {
	fmt.Println("token: ", a.Token)
	//userMsg :=a.Ctx.Values().Get("jwt").(*jwt.Token).Claims.(jwt.MapClaims)
	return iris.Map{"data": "OK"}
}

func (a *ArticleMVC) Add() iris.Map {
	// 取出 token
	//token := a.Ctx.Values().Get("jwt").(*jwt.Token)
	fmt.Println("token: ", a.Token)

	return iris.Map{"data": "OK"}
}
