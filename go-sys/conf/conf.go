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
	MongoHost string `yaml:"mongo_host"`// mongodb 地址
	MongoPort int `yaml:"mongo_port"`// mongodb 端口号
	MongoDB string `yaml:"mongodb"`// 使用的数据库
	MongoAuth string `yaml:"mongo_auth"'`// mongodb 账号
	MongoPawd string `yaml:"mongo_pawd"`// mongodb 密码
	//redis
	RedisHost string `yaml:"redis_host"`//
	RedisPort int `yaml:"redis_port"`
	// smtp 相关
	SmtpHost string `yaml:"smtp_host"`
	SmtpPort int `yaml:"smtp_port"`
	SmtpName string `yaml:"smtp_name"`
	SmtpAuth string `yaml:"smtp_auth"`
	SmtpEmail string `yaml:"smtp_email"`
	SmtpPawd string `yaml:"smtp_pawd"`
	SmtpTest string `yaml:"smtp_test"`// smtp 使用的测试账号
	// app
	AppName string `yaml:"app_name"`
	AppAddr string `yaml:"app_addr"`
	JwtKey string `yaml:"jwt_key"`
	JwtExp int `yaml:"jwt_exp"`
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
	yml , err := ioutil.ReadFile("conf.yml")
	if err != nil {
		panic(fmt.Sprintln("读取配置失败, ERROR:", err))
	}
	conf := Config{}
	err = yaml.Unmarshal(yml, &conf)
	//fmt.Println(conf)
	if err != nil {
		panic(fmt.Sprintln("反序列化配失败，ERROR:", err))
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
		//fmt.Printf("mongodb://%s:%d", MongoHost, MongoPort)
		//fmt.Println("err: ", err)
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
