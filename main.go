package main

import (
	"flag"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"
)

func withLogging(h http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		h.ServeHTTP(w, r)
		fmt.Printf("%v\t%v\t%v\n",
			r.Method,
			time.Since(start).Truncate(time.Millisecond),
			r.RequestURI,
		)
	})
}

func main() {
	wd, _ := os.Getwd()
	dir := flag.String("dir", wd, "directory to serve")
	addr := flag.String("addr", "0.0.0.0:8080", "address to bind to")
	flag.Parse()
	fmt.Printf("Serving %s at http://%s/\n", *dir, *addr)
	h := withLogging(http.FileServer(http.Dir(*dir)))
	if err := http.ListenAndServe(*addr, h); err != nil {
		log.Fatal(err)
	}
}
