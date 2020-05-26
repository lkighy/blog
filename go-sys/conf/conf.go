package conf

import (
	"fmt"
	"github.com/go-redis/redis/v8"
	"gopkg.in/mgo.v2"
	"gopkg.in/yaml.v2"
	"io/ioutil"
)

type Config struct {
	//mongodb
	MongoHost string // mongodb 地址
	MongoPort int // mongodb 端口号
	MongoDB string // 使用的数据库
	MongoAuth string // mongodb 账号
	MongoPawd string // mongodb 密码
	//redis
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
	// app
	AppName string
	AppAddr string
	JwtKey string
	JwtExp int
}

var DB *mgo.Database
var Session *mgo.Session

var Redis *redis.Client

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

var AppName string
var AppAddr string
var JwtKey string
var JwtExp int

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
	// mongodb
	MongoHost = conf.MongoHost
	MongoPort = conf.MongoPort
	MongoDB = conf.MongoDB
	MongoAuth = conf.MongoAuth
	MongoPawd = conf.MongoPawd
	// redis
	RedisHost = conf.RedisHost
	RedisPort = conf.RedisPort
	// stmp
	SmtpHost = conf.SmtpHost
	SmtpPort = conf.SmtpPort
	SmtpEmail = conf.SmtpEmail
	SmtpName = conf.SmtpName
	SmtpAuth = conf.SmtpAuth
	SmtpPawd = conf.SmtpPawd
	SmtpTest = conf.SmtpTest
	// app
	AppAddr = conf.AppAddr
	JwtKey = conf.JwtKey
	AppName = conf.AppName
	JwtExp = conf.JwtExp

	// 初始化 mongodb
	session, err := mgo.Dial(fmt.Sprintf("mongodb://%s:%d", MongoHost, MongoPort))

	if err != nil {
		panic("mongodb 服务器连接错误")
	}
	DB = session.DB(MongoDB)
	Session = session

	// 初始化 redis
	Redis = redis.NewClient(&redis.Options{
		//Addr: "localhost:6397",
		Addr: fmt.Sprintf("%s:%d", RedisHost, RedisPort),
		Password: "",
		DB: 0,
	})
}
