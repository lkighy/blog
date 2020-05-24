package utils

import (
	"fmt"
	"go-sys/conf"
	"net/smtp"
)

// 发送邮件
// from: 发送人
// name: 使用的名称
// to: 发送给谁
// body: 正文
func SendEmail(email, body string) {
	// todo 1: 获取 发送者的 smtp
	a := smtp.PlainAuth("", conf.SmtpAuth, conf.SmtpPawd, conf.SmtpHost)
	to := []string{email}
	msg :=
	smtp.SendMail(fmt.Sprintf("%s:%s", conf.SmtpHost, conf.SmtpPort), a, conf.SmtpEmail, to, []byte(body))
}

func mailMsg() {
	form := "form: name <100@qq.com>"
	to := "to: <100@qq.com>\r\nto: <100@qq.com>\r\nto: <100@qq.com>"
	subject := "subject: 这是标题"
	body := "这就是内容了喔"
	msg := fmt.Sprintf("%s\r\n%s\r\n%s\r\n%s\r\n", form, to, subject, body)
}