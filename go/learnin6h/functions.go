/*
* anonymous functions
* functions as typtes

*/
package main

import (
	"fmt"
)

func main() {
	for i := 0; i < 5; i++ {
		sayMessage("Test", i)
	}

	name := "Lilly"
	sayGreetingPtr("Hello", &name)
	fmt.Println(name)

	fmt.Println(sum(1, 2, 3))

	fmt.Println(divide(5, 1))

	anonymous := func() {fmt.Println("I'm anon")}
	anonymous()

	g := greeter{"Oh hi", "Mark"}
	g.greet()
	fmt.Println(g.name)
}

func sayMessage(msg string, idx int) {
	fmt.Print(msg)
	fmt.Print(" | index: ")
	fmt.Println(idx)
}

func sayGreetingPtr(greeting string, name *string) { 
	fmt.Println(greeting, *name)
	*name = "Ted"
}

func sum(values ...int) (result int) {
	/*
	This function has syntactic sugar in its declaration.
	(result int) is a named variable that will be returned.
	(no need to type return result, only return is sufficient)
	*/
	fmt.Println("Sum", values)
	for _, v := range values {
		result += v
	}
	return
}

/**
multiple return values
*/
func divide(a, b float64) (float64, error) {
	if b == 0.0 {
		return 0.0, fmt.Errorf("Cannot divide by 0")
	}
	return a/b, nil
}


type greeter struct {
	greeting string
	name string
}

func (g *greeter) greet() {
	fmt.Println(g.greeting, g.name)
	g.name = ""
}
