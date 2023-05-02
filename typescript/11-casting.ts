// Casting - this is the process of "overriding a type".

// ========================================> Casting with "as"

// A straightforward way to cast a variable is using the "as" keyword, which will directly change the type of the given variable.

let x: unknown = 'hello'
console.log((x as string).length)

// ! won't work because "y" is still holds a number.
let y: unknown = 4
console.log((y as string).length) // prints "undefined" since numbers don't have a length

// ========================================> Casting with "<>"
// ! This type of casting will not work with TSX, such as when working on React files.

// Using '<>' works the same as casting with 'as'.
let j: unknown = 'hello'
console.log((<string>j).length)

// ========================================> Force casting

// To override type errors that TypeScript may throw when casting, first cast to unknown, then to the target type.

let k = 'hello'
console.log((k as unknown as number).length) // k is not actually a number so this will return undefined
