package main

import (
	"fmt"
)

func dataTypes() {
	// four major types

   // 1 - Boolean
	var a bool = true  

 // 2 - Integer - can use type like uint, uint8, uint16, uint32, uint64
//   signed(both values) 
	signed:= -2
//  unsigned(only positive values)
	var b int = 5
	var unsigned uint = 2
// Type			Size				Range
// ==========================================
// (u)int		Depends				32 bits or 64 bit 
// (u)int8		8 bits/1 byte		0 to 255
// (u)int16		16 bits/2 byte		0 to 65535
// (u)int32		32 bits/4 byte		0 to 4294967295
// (u)int64		64 bits/8 byte		0 to 18446744073709551615



// 3 - Floats -  can use type like float32 and float64(float64 is default)
	var c float32 = 3.14  
	var x float64 = 1.7e+308
// Type			Size		Range
// ==================================
// float32		32 bits		-3.4e+38 to 3.4e+38.
// float64		64 bits		-1.7e+308 to +1.7e+308.


 // 4 - String
	var d string = "Hi!" 


	fmt.Println("Boolean: ", a)
	fmt.Println("Integer: ", b)
	fmt.Println("Float:   ", c)
	fmt.Println("String:  ", d)
	fmt.Printf("Type: %T, value: %v", signed, signed)
	fmt.Printf("Type: %T, value: %v", unsigned, unsigned)
	fmt.Printf("Type: %T, value: %v\n", x, x)
}
