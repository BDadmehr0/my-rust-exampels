// Dont Forget semicolon ";" :)

// Comments
/* 
Hello World 
*/

/*
Categories of Integer Data Types in Rust

Depending on the size of data, we can further classify the signed and unsigned integer type into various categories:


Size
	Signed
	Unsigned


8-bit
	i8
	u8


16-bit
	i16
	u16


32-bit
	i32
	u32


64-bit
	i64
	u64

128-bit
	i128
	u128
     */

/*
Rules for Naming Variables in Rust

let age = 31;     	// valid and good practice
let _age = 31;    	// valid variable 
let 1age = 31;    // inavlid variable

let age1 = 31;        // valid variable
let age_num = 31;     // valid variable
let s@lary = 52352;   // invalid variable

let first name = "Jack";    // invalid variable
let first_name = "Jack";    // valid variable
let first-name = "Jack";    // invalid variable

Note: Always try to give meaningful names to your variables. For example, name, age, number are better names than n, ag, nm.
 */

/*
Operator
	Example


+ (Addition)
	a + b


- (Subtraction)
	a - b


* (Multiplication)
	a * b


/ (Division)
	a / b


% (Remainder)
	a % b
 */

// exampel cpp requemets main functioan
fn main() {
     // varibale
     // for change after data use 'mut' in code exampel : "let mut age = 13" this methods not work 'const'

     /*
     Data Type in Rust
     - Integer
     - Floating-Point
     - Boolean
     - Character
     */

     // boolean type
     let flag1: bool = true;
     let flag2: bool = false;

     let number: i32 = 0; // i : integer type, 32: size of the data type (takes 32 bits of space memory)
     let salary: i32 = -1;
     let mut age = 13;
     let salary = 342523.23;

     let name = "dadmehr";
     // char type
     let character: char = 'z';

     // declare a float constant
     const PI: f32 = 3.14;

     //// Rust Type Casting
     // create a floating-point variable
     let decimal: f64 = 54.321;

     // convert floating point type to integer type
     let integer = decimal as u16;

     println!("character = {}\n", character);

     let character: char = 'B';

     // convert char type to u8 integer type
     let integer = character as u8;

     println!("character = {}", character);
     println!("integer = {}\n", integer);

     // if & else & elif(else if)
     if number == 0 {
          println!("if {} is greater than 0\n", number);
     } else if number > 0 {
          println!("elif {}", number);
     } else {
          println!("else {} ", number);
     }

     let number = -2;

     if number < 0 {
          // if outer condition evaluates to true evaluate the inner condition
          if number == -2 {
               println!("The current number is -2\n");
          } else {
               println!("The current number is {}\n", number);
          }
     }

     // 2 Methods print
     // println!("Hello World {}", name);
     print!("Hello World (without ln!) {}, {}", name, age);
     age = 14;
     // PI = 12; || error: aborting due to previous error
     println!("\nName = {0}, Age = {1}\n", name, age);
     // println!("Name = {name}, Age = {age}");

     /*
     '\n' new line ch
     */
     

     // Rust loop
     let mut number1 = 0;
     // infinite loop starts here
     loop {
          number1 += 1;
          println!("Loop {}", number1);
          
          if number1 >= 10 {
               // exit the loop
               println!("\n");
               break;
          }
     }

     // Rust while Loop
     // while condition {
     //      // code block
     // }
      
     // // code block outside while loop

     // outer loop counter
     let mut i = 1;
    
     // outer loop
     let mut i = 1;
    
     // start of outer loop
     while i <= 5 {
          let mut j = 1;
             
          // start of inner loop
          while j <= 5 {
              j += 1;
             
              // condition to skip iteration of the inner loop
              if j == 3 {
                  // move to the next iteration of the inner loop
                  continue;
               }
             
              print!("*");
          }

          println!("");
         
          i += 1;
     }

     //// Rust for Loop

     // The for loop in Rust is used to iterate a range of numbers. The syntax of for loop is:
     
     // for variable in lower_bound_number..upper_bound_number {
     //     // code block
     // }

     let mut sum = 0;
    
     // for loop to iterate over first 10 natural numbers
     for i in 1..11 {
          sum += i;
          println!("Sum: {}  i : {}", sum, i);
     }
    
     //// Rust Array
     // array of natural numbers
     // initialization of array without data type
     let arr = [1, 2, 3, 4, 5];

     // initialization of array with data type
     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
     
     // initialization of array with default values
     let numbers: [i32; 5] = [3; 5];
    
     println!("\nArray of numbers = {:?}\n", numbers);

     //// Access Elements of Rust Array
     let colors = ["red", "green", "blue"];

     /*      
    colors[0] - access the element at index 0 (first element)
    colors[1] - access the element at index 1 (second element)
    colors[2] - access the element at index 2 (third element)
    */
     // accessing element at index 0
     println!("1st Color: {}", colors[0]);

     // accessing element at index 1
     println!("2nd Color: {}", colors[1]);

     // accessing element at index 2
     println!("3rd Color: {}\n", colors[2]);

     //// Mutable Array in Rust
     // create a mutable array in rust
     let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

     // change the value of the 3rd element in the array
     numbers[2] = 0;
    
     println!("changed array = {:?}\n", numbers);
     
     //// Looping Through an Array in Rust
     
     // loop through an array to print its index and value
     for index in 0..3 {
          println!("Index: {} -- Value: {}", index, colors[index]);
     }
}
