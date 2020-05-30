package login

import (
	"fmt"
	"go-sys/api"
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
		Email string `json:"email" validate:"required"`
	}
	if err := ctx.ReadJSON(&data); err != nil {
		//ctx.JSON(iris.Map{"code": 505, "msg": "缺少参数"})
		api.Fail(ctx, "缺少参数", err)
		return
	}

	// 判断该邮箱是否存在
	_, err := service.FindOneUserByEmail(data.Email)
	fmt.Println("user: ", data.Email)
	if err == mgo.ErrNotFound {
		//ctx.JSON(iris.Map{"code": 505, "msg": "邮箱不存在"})
		api.Fail(ctx, "邮箱不存在", err)
		return
	}

	// 获取 ip 地址
	addr := ctx.Request().RemoteAddr
	ip, _, err := net.SplitHostPort(addr)
	if err != nil {
		//ctx.JSON(iris.Map{ "code": 500, "msg":  "系统错误"})
		api.Fail(ctx, "系统错误", err)
		return
	}
	// 查询是否之前是否发送过邮箱
	_, err = conf.Redis.Get(conf.Redis.Context(), fmt.Sprintf("lockcmk:%s:%s", data.Email, ip)).Result()
	// 判断值存不存在，存在，且 err 不为 nil，则
	if err != redis.Nil && err == nil {
		//ctx.JSON(iris.Map{"code": 505, "msg": "请一分钟后再发送验证码"})
		api.Fail(ctx, "请一分钟后再发送验证码", err)
		return
	}
	// 生成验证码
	ckm := utils.GenerateVerify()

	// 测试中，暂时注释掉
	// todo 1: 生成发送的正文
	// 从 数据库中得到模板
	//template := "%s" // 如果无法从数据库中获取模板，则使用默认模板
	//title := "验证码"
	//emailTemplate, err := service.FindOneEmailTemplateByEmail(data.Email)
	//if err == nil && err != mgo.ErrNotFound  {
	//	template = emailTemplate.Template
	//	title = emailTemplate.Title
	//}
	//// todo 2: 发送邮件
	//if err = utils.SendEmail(data.Email, title, fmt.Sprintf(template, ckm)); err != nil {
	//	fmt.Println("发送邮件：", err)
	//	//ctx.JSON(iris.Map{"code": 500, "msg": "邮件发送失败，请联系管理员"})
	//	api.ErrorFail(ctx, "邮件发送失败，请联系管理员", err)
	//	return
	//}

	// todo 2: 将验证码记录至 redis 中, 并设置过期时间为 5 分钟
	err = conf.Redis.Set(conf.Redis.Context(), fmt.Sprintf("ckm:%s:%s", data.Email, ip), ckm, 5 * 60 * time.Second).Err()
	if err != nil {
		//ctx.JSON(iris.Map{"code": 500, "msg": "系统错误"})
		api.Fail(ctx, "系统错误", err)
		return
	}
	err = conf.Redis.Set(conf.Redis.Context(), fmt.Sprintf("lockckm:%s:%s", data.Email, ip), ckm, 60 * time.Second).Err()
	if err != nil {
		//ctx.JSON(iris.Map{"code": 500, "msg": "系统错误"})
		api.Fail(ctx, "系统错误", err)
		return
	}
	// todo 3: 返回处理，表示已经发送
	//ctx.JSON(iris.Map{ "code": 200, "msg": "邮件已发送，请耐心等待邮件发送到您的邮箱中",
	//})
	//api.OK(ctx, "邮件已发送，请耐心等待邮件发送到您的邮箱中", "")
	api.OK(ctx, "测试邮件：" + ckm, "")
}

// 验证码登录
func CkmLogin(ctx iris.Context)  {
	var data struct {
		Email string `json:"email" validate:"required"`
		Ckm string `json:"ckm" validate:"required"`
	}
	ctx.Next()
	if err := ctx.ReadJSON(&data); err != nil {
		//ctx.JSON(iris.Map{"code": 505, "msg": "缺少参数"})
		api.Fail(ctx, "缺少参数", err)
		return
	}
	// 判断该邮箱是否存在
	_, err := service.FindOneUserByEmail(data.Email)
	if err == mgo.ErrNotFound {
		//ctx.JSON(iris.Map{"code": 505, "msg": "邮箱不存在"})
		api.Fail(ctx, "邮箱不存在", err)
		return
	}

	// 获取 ip 地址
	addr := ctx.Request().RemoteAddr
	ip, _, err := net.SplitHostPort(addr)
	if err != nil {
		//ctx.JSON(iris.Map{ "code": 500, "msg":  "系统错误"})
		api.Fail(ctx, "系统错误", err)
		return
	}

	// 验证验证码是否正确
	ckm, err := conf.Redis.Get(conf.Redis.Context(), fmt.Sprintf("ckm:%s:%s", data.Email, ip)).Result()
	if err != nil || err == redis.Nil {
		//ctx.JSON(iris.Map{"code": 505, "msg": "验证码过期"})
		api.Fail(ctx, "验证码过期", err)
		return
	}
	if data.Ckm != ckm {
		//ctx.JSON(iris.Map{"code": 505, "msg": "验证码错误"})
		api.Fail(ctx, "验证码错误", err)
		return
	}

	// 生成 jwt
	// 通过邮箱验证验证用户，
	// 通过ip来保证同一时间只能在一台电脑上登录，
	// 通过过期时间来确保超出时间后它不在有效
	token := api.NewJwt(data.Email, ip)

	auth, err := token.SignedString([]byte(conf.JwtKey))
	//auth, err := token.SigningString()
	if err != nil {
		api.Fail(ctx, "生成token失败：", err)
		return
	}
	//ctx.JSON(iris.Map{"code": 200, "data": iris.Map{"token": "bears "+auth}})
	api.OK(ctx, "", iris.Map{"token": auth})
}
