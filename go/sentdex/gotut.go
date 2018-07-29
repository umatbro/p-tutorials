package main

import (
	"fmt"
)

func add(x, y float64) float64 {
	return x + y
}

const x int = 5

func multiple(a, b string) (string, string) {
	return a, b
}

func main() {
	var num1, num2 float64 = 5.6, 9.5
	// or 
	// num1, num2 := 5.6, 9.5

	fmt.Println(add(num1, num2))
	
	w1, w2 := "Hey", "there"
	fmt.Println(multiple(w1, w2))

	var a int = 62
	var b float64 = float64(a)
	x := a // x will be type int
}
