package main

import (
	"embed"
	"io/fs"
	"log"
	"net/http"
)

//go:embed build/*
var static embed.FS //nolint:gochecknoglobals

func main() {
	substatic, errSub := fs.Sub(static, "build")
	if errSub != nil {
		log.Fatal("static Router: %w", errSub)
	}

	staticFS := http.FS(substatic)
	fs := http.FileServer(staticFS)

	http.Handle("/", fs)

	log.Println("Listening on :3000")
	err := http.ListenAndServe(":3000", nil)
	if err != nil {
		log.Fatal(err)
	}
}
