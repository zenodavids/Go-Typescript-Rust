// Topic: Decision making with match
//
// Program requirements:
// * Display "none", "two", "three" or "other" based on whether the value of a variable is 1, 2, 3 or some other number, respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * use an underscore (_) o match any variable

fn main (){
  let num = 2;

  match num{
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("other"),
  }

}