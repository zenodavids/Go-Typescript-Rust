/*
* There are special types that can create loopholes to by-pass errors when dealing with the "implicit" type assignment

 * "any" - used to "disable type checking". allow different datatypes to be reassigned as values (should be avoided at "any" cost...)

 * "unknown" - similar, but safer alternative to "any".

 * "never" - is rarely used, effectively throws an error whenever it is defined.

 * "undefined & null" - refer to the JavaScript primitives undefined and null respectively.
 */

// =====================================================
// any
// ! TypeScript will not be able provide type safety, and tools which rely on type data, such as auto completion, will not work. Remember, it should be avoided at "any" cost...
// Example without the "any" keyword
let u = true
u = 'string' // Error: Type 'string' is not assignable to type 'boolean'.

// Example the "any" keyword
let v: any = true
v = 'string' // no error as it can be "any" type
Math.round(v) // no error as it can be "any" type
console.log(v)

// ===================================================
// unknown
let w: unknown = 1
w = 'string' // no error

w = {
  runANonExistentMethod: () => {
    console.log('I think therefore I am')
  },
} as { runANonExistentMethod: () => void }

//? How can we avoid the error for the code commented out below when we don't know the type?
// w.runANonExistentMethod(); // Error: Object is of type 'unknown'.

if (typeof w === 'object' && w !== null) {
  ;(w as { runANonExistentMethod: Function }).runANonExistentMethod()
}
// Although we have to cast multiple times we can do a check in the if to secure our type and have a safer casting
