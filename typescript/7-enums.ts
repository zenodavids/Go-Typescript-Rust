// Enums come in two flavors "string" and "numeric". Lets start with numeric.

// ======================================> Numeric Enums(default)

// By default, enums will initialize the first value to 0 and add 1 to each additional value:

enum CardinalDirections {
  North,
  East,
  South,
  West,
}
let currentDirection = CardinalDirections.North
console.log(currentDirection) // logs 0

currentDirection = 'North' // Error: "North" is not assignable to type 'CardinalDirections'.

// Numeric Enums - Initialized
// You can set the value of the first numeric enum and have it auto increment from that:

enum Directions {
  North = 1,
  East,
  South,
  West,
}

console.log(Directions.North) // logs 1
console.log(Directions.West) // logs 4

// Numeric Enums - Fully Initialized
// You can assign unique number values for each enum value. Then the values will not incremented automatically:

enum StatusCodes {
  NotFound = 404,
  Success = 200,
  Accepted = 202,
  BadRequest = 400,
}

console.log(StatusCodes.NotFound) // logs 404
console.log(StatusCodes.Success) // logs 200

// =============================================> String Enums

// Enums can also contain strings. This is more common than numeric enums, because of their readability and intent.

// For example, the following code defines an enum called Color with four values.

enum Paint {
  RED = 'Red',
  GREEN = 'Green',
  BLUE = 'Blue',
  YELLOW = 'Yellow',
}

// Now we can use the Color enum to refer to each of the values instead of using the strings directly:

let paint: Paint = Paint.RED
