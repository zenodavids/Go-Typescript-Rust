package main

import (
	"fmt"
)

func conditionals() {
	///////////////////////////////////
/* ====== Conditional Statements ====== */
//////////////////////////////////

// =================> 1. if, if else, else if

// 👉 snytax:
// if condition1 {
    // code to be executed if condition1 is true
// } else if condition2 {
    // code to be executed if condition1 is false and condition2 is true
//  } else {
   // code to be executed if condition1 and condition2 are both false
// }


  time := 22
  if time < 10 {
    fmt.Println("Good morning.")
  } else if time < 20 {
    fmt.Println("Good day.")
  } else {
    fmt.Println("Good evening.")
  }
}


