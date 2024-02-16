package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"net/http"
	"os"
	"strings"
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
	addr := flag.String("addr", "0.0.0.0:3000", "address to bind to")
	flag.Parse()

	// print a useful address
	p := func(a string) {
		if !strings.Contains(a, ":") {
			_, port, _ := net.SplitHostPort(*addr)
			a = a + ":" + port
		}
		fmt.Printf("Serving %s at http://%s/\n", *dir, a)
	}
	if !strings.HasPrefix(*addr, "0.") {
		p(*addr)
	} else {
		addrs, err := net.InterfaceAddrs()
		if err != nil {
			p(*addr)
		}
		for _, address := range addrs {
			if ipnet, ok := address.(*net.IPNet); ok && !ipnet.IP.IsLoopback() {
				if ipnet.IP.To4() != nil {
					p(ipnet.IP.String())
				}
			}
		}
	}

	h := withLogging(http.FileServer(http.Dir(*dir)))
	if err := http.ListenAndServe(*addr, h); err != nil {
		log.Fatal(err)
	}
}
