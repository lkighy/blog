package api

import (
	"github.com/dgrijalva/jwt-go"
	"go-sys/conf"
	"time"
)

func NewJwt(email, ip string) *jwt.Token {
	return jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"email": email,
		"iss": conf.AppName,
		"ip": ip,
		"exp":time.Now().Add(time.Hour * time.Duration(conf.JwtExp)).Unix(), // 过期时间
	})
}
