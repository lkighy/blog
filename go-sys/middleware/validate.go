package middleware

import (
	"github.com/dgrijalva/jwt-go"
	jwtmiddleware "github.com/iris-contrib/middleware/jwt"
	"github.com/kataras/iris/v12/context"
	"go-sys/api"
	"go-sys/conf"
)

// 校验验证 jwt
func JWTMiddleware() *jwtmiddleware.Middleware {
	return jwtmiddleware.New(jwtmiddleware.Config{
		ValidationKeyGetter: func(token *jwt.Token) (interface{}, error) {
			return []byte(conf.JwtKey), nil
		},
		SigningMethod: jwt.SigningMethodHS256,
		ErrorHandler: func(ctx context.Context, err error) {
			api.Unauthorized(ctx, "用户未认证", err)
			//ctx.JSON(iris.Map{"code": 401, "msg": "Unauthorized", "data": err})
		},
	})
}

//
