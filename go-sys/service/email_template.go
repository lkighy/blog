package service

import (
	"go-sys/conf"
	"gopkg.in/mgo.v2/bson"
)

// 邮件模板
type EmailTemplate struct {
	Email string // 邮箱
	Title string // 标题
	// 例如: <h1>验证码</h1><p>这是您的验证码%s</p>
	Template string // 模板
}

func (t *EmailTemplate) FindOne(document interface{}) error {
	coll := conf.DB.C("emailTemplate")
	return coll.Find(document).One(t)
}

// 通过邮箱查询邮件模板
func FindOneEmailTemplateByEmail(email string) (EmailTemplate, error){
	coll := conf.DB.C("emailTemplate")
	emailTemplate := EmailTemplate{}
	err := coll.Find(bson.M{"email": email}).One(&emailTemplate)
	return emailTemplate, err
}
