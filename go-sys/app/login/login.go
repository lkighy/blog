package login

import (
	"fmt"
	"go-sys/conf"
	"go-sys/service"
	"go-sys/utils"
	"gopkg.in/mgo.v2"
	"net"

	"github.com/go-redis/redis/v8"
	"github.com/kataras/iris/v12"
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

	// 判断该邮箱是否存在
	_, err := service.FindOneUserByEmail(data.email)
	if err == mgo.ErrNotFound {
		ctx.JSON(iris.Map{"code": 500, "msg": "邮箱不存在"})
	}

	// 获取 ip 地址
	addr := ctx.Request().RemoteAddr
	ip, _, err := net.SplitHostPort(addr)
	if err != nil {
		ctx.JSON(iris.Map{
			"code": 500,
			"msg":  "系统错误",
		})
		return
	}

	// 查询是否之前是否发送过邮箱
	_, err = conf.Client.Get(conf.Client.Context(), fmt.Sprintf("locksendcmk:%s:%s", data.email, ip)).Result()
	// 判断值存不存在，存在，且 err 不为 nil，则
	if err != redis.Nil && err == nil {
		ctx.JSON(iris.Map{"code": "505", "msg": "请一分钟后再发送验证码"})
		return
	}
	// 生成验证码
	ckm := utils.GenerateVerify()
	// todo 1: 生成发送的正文
	// 从 数据库中得到模板
	template := "%s" // 如果无法从数据库中获取模板，则使用默认模板
	emailTemplate, err := service.FindOneEmailTemplateByEmail(data.email)
	if err == mgo.ErrNotFound || err != nil  {
		template = emailTemplate.Template
	}
	// todo 2: 发送邮件
	utils.SendEmail(data.email, fmt.Sprintf(template, ckm))
	// todo 2: 将验证码记录至 redis 中
	// todo 3: 返回处理，表示已经发送

	// 从新定义一下，获取smtp 账号应该由自己线下定义才对，否则将有大问题

	ctx.JSON(iris.Map{
		"code": 200,
		"data": iris.Map{
			"ip":  ip,
			"cmk": ckm,
		},
	})
}
