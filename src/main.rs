

#[allow(unused_variables, unused_mut)]
const ID_1: i32 = 4; // define a global constant variable

fn main() {
   
 //explicitly define an integer
 let a:i32 = 24;
 let b:u64 = 23;
 let c:usize = 26;
 let d:isize = 29;
 //print the values
 println!("a: {}", a);
 println!("b: {}", b);
 println!("c: {}", c);
 println!("d: {}", d);

  //implicitly define an integer
  let a = 21; 
  let b = 1;
  let c = 54;
  let d = 343434;
  //print the variable
  println!("a: {}", a);
  println!("b: {}", b);
  println!("c: {}", c);
  println!("d: {}", d);

   //explicitly define a float type
   let f1:f32 = 32.9;
   let f2:f64 = 6789.89;
   println!("f1: {}", f1);
   println!("f2: {}", f2);

    //implicitly define a float type
    let pi = 3.14;
    let e = 2.17828;
    println!("pi: {}", pi);
    println!("e: {}", e);

    //explicitly define a bool
    let is_bool:bool = true;
    println!("explicitly_defined: {}", is_bool);
      // assign a boolean value
      let a = true;
      let b = false;
      println!("a: {}", a);
      println!("b: {}", b);
       // get a value from an expression
    let c = 10 > 2;
    println!("c: {}", c);

    // explicitly define character
    let char_1:char = 'e';
    println!("character1: {}", char_1); 
      // implicitly define
      let char_2 = 'a';
      let char_3 = 'b';
      println!("character2: {}", char_2);
      println!("character3: {}", char_3);

       // explicitly define string
    let str_1:&str = "Rust Programming";
    println!("String 1: {}", str_1); 

      // implicitly define
    let str_2 = "Rust Programming";
    println!("String 2: {}", str_2);

     //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   // initialize an array of size 4 with 0
   let arr1 = [0 ; 4]; 
  
   //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr[0]);
   // initialize an array of size 4 with 0
   let arr1 = [0; 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr1[0]);

   //define a mutable array of size 4
   let mut arr:[i32;4] = [1, 2, 3, 4]; 
   println!("The value of array at index 1: {}", arr[1]);
   arr[1] = 9;
   println!("The value of array at index 1: {}", arr[1]);

   //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   //Using debug trait
   println!("\nPrint using a debug trait");
   println!("Array: {:?}", arr);

    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    // print the length of array
    println!("Length of array: {}", arr.len());


     //define an array of size 4
     let arr:[i32;4] = [1, 2, 3, 4]; 
     //define the slice
     let slice_array1:&[i32] = &arr;
     let slice_array2:&[i32] = &arr[0..2];
     // print the slice of an array
     println!("Slice of an array: {:?}", slice_array1);
     println!("Slice of an array: {:?}", slice_array2);

     let arr:[i32;4] = [1,2,3,4]; 
     let slice_array:&[i32] = &arr[0..1];
     println!("Slice of an array : {:?}", slice_array);

      //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // define a tuple with type annotated
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // access value of a tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // get individual values out of tuple
    let (w ,x, y, z) = person_data;
    //print values
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);

     //define a tuple
     let mut person_data = ("Alex", 48, "35kg", "6ft");
     //print the value of tuple
     println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
     //modify the value at index 0
     person_data.0 = "John";
     //print the modified value
     println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);

      //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);

    const ID_2: u32 = 3; // define a local constant variable
    println!("ID:{}", ID_1); // print the global constant variable
    println!("ID:{}", ID_2); // print the local constant variable

    //challenge, define an array
    let arr:[i32;6] = [0, 2, 4, 6, 8, 10]; 
    // print the values of array
    println!("{},{},{},{},{},{}", arr[0],arr[1],arr[2],arr[3],arr[4],arr[5]);

    //challenge Declare a tuple
    let persons = ("Alex", 21, "Abe", 22, "Anna",23);
    //print the value of tuple
    println!("{}:{}, {}:{}, {}:{}", persons.0,persons.1,persons.2,persons.3,persons.4,persons.5);




}
 