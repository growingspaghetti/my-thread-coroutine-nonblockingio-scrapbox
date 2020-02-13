package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"time"
)

func main() {
	// pausable
	c := make(chan string)

	// spawn resumer 1
	go func() {
		resp, err := http.Get("https://www.youtube.com/")
		if err != nil {
			panic(err)
		}
		defer resp.Body.Close()
		text, _ := ioutil.ReadAll(resp.Body)
		c <- string(text)
	}()

	// either
	select {
	// pause a
	case youtube := <-c:
		fmt.Println(youtube)
	// pause b
	case <-time.After(3 * time.Nanosecond): // spawn resumer 2
		fmt.Println("time up.")
	}
}
