// Slices in go programming language are "basically flexible arrays"

// ? cap() function - returns the capacity of the slice (the number of elements the slice can grow or shrink to)
// ? len() function - returns the length of the slice (the number of elements in the slice)
package main

import (
	"fmt"
)

func slice() {

///////////////////////////////////
/* ====== Create a slice ====== */
//////////////////////////////////

// three ways to create a slice

// =================> 1. Using the []datatype{values}
// 👉 slice_name := []datatype{values}

	// an empty slice of 0 length and 0 capacity
	createSlice := []int{}

	// a slice of integers of length 3 and also the capacity of 3.
	createSlice1 := []int{1,2,3}

	fmt.Println(createSlice)
	fmt.Println(createSlice1)



//===================> 2. Create a slice from an array
// 👉 slice_name := array_name[start:end]

	arr1 := [6]int{10, 11, 12, 13, 14,15} // an array
	myslice := arr1[2:4] // A slice made from the above array

	fmt.Printf("myslice = %v\n", myslice)
	fmt.Printf("length = %d\n", len(myslice))
	fmt.Printf("capacity = %d\n", cap(myslice))


// =================> 3. Using the make() function
// 👉 slice_name := make([]type, length, capacity)

  myslice1 := make([]int, 5, 10)
  fmt.Printf("myslice1 = %v\n", myslice1) // myslice1 = [0 0 0 0 0]
  fmt.Printf("length = %d\n", len(myslice1)) // length = 5
  fmt.Printf("capacity = %d\n", cap(myslice1)) // capacity = 10

// ! If the capacity parameter is not defined, it will be equal to length.
  myslice2 := make([]int, 5)
  fmt.Printf("myslice2 = %v\n", myslice2) // myslice2 = [0 0 0 0 0]
  fmt.Printf("length = %d\n", len(myslice2)) // length = 5
  fmt.Printf("capacity = %d\n", cap(myslice2)) // length = 5

///////////////////////////////////
/* ====== Modify a slice ====== */
//////////////////////////////////

// =================>  Change Elements of a Slice
// 👉 slice_name := [index]{ value(s)}

	prices := []int{10,20,30}

	// change the element in the 2nd index (which is 30) to 50
	prices[2] = 50

	fmt.Println(prices[2]) // prints 50

// =================>  Add / Append Elements To the end of a Slice
// 👉 slice_name = append(slice_name, element1, element2, ...)

	sliceAppend := []int{1, 2, 3, 4, 5, 6}

	fmt.Printf("sliceAppend = %v\n", sliceAppend) // sliceAppend = [1 2 3 4 5 6]
	fmt.Printf("length = %d\n", len(sliceAppend)) // length = 6
	fmt.Printf("capacity = %d\n", cap(sliceAppend))	// capacity = 6


	sliceAppend = append(sliceAppend, 20, 21)

	fmt.Printf("sliceAppend = %v\n", sliceAppend) // sliceAppend = [1 2 3 4 5 6 20 21]
	fmt.Printf("length = %d\n", len(sliceAppend)) // length = 8
	fmt.Printf("capacity = %d\n", cap(sliceAppend)) // capacity = 12

	// =================>  Add / Append One Slice To Another Slice
// 👉 slice3 = append(slice1, slice2...)

	sliceAppend1 := []int{1,2,3}
	sliceAppend2 := []int{4,5,6}
	sliceAppend3 := append(sliceAppend1, sliceAppend2...)

	fmt.Printf("sliceAppend3=%v\n", sliceAppend3) // sliceAppend3=[1 2 3 4 5 6]
	fmt.Printf("length=%d\n", len(sliceAppend3)) // length=6
	fmt.Printf("capacity=%d\n", cap(sliceAppend3)) // capacity=6

	// =================>  Change The Length of a Slice
// 👉 Unlike arrays in Go, it is possible to change the length of a slice.

	changeLengthArr := [6]int{9, 10, 11, 12, 13, 14} // An array

	changeSliceLength := changeLengthArr[1:5] // create a Slice from the array
	changeSliceLength = changeLengthArr[1:3] // Change length by re-slicing the array
	changeSliceLength = append(changeSliceLength, 20, 21, 22, 23) // Change length by appending items

	fmt.Printf(" %v\n", changeLengthArr)
	fmt.Printf(" %v\n", changeSliceLength)

//! =================>  Memory Efficiency
//  Go loads all the slice elements into the memory. so if you need just a specific set of elements to go into memory, use the copy() function
// 👉 copy(destination, source)


 // Original slice
	// Declaring a slice of integers with initial values
  numbers := []int{1,2,3,4,5,6,7,8,9,10,11,12,13,14,15} 
 
  // Printing the original slice
  fmt.Printf("numbers = %v\n", numbers)
  // Printing the length of the slice
  fmt.Printf("length = %d\n", len(numbers))
  // Printing the capacity of the slice
  fmt.Printf("capacity = %d\n", cap(numbers)) 


    
  // Create copy with only needed numbers
  // Creating a new slice with all elements except the last 10
  neededNumbers := numbers[:len(numbers)-10] 
  
  // Creating a new empty slice but giving it the same length as neededNumbers
  numbersCopy := make([]int, len(neededNumbers)) 

  // Copying the elements of neededNumbers to numbersCopy
  copy(numbersCopy, neededNumbers) 

  // Printing the new slice
  fmt.Printf("numbersCopy = %v\n", numbersCopy) 
  // Printing the length of the new slice
  fmt.Printf("length = %d\n", len(numbersCopy)) 
  // Printing the capacity of the new slice
  fmt.Printf("capacity = %d\n", cap(numbersCopy)) 

}
// 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