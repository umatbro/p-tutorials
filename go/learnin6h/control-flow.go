package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

func req() {
	res, err := http.Get("http://www.google.com/robots.txt")
	if err != nil {
		log.Fatal(err)
	}
	robots, err := ioutil.ReadAll(res.Body)
	defer res.Body.Close()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s", robots)
}


func panic_() {
	a, b := 1, 0
	defer func () {
		if err := recover(); err != nil {
			log.Println("Error:", err)
		}
	}()
	ans := a / b
	fmt.Println(ans)
}


func main() {
	// fmt.Println("start")
	// defer fmt.Println("middle")
	// fmt.Println("end")
	// req()
	panic_()
}

/**
Defer
Panic
Recover

Defer can be used to delay certain action.
It works in a way that statement is delayed until the last statement in the function is executed but before 
function returns.
Defered calls are executed in LIFO order.
Arguments evaluated at the time defer is executed, not at the time of called function execution.

Panic
------
Panic is a state when application gets to the point where it cannot work anymore.

Recover
------
Used to recover from panics
Only useful in deferred functions
Current function will not attempt to continue, but higher functions in call stack will.
*/
