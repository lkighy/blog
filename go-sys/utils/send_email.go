package utils

import (
	"go-sys/conf"
	"gopkg.in/gomail.v2"
)

// 发送邮件
// from: 发送人
// name: 使用的名称
// to: 发送给谁
// body: 正文
func SendEmail(email, title,  body string) error {
	// todo 1: 获取 发送者的 smtp
	m := gomail.NewMessage()
	m.SetHeader("From", conf.SmtpEmail)
	m.SetHeader("To", email)
	m.SetHeader("Subject", title)
	//m.SetHeader("text/html", body)
	m.SetBody("text/html", body)
	d := gomail.NewDialer(conf.SmtpHost, conf.SmtpPort, conf.SmtpAuth, conf.SmtpPawd)
	return d.DialAndSend(m)
}

