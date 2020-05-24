package main

import (
	stdContext "context"
	"github.com/kataras/iris/v12"
	"go-sys/app"
	"os"
	"os/signal"
	"syscall"
	"time"
	"go-sys/conf"
)

func main() {
	r := app.Router()

	go func() {
		ch := make(chan os.Signal, 1)
		signal.Notify(ch,
			// kill -SIGINT XXXX 或 Ctrl+c
			os.Interrupt,
			syscall.SIGINT, // register that too, it should be ok
			// os.Kill等同于syscall.Kill
			os.Kill,
			syscall.SIGKILL, // register that too, it should be ok
			// kill -SIGTERM XXXX
			syscall.SIGTERM,
		)
		select {
		case <-ch:
			println("shutdown...")
			timeout := 5 * time.Second
			ctx, cancel := stdContext.WithTimeout(stdContext.Background(), timeout)
			defer cancel()
			r.Shutdown(ctx)
			conf.Client.Close()
		}
	}()
	r.Run(iris.Addr("127.0.0.1:8080"))

	//app := iris.New()
	//app.Logger().SetLevel("debug")
	//// Optionally, add two built'n handlers
	//// that can recover from any http-relative panics
	//// and log the requests to the terminal.
	//app.Use(recover.New())
	//app.Use(logger.New())
	//
	//// Method:   GET
	//// Resource: http://localhost:8080
	//app.Handle("GET", "/", func(ctx iris.Context) {
	//	ctx.HTML("<h1>Welcome</h1>")
	//})
	//
	//// same as app.Handle("GET", "/ping", [...])
	//// Method:   GET
	//// Resource: http://localhost:8080/ping
	//app.Get("/ping", func(ctx iris.Context) {
	//	ctx.WriteString("pong")
	//})
	//
	//// Method:   GET
	//// Resource: http://localhost:8080/hello
	//app.Get("/hello", func(ctx iris.Context) {
	//	ctx.JSON(iris.Map{"message": "Hello Iris!"})
	//})
	//
	//// http://localhost:8080
	//// http://localhost:8080/ping
	//// http://localhost:8080/hello
	//app.Run(iris.Addr(":8080"), iris.WithoutServerError(iris.ErrServerClosed))
}