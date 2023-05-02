// A class is a blueprint of an object that defines its properties and methods. properties and methods are bundled together in a single unit.

// Class syntax in TypeScript is similar to other languages like Java, C#, etc.

// ========================================================> Members: Types

// The members of a class (properties & methods) are typed using type annotations, similar to variables.

class Employee {
  name: string
}
// Creating an Instance of a Class
const employee = new Employee()

//accessing the name property
employee.name = 'John Doe'
employee.name = 'Jane'

/* * Members: Visibility
  Class members modifiers affect visibility.

  There are three main visibility modifiers in TypeScript;

 * public - (default) :
  allows access to the class member from anywhere
 *private :
 only allows access to the class member from within the class
 * protected :
  allows access to the class member from itself and any classes that inherit it, which is covered in the inheritance section below
 * 
 */

// The "Person" class is defined here
class Person {
  // It has a private "name" property, which can only be accessed from within the class itself
  private name: string

  // The class also has a constructor method, which takes in a "name" parameter and assigns it to the "name" property (this.name = name)
  // The constructor is marked "public", which means it can be called from outside the class
  // We use the "this" keyword to refer to the current instance of the class
  public constructor(name: string) {
    this.name = name
  }

  // Finally, the class has a "getName" method, which simply returns the value of the "name" property
  // This method is marked "public", which means it can be called from outside the class
  public getName(): string {
    return this.name
  }
}

// Outside the class definition, we create a new instance of the "Person" class with the name "Jane"
// This creates a new object with a "name" property set to "Jane"
const person = new Person('Jane')

// ========================================================> Parameter Properties

// define class members in the constructor, by adding a visibility modifiers to the parameter.

class Staff {
  // name is a private member variable
  public constructor(private name: string) {}

  public getName(): string {
    return this.name
  }
}

const staff = new Staff('Jane')
console.log(staff.getName())

// ========================================================> Readonly

// the "readonly" keyword can prevent class members from being changed.

class Applicant {
  private readonly name: string

  public constructor(name: string) {
    // name cannot be changed after this initial definition, which has to be either at it's declaration or in the constructor.
    this.name = name
  }

  public getName(): string {
    return this.name
  }
}

const applicant = new Applicant('Jane')
console.log(applicant.getName())

// ========================================================> Inheritance: Implements

// Interfaces can be used to define the type a class must follow through the "implements" keyword.

// Define an interface named "Shape" that has a single method named "getArea" that returns a number
interface Shape {
  getArea: () => number
}

// Define a "Rectangle" class that implements the "Shape" interface
class Rectangle implements Shape {
  // Define a constructor that takes "width" and "height" as arguments and assigns them to corresponding properties
  public constructor(
    protected readonly width: number,
    protected readonly height: number
  ) {}
  // Define a "getArea" method that calculates and returns the area of the rectangle
  public getArea(): number {
    return this.width * this.height
  }
}

//! A class can implement multiple interfaces  like so: class Rectangle implements Shape, Colored {...

// ========================================================> Inheritance: Extends

// Classes can extend each other through the "extends" keyword.
// ! A class can only extends one other class.

interface Shape {
  // Define an interface named "Shape" that has a single method named "getArea" that returns a number
  getArea: () => number
}

class TheRectangle implements Shape {
  // Define a "TheRectangle" class that implements the "Shape" interface
  public constructor(
    protected readonly width: number,
    protected readonly height: number
  ) {
    // Define a constructor that takes "width" and "height" as arguments and assigns them to corresponding properties
  }
  public getArea(): number {
    // Define a "getArea" method that calculates and returns the area of the Therectangle
    return this.width * this.height
  }
}

class Square extends TheRectangle {
  // Define a "Square" class that extends the "TheRectangle" class
  public constructor(width: number) {
    // "overrides" TheRectangle's constructor and Define it's own constructor that takes "width" as a parameter and passes it as "width" and "height" to the "TheRectangle" constructor
    super(width, width)
  }
  // "getArea" gets inherited from "TheRectangle"
}

// ========================================================> Override

// When a class extends another class, it can replace the members of the parent class with the same name.
// ? Newer versions of TypeScript allow explicitly marking this with the override keyword.

interface Shape {
  getArea: () => number
}

class ARectangle implements Shape {
  // using protected for these members allows access from classes that extend from this class, such as ASquare
  public constructor(
    protected readonly width: number,
    protected readonly height: number
  ) {}
  public getArea(): number {
    return this.width * this.height
  }
  public toString(): string {
    return `ARectangle[width=${this.width}, height=${this.height}]`
  }
}

class ASquare extends ARectangle {
  public constructor(width: number) {
    super(width, width)
  }

  // this toString replaces the toString from ARectangle using the "override" keyword
  public override toString(): string {
    return `ASquare[width=${this.width}]`
  }
}

// ! By default the "override" keyword is optional when overriding a method, and only helps to prevent accidentally overriding a method that does not exist. Use the setting "noImplicitOverride" to force it to be used when overriding.

// ========================================================>  Abstract Class
// is like a blueprint for other classes to use.

abstract class Animal {
  abstract makeSound(): void

  move(): void {
    console.log('Moving...')
  }
}

// This Animal class is just a blueprint, it can't be used directly to create an animal.
// The Dog class has to follow the rules set out in the Animal blueprint, which says it must have a makeSound method.

class Dog extends Animal {
  makeSound(): void {
    console.log('Woof!')
  }
}

// Now we can create a new Dog object and use it like this:

const dog = new Dog()
dog.makeSound() // Output: "Woof!"
dog.move() // Output: "Moving..."
