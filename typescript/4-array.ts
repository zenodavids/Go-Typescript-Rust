// For example, we can declare an array of strings as follows:
let animals: string[] = ['dog', 'cat', 'fish', 'lion']

const numbers = [1, 2, 3] // inferred to type number[]
numbers.push(4) // no error
// comment line below out to see the successful assignment
numbers.push('2') // Error: Argument of type 'string' is not assignable to parameter of type 'number'

// =======================> Using the Generic Array Type Syntax

// The generic array type syntax is used when the type of elements stored in the array is unknown or when the elements are of different types.
// For example, if you want to store strings and numbers in the same array, the generic array type syntax will be:
let myArray: Array<string | number> = ['Hello', 1, 2, 'World']

// =============================>  Array Keywords

// "readonly" -  prevent arrays from being changed.
const names: readonly string[] = ['Dylan']
names.push('Jack') // Error: Property 'push' does not exist on type 'readonly string[]'.
//? try removing the readonly modifier and see if it works?

// We can access the elements of the array using their index. The index of the first element is 0 and the last element is the length of the array minus 1.

let firstElement = animals[0] // firstElement will be "dog"
let lastElement = animals[animals.length - 1] // lastElement will be "lion"

// We can also add and remove elements from the array using the push() and pop() methods respectively.

// Adding an element
animals.push('elephant') // animals will now be ["dog", "cat", "fish", "lion", "elephant"]

// Removing an element
animals.pop() // animals will now be ["dog", "cat", "fish", "lion"]

// In addition, we can loop over all the elements of the array using the for...of loop.
for (let animal of animals) {
  console.log(animal) // will log "dog", "cat", "fish", "lion"
}
