package main

import (
	"fmt"
)

func array() {
	a := [3]int{1, 2, 3}
	b := &a[0]
	c := &a[1]
	fmt.Printf("%v %p %p", a, b, c)
}

func main() {
	var x int = 20
	b := &x
	fmt.Println(x)
	fmt.Println(b)
	fmt.Println(*b)

	array()
}