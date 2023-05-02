// ================================================> Partial

// Partial changes all the properties in an object to be optional.

interface Point {
  x: number
  y: number
}

let pointPart: Partial<Point> = {} // `Partial` allows x and y to be optional
pointPart.x = 10

// ================================================> Required

// Required changes all the properties in an object to be required.

interface Vehicle {
  brand: string
  model: string
  mileage?: number
}

let myCar: Required<Vehicle> = {
  brand: 'Ford',
  model: 'Focus',
  mileage: 12000, // `Required` forces mileage to be defined
}

// ================================================> Record

// Record is a shortcut to defining an object type with a specific key type and value type.

const peerAgeMap: Record<string, number> = {
  Alice: 21,
  Bob: 25,
}

//? NOTE: "Record<string, number>" is equivalent to "{ [key: string]: number }"

// ================================================> Omit

// Omit removes keys from an object type.

interface Recruit {
  identity: string
  age: number
  location?: string
}

const bob: Omit<Recruit, 'age' | 'location'> = {
  identity: 'Bob',
  // `Omit` has removed age and location from the type and they can't be defined here
}

// ================================================> Pick

// Pick removes all but the specified keys from an object type.

interface People {
  person: string
  age: number
  location?: string
}

const jane: Pick<People, 'person'> = {
  person: 'jane',
  // `Pick` has only kept name, so age and location were removed from the type and they can't be defined here
}

// ================================================> Exclude

// Exclude removes types from a union.

type Primitive = string | number | boolean
const option: Exclude<Primitive, string> = true // a string cannot be used here since Exclude removed it from the type.

// ================================================> ReturnType

// ReturnType extracts the return type of a function type.

type PointGenerator = () => { x: number; y: number }
const point: ReturnType<PointGenerator> = {
  x: 10,
  y: 20,
}

// ================================================> Parameters

// Parameters extracts the parameter types of a function type as an array.

// This defines a type PointPrinter which is a function that takes an object with x and y coordinates and returns nothing
type PointPrinter = (p: { x: number; y: number }) => void

// This gets the type of the first parameter of the PointPrinter function using the Parameters utility type.
// In this case, it retrieves the type of the object with x and y properties.
// Then it declares a constant named point with that type and assigns it an object with x and y properties.
const pointer: Parameters<PointPrinter>[0] = {
  x: 10,
  y: 20,
}
