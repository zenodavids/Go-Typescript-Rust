// Objects in typescripts
const car: { type: string; model: string; year: number } = {
  type: 'Toyota',
  model: 'Corolla',
  year: 2009,
}
car.type = 'Ford' // no error
car.type = 2 // Error: Type 'number' is not assignable to type 'string'.

// =============================> Optional Properties

// These are properties that don't have to be defined in the object definition.

// ? Bad way to use optional properties
const badCar: { type: string; mileage: number } = {
  // Error: Property 'mileage' is missing in type '{ type: string; }' but required in type '{ type: string; mileage: number; }'.
  type: 'Toyota',
}
badCar.mileage = 2000

// Good way to use optional properties(using the "?:" operator)
const goodCar: { type: string; mileage?: number } = {
  // no error
  type: 'Toyota',
}
goodCar.mileage = 2000

// =============================> Index Signatures

// can be used for objects without a defined list of properties.

const nameAgeMap: { [index: string]: number } = {}
// "[index: string]" means the key must be a string
// and the pair must be a number
nameAgeMap.Jack = 25 // no error
nameAgeMap.Mark = 'Fifty' // Error: Type 'string' is not assignable to type 'number'.
console.log(nameAgeMap)
