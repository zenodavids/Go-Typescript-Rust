// - **int** - stores whole numbers like **123** or **-123**
// - **float** - stores decimal numbers like **1.23** or **-1.23**
// - **string** - stores text like **Hello**
// - **bool** - stores values like **true** or **false**

package main

import (
	"fmt"
)

/////////////////////////////////////////////////////////////////
/* =============== single Variables ======================= */
/////////////////////////////////////////////////////////////////

func variable() {
	// ===============================================

	/* creating (declaring) variables - 2 ways */

	// 1 -  var variableName type = value
	// can be created outside/inside of a function and used inside of a function
  var student1 string = "John" //type is string

	// 2 - variableName := value
	// ! must be created and used inside a function
  x := 2 //type is inferred

  var student2 = "Jane" //type is inferred

    fmt.Println(student1)
  	fmt.Println(student2)
  	fmt.Println(x)
//   ===============================================

/* Variable Declaration Without Initial Value */

  	var a string
  	var b int
  	var c bool

	// variables with no values will be set to the default value of its type:
	fmt.Println(a) // prints an empty space
  	fmt.Println(b) // prints 0
  	fmt.Println(c) // prints false

	// can assign a value to a variable after it is declared
	a = "Zeno";
	b = 2;
	c = true;

	fmt.Println(a) // now prints Zeno
  	fmt.Println(b) // now prints 2
  	fmt.Println(c) // now prints true
	
	// ! can't declare a variable using ":=" without assigning a value to it.
	// =============================================================


}

/////////////////////////////////////////////////////////////////
/* =============== Multiple Variables ======================= */
/////////////////////////////////////////////////////////////////

func multipleVariables(){
	// ===============================================
	// declare multiple variables in the same line.

	// using the "type" keyword (int) allows only one type of variables
	var a, b, c, d int = 1, 3, 5, 7

  	fmt.Println(a)
  	fmt.Println(b)
  	fmt.Println(c)
  	fmt.Println(d)

	// if the type is not specified, any type can be used
	var e, f = 6, "Hello"
  	g, h := 7, "World!"

  	fmt.Println(e)
  	fmt.Println(f)
  	fmt.Println(g)
  	fmt.Println(h)

	// multiple variables can also be grouped
	   var (
     i int
     j int = 1
     k string = "hello"
   )

  fmt.Println(i)
  fmt.Println(j)
  fmt.Println(k)


	// 
}