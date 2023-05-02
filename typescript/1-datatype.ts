// Datatypes in TypeScript are used to define the type of a variable or a function parameter or the return type of a function. There are two categories of datatypes in TypeScript:

// ============================================================
// ===============> 1. Primitive Datatypes:
// These are basic datatypes and include number, string, boolean, undefined and null.

// Number:
// Number data type is used to represent numerical values. It can be written with or without decimal points.
// Example:
let numberExample: number = 10
console.log(numberExample)
// output: 10

// String:
// String data type is used to represent textual data. It is written within single or double quotes.
// Example:
let stringExample: string = 'Hello World!'
console.log(stringExample)
// output: Hello World!

// Boolean:
// Boolean data type is used to represent true or false values.
// Example:
let booleanExample: boolean = true
console.log(booleanExample)
// output: true

// Undefined:
// Undefined data type is used to represent an uninitialized value.
// Example:
let undefinedExample: undefined
console.log(undefinedExample)
// output: undefined

// Null:
// Null data type is used to represent an empty value.
// Example:
let nullExample: null = null
console.log(nullExample)
// output: null

// ==========================================================
// ===============> 2. Complex Datatypes:
// These are more advanced datatypes and include array, tuple, enum, any and void.

// Array:
// Array data type is used to represent a collection of values of the same type.
// Example:

let arrayExample: number[] = [1, 2, 3, 4]
console.log(arrayExample)
// output: [1, 2, 3, 4]

// Tuple:
// Tuple data type is used to represent a collection of values of different types.
// Example:
let tupleExample: [string, number] = ['Hello', 10]
console.log(tupleExample)
// output: ['Hello', 10]

// Enum:
// Enum data type is used to create a set of named constants.
// Example:
enum Color {
  Red,
  Green,
  Blue,
}
let enumExample: Color = Color.Blue
console.log(enumExample)
// output: 2

// Any:
// Any data type is used to represent any type of value.
// Example:
let anyExample: any = 10
console.log(anyExample)
// output: 10

anyExample = 'Hello'
console.log(anyExample)
// output: Hello

// Void:
// Void data type is used to represent an absence of a value.
// Example:
function returnNothing(): void {
  console.log('No return value')
}
let voidExample: void = returnNothing()
console.log(voidExample)
// output: No return value
