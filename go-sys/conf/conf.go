package conf

import (
	"github.com/go-redis/redis/v8"
	"gopkg.in/mgo.v2"
	"gopkg.in/yaml.v2"
	"io/ioutil"
)

type Config struct {
	MongoHost string // mongodb 地址
	MongoPort int // mongodb 端口号
	MongoDB string // 使用的数据库
	MongoAuth string // mongodb 账号
	MongoPawd string // mongodb 密码
	RedisHost string //
	RedisPort int
	// smtp 相关
	SmtpHost string
	SmtpPort int
	SmtpName string
	SmtpAuth string
	SmtpEmail string
	SmtpPawd string
	SmtpTest string // smtp 使用的测试账号
}

var DB *mgo.Database
var Session *mgo.Session

var Client *redis.Client

var MongoHost string // mongodb 地址
var MongoPort int // mongodb 地址
var MongoDB string // 使用的数据库
var MongoAuth string // mongodb 账号
var MongoPawd string // mongodb 密码
var RedisHost string //
var RedisPort int
var SmtpHost string
var SmtpPort int
var SmtpEmail string
var SmtpName string
var SmtpAuth string
var SmtpPawd string
var SmtpTest string // smtp 使用的测试账号

func init() {
	// 初始化配置信息
	yml , err := ioutil.ReadFile("./conf.yml")
	if err != nil {
		panic("读取配置失败")
	}
	conf := Config{}
	err = yaml.Unmarshal(yml, &conf)
	if err != nil {
		panic("反序列化配")
	}
	MongoHost = conf.MongoHost
	MongoPort = conf.MongoPort
	MongoDB = conf.MongoDB
	MongoAuth = conf.MongoAuth
	MongoPawd = conf.MongoPawd
	RedisHost = conf.RedisHost
	RedisPort = conf.RedisPort
	SmtpHost = conf.SmtpHost
	SmtpPort = conf.SmtpPort
	SmtpEmail = conf.SmtpEmail
	SmtpName = conf.SmtpName
	SmtpAuth = conf.SmtpAuth
	SmtpPawd = conf.SmtpPawd
	SmtpTest = conf.SmtpTest

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
