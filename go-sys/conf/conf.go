package conf

import (
	"gopkg.in/mgo.v2"
	"github.com/go-redis/redis/v8"
)

var DB *mgo.Database

var Client *redis.Client

func init() {
	// 初始化 mongodb
	session, err := mgo.Dial("127.0.0.1:27017")
	if err != nil {
		panic("mongodb 服务器连接错误")
	}
	DB = session.DB("test")

	// 初始化 redis

	Client = redis.NewClient(&redis.Options{
		Addr: "localhost:6397",
		Password: "",
		DB: 0,
	})
}
