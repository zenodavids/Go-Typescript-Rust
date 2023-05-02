// TypeScript allows types to be defined "separately" from the variables that use them.
// Aliases and Interfaces allows types to be easily shared between different variables/objects.

// ===========================================================> Type Aliases

// Aliases allow defining types with a custom name (an Alias).
// can be used for primitives like string or more complex types such as objects and arrays:

// define a type alias for the year of a car, which is a number
type CarYear = number
// define a type alias for the type of a car, which is a string
type CarType = string
// define a type alias for the model of a car, which is also a string
type CarModel = string
// define a type for a car, which has a year (a CarYear), a type (a CarType), and a model (a CarModel)
type Car = {
  year: CarYear
  type: CarType
  model: CarModel
}
// create a variable called carYear and assign it the value 2001 (which is a CarYear)
const carYear: CarYear = 2001
// create a variable called carType and assign it the value 'Toyota' (which is a CarType)
const carType: CarType = 'Toyota'
// create a variable called carModel and assign it the value 'Corolla' (which is a CarModel)
const carModel: CarModel = 'Corolla'
// create a variable called vehicle and assign it an object with a year (set to carYear), a type (set to carType), and a model (set to carModel)
const vehicle: Car = {
  year: carYear,
  type: carType,
  model: carModel,
}

// ===============================================> Interfaces
// Interfaces are similar to type aliases, except they only apply to "object" types.

// define an interface called Rectangle that describes the properties of a rectangle
interface Rectangle {
  height: number // The height of the rectangle (a number)
  width: number // The width of the rectangle (a number)
}

// create a variable called rectangle and assign it an object with a height of 20 and a width of 10
const rectangle: Rectangle = {
  height: 20,
  width: 10,
}

// =============== Extending Interfaces

// Interfaces can extend each other's definition.

// Extending an interface means you are creating a new interface with the same properties as the original, plus something new.

// define an interface called Rectangle that describes the properties of a rectangle
interface Rectangle {
  height: number // The height of the rectangle (a number)
  width: number // The width of the rectangle (a number)
}

// define an interface called ColoredRectangle that extends the Rectangle interface because a color property wants to be added to it.
interface ColoredRectangle extends Rectangle {
  color: string // The color of the rectangle (a string)
}

// create a variable called coloredRectangle and assign it an object with a height of 20, a width of 10, and a color of "red"
const coloredRectangle: ColoredRectangle = {
  height: 20,
  width: 10,
  color: 'red',
}

console.log(coloredRectangle)
