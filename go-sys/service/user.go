package service

import (
	"go-sys/conf"

	"gopkg.in/mgo.v2/bson"
)

type User struct {
	Email    string
	Username string
}

// 查询
func FindOneUserByEmail(email string) (User, error) {
	coll := conf.DB.C("user")

	user := User{}
	err := coll.Find(bson.M{"email": email}).One(&user)

	return user, err
}

// 修改
