package middleware

import (
	"github.com/dgrijalva/jwt-go"
	jwtmiddleware "github.com/iris-contrib/middleware/jwt"
	"go-sys/conf"
)

// 校验验证 jwt
func JWTMiddleware() *jwtmiddleware.Middleware {
	return jwtmiddleware.New(jwtmiddleware.Config{
		ValidationKeyGetter: func(token *jwt.Token) (interface{}, error) {
			return []byte(conf.JwtKey), nil
		},
		SigningMethod: jwt.SigningMethodES256,
	})
}

//
