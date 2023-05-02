// Go has three functions to output text;
// Print()
// Println()
// Printf()

package main

import (
	"fmt"
)

func print() {
	///////////////////////////////////////////////////////////
/* =============== Print() function ======================= */
//////////////////////////////////////////////////////////////
  var i,j string = "Hello","World"

// print multiple lines with just one print function
  fmt.Print(i, "\n",j)

//  use " " to add a space between string arguments
  fmt.Print(i, " ", j)

//  inserts a space between the arguments if neither are strings
	var a, b = 10,20
  fmt.Print(a, b)

  	///////////////////////////////////////////////////////////
/* =============== Println() function ======================= */
//////////////////////////////////////////////////////////////
//  similar to Print(), difference is a whitespace is added between the arguments, and a newline is added at the end

	fmt.Println(i, "\n",j)


///////////////////////////////////////////////////////////
/* =============== Printf() function ======================= */
//////////////////////////////////////////////////////////////
// first formats its argument based on the given formatting verb and then prints them.
// %v is used to print the value of the arguments
// %#v	Prints the value in Go-syntax format
// %T	Prints the type of the value
// %%	Prints the % sign

  var d string = "Hello"
  var e int = 15

  fmt.Printf("%v\n", d)
  fmt.Printf("%#v\n", d)
  fmt.Printf("d has value: %v and type: %T\n", d, d)
  fmt.Printf("e has value: %v and type: %T", e, e)
	fmt.Printf("%v%%\n", e)
  fmt.Printf("%T\n", e)


//   Integer Formatting Verbs
// =======================
// Verb		Description
// %b		Base 2
// %d		Base 10
// %+d		Base 10 and always show sign
// %o		Base 8
// %O		Base 8, with leading 0o
// %x		Base 16, lowercase
// %X		Base 16, uppercase
// %#x		Base 16, with leading 0x
// %4d		Pad with spaces (width 4, right justified)
// %-4d		Pad with spaces (width 4, left justified)
// %04d		Pad with zeroes (width 4)


// Float Formatting Verbs
// ========================
// Verb		Description
// %e		Scientific notation with 'e' as exponent
// %f		Decimal point, no exponent
// %.2f		Default width, precision 2
// %6.2f	Width 6, precision 2
// %g		Exponent as needed, only necessary digits


// String Formatting Verbs
// =========================
// Verb		Description
// %s		Prints  value as plain string
// %q		Prints  value as a double-quoted string
// %8s		Prints  value as plain string (width 8, right justified)
// %-8s		Prints  value as plain string (width 8, left justified)
// %x		Prints  value as hex dump of byte values
// % x		Prints  value as hex dump with spaces


// Boolean Formatting Verbs
// ===========================
// Verb		Description
// %t		Prints Boolean Value (true or false) (same as using %v)

}