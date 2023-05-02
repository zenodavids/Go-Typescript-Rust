//  Union Types

// Union types are used when a value can be more than a single type. Such as when a property would be string or number.

// ==================================> Union | (OR)

// Using the | we are saying our parameter is a string or number:
function printStatusCode(code: string | number) {
  console.log(`My status code is ${code}.`)
}
printStatusCode(404)
printStatusCode('404')

// ==================================> Union Type Errors
//! Note: you need to know what your type is when union types are being used to avoid type errors:

function StatusCode(code: string | number) {
  console.log(`My status code is ${code.toUpperCase()}.`) // error: Property 'toUpperCase' does not exist ontype 'string | number'.
  // TODO: Property 'toUpperCase' does not exist on type 'number'
}
