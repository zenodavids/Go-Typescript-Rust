// Generics makes it easier to write reusable code.

// ================================================> Functions

// Generics with functions help make more generalized methods which more accurately represent the types used and returned.

function createPair<S, T>(v1: S, v2: T): [S, T] {
  return [v1, v2]
}
console.log(createPair<string, number>('hello', 42)) // ['hello', 42]

// ================================================> Classes

// Generics can be used to create generalized classes.

// define a class called NamedValue with a generic type T
class NamedValue<T> {
  // declare a private property called _value with type T or undefined
  private _value: T | undefined

  // create a constructor with a parameter name of type string and make it private
  constructor(private name: string) {}

  // create a method called setValue that takes in a parameter value of type T
  public setValue(value: T) {
    // set _value to value
    this._value = value
  }

  // create a method called getValue that returns _value
  public getValue(): T | undefined {
    return this._value
  }

  // create a method called toString that returns a string containing the name and _value
  public toString(): string {
    return `${this.name}: ${this._value}`
  }
}

// create a new instance of NamedValue with type number and name 'myNumber'
let value = new NamedValue<number>('myNumber')
// set the value of the instance to 10
value.setValue(10)
// log the result of the toString method of the instance to the console
console.log(value.toString()) // myNumber: 10

// ================================================> Type Aliases

// Generics in type aliases allow creating types that are more reusable.

type Wrapped<T> = { value: T }

const wrappedValue: Wrapped<number> = { value: 10 }

// ================================================> Default Value

// Generics can be assigned default values which apply if no other value is specified or inferred.

// Define a generic class NamedValue that takes a type argument T (defaults to string)
class NamedValue<T = string> {
  // A private property named _value of type T or undefined
  private _value: T | undefined

  // A constructor that takes a string argument named name, which is assigned to a private property named name
  constructor(private name: string) {}

  // A method named setValue that takes a value of type T as argument and assigns it to the private property _value
  public setValue(value: T) {
    this._value = value
  }

  // A method named getValue that returns the value of the private property _value
  public getValue(): T | undefined {
    return this._value
  }

  // A method named toString that returns a string representation of the NamedValue instance
  public toString(): string {
    return `${this.name}: ${this._value}`
  }
}

// Create an instance of the NamedValue class with type argument inferred as string
let value = new NamedValue('myNumber')

// Set the value of the NamedValue instance to 'myValue'
value.setValue('myValue')

// Print the string representation of the NamedValue instance to the console
console.log(value.toString())

// ================================================> Extends

// Constraints can be added to generics to limit what's allowed. The constraints make it possible to rely on a more specific type when using the generic type.

function createLoggedPair<S extends string | number, T extends string | number>(
  v1: S,
  v2: T
): [S, T] {
  console.log(`creating pair: v1='${v1}', v2='${v2}'`)
  return [v1, v2]
}
