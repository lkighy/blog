package login

import (
	"fmt"
	"go-sys/conf"
	"go-sys/service"
	"go-sys/utils"
	"gopkg.in/mgo.v2"
	"net"
	"time"

	"github.com/go-redis/redis/v8"
	"github.com/kataras/iris/v12"
)

// 发送验证码
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
		ctx.JSON(iris.Map{ "code": 500, "msg":  "系统错误"})
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
	title := "验证码"
	emailTemplate, err := service.FindOneEmailTemplateByEmail(data.email)
	if err == nil  {
		template = emailTemplate.Template
		title = emailTemplate.Title
	}
	// todo 2: 发送邮件
	if err = utils.SendEmail(data.email, title, fmt.Sprintf(template, ckm)); err != nil {
		ctx.JSON(iris.Map{"code": "505", "msg": "邮件发送失败，请联系管理员"})
		return
	}
	// todo 2: 将验证码记录至 redis 中, 并设置过期时间为 5 分钟
	err = conf.Client.Set(conf.Client.Context(), fmt.Sprintf("%s", ""), ckm, 5 * 60 * time.Second).Err()
	if err != nil {
		ctx.JSON(iris.Map{"code": "505", "msg": "系统错误"})
		return
	}
	// todo 3: 返回处理，表示已经发送
	ctx.JSON(iris.Map{ "code": 200, "msg": "邮件已发送，请耐心等待邮件发送到您的邮箱中",
	})
}

// 验证码登录
func CkmLogin(ctx iris.Context)  {

}
