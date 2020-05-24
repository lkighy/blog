package conf

import (
	"github.com/go-redis/redis/v8"
	"gopkg.in/mgo.v2"
)

type Config struct {
	MongoHost string // mongodb 地址
	MongoPort string // mongodb 端口号
	MongoDB string // 使用的数据库
	MongoAuth string // mongodb 账号
	MongoPawd string // mongodb 密码
	RedisHost string //
	RedisPort string
	// smtp 相关
	SmtpHost string
	SmtpPort string
	SmtpAuth string
	SmtpPawd string
	SmtpTest string // smtp 使用的测试账号
}

var DB *mgo.Database
var Session *mgo.Session

var Client *redis.Client

var MongoAddr string // mongodb 地址
var MongoDB string // 使用的数据库
var MongoAuth string // mongodb 账号
var MongoPawd string // mongodb 密码
var RedisAddr string //
var SmtpHost string
var SmtpPort string
var SmtpEmail string
var SmtpName string
var SmtpAuth string
var SmtpPawd string
var SmtpTest string // smtp 使用的测试账号

func init() {
	// 初始化配置信息

	// 初始化 mongodb
	session, err := mgo.Dial("mongodb://127.0.0.1:27017")
	if err != nil {
		panic("mongodb 服务器连接错误")
	}
	DB = session.DB("test")
	Session = session

	// 初始化 redis
	Client = redis.NewClient(&redis.Options{
		Addr: "localhost:6397",
		Password: "",
		DB: 0,
	})
}
