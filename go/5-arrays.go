package main

import (
	"fmt"
)

func array() {

////////////////////////////////////////////////
/* ====== Declare an Array in two ways ====== */
////////////////////////////////////////////////

/*first way(defined length):

var array_name = [length]datatype{values} 
array_name := [length]datatype{values}
*/ 
	var arr1 = [3]int{1,2,3}
	arr2 := [5]int{4,5,6,7,8}
	var cars = [4]string{"Volvo", "BMW", "Ford", "Mazda"}

/*second way(undefined length):

var array_name = [...]datatype{values}
array_name := [...]datatype{values} 
*/ 
	var arr3 = [...]int{1,2,3}
	arr4 := [...]int{4,5,6,7,8}

	fmt.Println(arr1)
	fmt.Println(len(arr2)) // returns the length of an array
	fmt.Println(arr3)
	fmt.Println(arr4)
	fmt.Print(cars)

/////////////////////////////////////////
/* ====== Array Initialization ====== */
///////////////////////////////////////

  arr5 := [5]int{} //not initialized
  arr6 := [5]int{1,2} //partially initialized
  arr7 := [5]int{1,2,3,4,5} //fully initialized

  fmt.Println(arr5)
  fmt.Println(arr6)
  fmt.Println(arr7)

/////////////////////////////////////////////////////
/* ====== Initialize Only Specific Elements ====== */
////////////////////////////////////////////////////

// The array above has 5 elements.
arr8 := [5]int{1:10,2:40}

// 1:10 means: assign 10 to array index 1 (second element).
// 2:40 means: assign 40 to array index 2 (third element).

	fmt.Println(arr8)

////////////////////////////////////////////////////////////////
/* ====== Access Elements of an Array by their index number ====== */
////////////////////////////////////////////////////////////////

	prices := [3]int{10,20,30}

	fmt.Println(prices[0]) // prints 10
	fmt.Println(prices[2]) // prints 30

/////////////////////////////////////////////////////////////////////////////
/* ====== change the value of an array element by their index number ====== */
/////////////////////////////////////////////////////////////////////////////

	number := [3]int{10,20,30}

	number[2] = 50 // change the element in the second index (which is th third element) to 50 
	fmt.Println(number)

}	