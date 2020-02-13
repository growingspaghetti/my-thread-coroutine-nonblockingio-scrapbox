package main

import "fmt"

func main() {
	// pausable
	c := make(chan string)
	// spawn resumer
	go func() {
		c <- "a"
	}()
	// pause
	fmt.Println(<-c)
}
