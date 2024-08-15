pub fn data_type() {
    let x: u32 = 15;
    let y: u32 = x + 10;
    println!("Value of y is {y}");

    let z: i32 = -15;
    println!("Value of negative {z}");

    let name: &str = "Gaurav";
    println!("The user name is {name}");

    let marks: f64 = 15.5;
    println!("Printing a float {marks}");

    let age_above: bool = true;
    println!("The user above 18: {age_above}");

    // Tuple
    let user_data: (&str, u32, char) = ("Gaurav", 20, 'M');
    let (name, age, gender) = user_data;
    println!("User Data: name={name}, age={age}, gender={gender}");

    // Mutable
    let mut number: i32 = 10;
    number += 1;
    number += 1;
    println!("The Number: {number}");

    // Array
    let arr: [i32; 3] = [1, 2, 3];
    let first_e: i32 = arr[0];
    println!("Print the array: {first_e}");

    // String
    let mut new_str: String = String::from("hello");
    new_str.push_str(", world!");
    println!("The mut new_str: {new_str}");
}
