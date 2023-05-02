/*
* Tuples
A tuple is a typed-element array with a pre-defined length and types for each index.
they allow each element in the array to be a known type of value
 * To define a tuple, specify the type of each element in the array:
 */

// Good Tuple
let goodTuple: [number, boolean, string]
// initialize correctly like in the order above
goodTuple = [5, false, 'Coding God was here']

// Bad Tuple
let badTuple: [number, boolean, string]
// initialized incorrectly which throws an error
badTuple = [false, 'Coding God was mistaken', 5]

// ==============================> Readonly Tuple
// A good practice is to make your tuple readonly.
// Tuples only have strongly defined types for the initial values:

let ourTuple: readonly [number, boolean, string]
// initialize correctly
ourTuple = [5, false, 'Coding God was here']
// We have no type safety in our tuple for indexes after the third element in the array
ourTuple.push('Something new and wrong')

// ==============================> Named Tuples

// Named tuples allow us to provide context for our values at each index.

const nameGraph: [x: number, y: number] = [55.2, 41.3]
// access the value of "x" using "graph[0]" and the value of "y" using "graph[1]".

// ==============================> Destructuring Tuples

// Destructuring tuples allow us to extract values from a tuple and assign them to variables.

const graph: [number, number] = [55.2, 41.3]
const [x, y] = graph

// use the variables x and y to access the values of the tuple elements.
console.log(x) // Output: 55.2
console.log(y) // Output: 41.3

// Tuples are useful when we need to represent a collection of heterogeneous data, such as an address. For example:
let address: [string, number, string] = ['123 Main Street', 12345, 'New York']

// We can also use tuples to create functions that return multiple values. For example:
function getUserInfo(): [string, number] {
  let name = 'John'
  let age = 30
  return [name, age]
}

let userInfo = getUserInfo()
console.log(userInfo[0]) // Outputs "John"
console.log(userInfo[1]) // Outputs 30
