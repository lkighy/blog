package login

import (
	"fmt"
	"github.com/go-redis/redis/v8"
	"github.com/kataras/iris/v12"
	"go-sys/conf"
	"go-sys/utils"
	"net"
)

func SendCkm(ctx iris.Context) {
	// 获取 ip 地址
	var data struct {
		email string `json:"email" validate:"required"`
	}

	if err := ctx.ReadJSON(&data); err != nil {
		ctx.JSON(iris.Map{"code": "500", "msg": "缺少参数"})
		return
	}

	addr := ctx.Request().RemoteAddr
	//net.Ip
	//net.LookupIP()
	//ip := strings.Split(addr, ":")
	ip, _, err := net.SplitHostPort(addr)
	if err != nil {
		ctx.JSON(iris.Map{
			"code": 500,
			"msg": "系统错误",
		})
		return
	}
	// 检测
	_, err = conf.Client.Get(conf.Client.Context(),  fmt.Sprintf("locksendcmk:%s:%s", data.email, ip)).Result()
	// 判断值存不存在，存在，且 err 不为 nil，则
	if err != redis.Nil && err == nil {
		ctx.JSON(iris.Map{"code": "505", "msg": "请一分钟后再发送验证码"})
		return
	}
	// 生成验证码
	ckm := utils.GenerateVerify()
	// todo 1: 记录至 redis 中
	// todo 2: 发送邮件
	// todo 3: 返回结构，表示已经发送

	//ctx.HTML("IP: %s,", addr)
	ctx.JSON(iris.Map{
		"code": 200,
		"data": iris.Map{
			"ip": ip,
			"cmk": ckm,
		},
	})
}
