package service

import (
	"go-sys/conf"

	"gopkg.in/mgo.v2/bson"
)

type User struct {
	Email    string
	Username string
}

func FindOneUser(email string) {
	coll := conf.DB.C("user")

	result := User{}
	if err := coll.Find(bson.M{"email": email}).One(&result); err != nil {

	}

}
