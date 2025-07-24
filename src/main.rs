fn main() {
    println!("Hello, world!");

    let age: u8 = 30; // 8 bit unsigned integer
    let mut number_one: i8 = 10; // 8 bit signed integer
    let number_two: i16 = 20; // 16 bit signed integer

    number_one -= number_two as i8; // casting to i8 (possible data loss)
    println!("{}", number_one);
    println!("I am {} years old.", age);

    // Boolean values
    let is_rust_fun: bool = true;
    let is_rust_bored = false;

    if is_rust_fun {
        println!("Rust is fun!");
    } else {
        println!("Rust is boring!");
    }
    if is_rust_bored {
        println!("Rust is boring!");
    } else {
        println!("Rust is not boring!");
    }

    // Comparison operators
    let a = 10;
    let b = 20;
    if a < b {
        println!("a is less than b");
    } else if a > b {
        println!("a is greater than b");
    } else {
        println!("a is equal to b");
    }

    // Loops
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    for i in 0..5 {
        println!("For loop iteration: {}", i);
    }

    loop {
        println!("This is an infinite loop");
        break; // breaks the loop
    }

    // Character type
    let c: char = 'e';
    println!("The character is: {}", c);

    // Comparasion of numbers
    let float_one: f32 = 3.14; // 16 bit floating point number
    let integer_one: u8 = 5; // 8 bit unsigned integer
    println!("Float one: {}, Integer one: {}, Sum: {}", float_one, integer_one, float_one + integer_one as f32);

    // Array and Slices
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4]; // slice from index 1 to 3
    println!("Array: {:?}", arr);
    println!("Slice: {:?}", slice);
    let chars: [char; 3] = ['a', 'b', 'c'];
    println!("Characters: {:?}", chars);

    // Tuples
    let tuple: (i32, f64, char, (i32, char)) = (42, 3.14, 'x', (-2, 'c'));
    let (x, y, z, _random) = tuple; // destructuring the tuple
    println!("Tuple values: x = {}, y = {}, z = {}, Random: {}", x, y, z, tuple.3.0);

    // Strings
    let mut string_one = String::from("Hello");
    let string_two = String::from("World");
    string_one.push_str(" "); // appending a space
    string_one.push_str(&string_two); // appending another string
    println!("Concatenated String: {}", string_one);

    // String Slices
    let str_slice: &str = &string_one[0..5]; // slicing the string
    println!("String Slice: {}", str_slice);

    // String Length
    let length = string_one.len();
    println!("Length of the string is: {}", length);

    // String Iteration
    for c in string_one.chars() {
        println!("Character: {}", c);
    }
    
    // String Comparison
    if string_one == "Hello World" {
        println!("The string is equal to 'Hello World'");
    } else {
        println!("The string is not equal to 'Hello World'");
    }

    // String Formatting
    let formatted_string = format!("{} {}", string_one, "from Rust!");
    println!("Formatted String: {}", formatted_string);

    // String Replacement
    let replaced_string = string_one.replace("World", "Rust");
    println!("Replaced String: {}", replaced_string);

    // String to Uppercase
    let upper_string = string_one.to_uppercase();
    println!("Uppercase String: {}", upper_string);

    // String to Lowercase
    let lower_string = string_one.to_lowercase();
    println!("Lowercase String: {}", lower_string);

    // String Trimming
    let trimmed_string = string_one.trim();
    println!("Trimmed String: '{}'", trimmed_string);

    // String Splitting
    let split_string: Vec<&str> = string_one.split(' ').collect();
    println!("Split String: {:?}", split_string);

    // String Contains
    if string_one.contains("World") {
        println!("The string contains 'World'");
    } else {
        println!("The string does not contain 'World'");
    }

    // String Ends With
    if string_one.ends_with("Rust!") {
        println!("The string ends with 'Rust!'");
    } else {
        println!("The string does not end with 'Rust!'");
    }

    // String Starts With
    if string_one.starts_with("Hello") {
        println!("The string starts with 'Hello'");
    } else {
        println!("The string does not start with 'Hello'");
    }

    // String Indexing
    if let Some(first_char) = string_one.chars().next() {
        println!("The first character of the string is: {}", first_char);
    } else {
        println!("The string is empty, no first character.");
    }

    // String Length Check
    if string_one.is_empty() {
        println!("The string is empty.");
    } else {
        println!("The string is not empty.");
    }

    // String Capacity
    let capacity = string_one.capacity();
    println!("The capacity of the string is: {}", capacity);

    // String Resizing
    string_one.reserve(10); // reserves additional space
    println!("String after reserving additional space: {}", string_one);
    string_one.shrink_to_fit(); // shrinks the capacity to fit the current length
    println!("String after shrinking to fit: {}", string_one);

    // String Reversal
    let reversed_string: String = string_one.chars().rev().collect();
    println!("Reversed String: {}", reversed_string);

    // String Comparison with &str
    if string_one == "Hello World from Rust!" {
        println!("The string is equal to 'Hello World from Rust!'");
    } else {
        println!("The string is not equal to 'Hello World from Rust!'");
    }

    // String Comparison with String
    let another_string = String::from("Hello World from Rust!");
    if string_one == another_string {
        println!("The string is equal to another_string");
    } else {
        println!("The string is not equal to another_string");
    }

    // String Iteration with Enumerate
    for (index, character) in string_one.chars().enumerate() {
        println!("Character at index {}: {}", index, character);
    }

    // String Iteration with Char Indices
    for (index, character) in string_one.char_indices() {
        println!("Character '{}' at byte index {}", character, index);
    }

    // String Iteration with Bytes
    for byte in string_one.bytes() {
        println!("Byte: {}", byte);
    }

    // String Iteration with Chunks
    for chunk in string_one.as_bytes().chunks(2) {
        println!("Chunk: {:?}", chunk);
    }

    // String Iteration with Lines
    let multiline_string = "Hello\nWorld\nfrom\nRust!";
    for line in multiline_string.lines() {
        println!("Line: {}", line);
    }

    // Boolean Operations
    let is_true = true;
    let is_false = false;
    println!("is_true AND is_false: {}", is_true && is_false); // logical AND
    println!("is_true OR is_false: {}", is_true || is_false); // logical OR
    println!("NOT is_true: {}", !is_true); // logical NOT
    println!("NOT is_false: {}", !is_false); // logical NOT
    println!("is_true XOR is_false: {}", is_true ^ is_false); // logical XOR
    println!("is_true NAND is_false: {}", !(is_true && is_false)); // logical NAND
    println!("is_true NOR is_false: {}", !(is_true || is_false)); // logical NOR
    println!("is_true XNOR is_false: {}", !(is_true ^ is_false)); // logical XNOR
    println!("is_true IMPL is_false: {}", !is_true || is_false); // logical implication
    println!("is_true EQ is_false: {}", is_true == is_false); // logical equality
    println!("is_true NEQ is_false: {}", is_true != is_false); // logical inequality
    println!("is_true LT is_false: {}", is_true < is_false); // less than
    println!("is_true GT is_false: {}", is_true > is_false); // greater than
    println!("is_true LEQ is_false: {}", is_true <= is_false); // less than or equal to
    println!("is_true GEQ is_false: {}", is_true >= is_false); // greater than or equal to
    println!("is_true EQV is_false: {}", (is_true && is_false) || (!is_true && !is_false)); // logical equivalence
    println!("is_true IMP is_false: {}", !is_true || is_false); // logical implication

    // if else if ladder
    let number = 15;
    if number < 10 {
        println!("Number is less than 10");
    } else if number < 20 {
        println!("Number is between 10 and 20");
    } else if number < 30 {
        println!("Number is between 20 and 30");
    } else {
        println!("Number is 30 or more");
    }

    // match statement
    match number {
        0 => println!("Number is zero"),
        1..=10 => println!("Number is between 1 and 10"),
        11..=20 => println!("Number is between 11 and 20"),
        21..=30 => println!("Number is between 21 and 30"),
        _ => println!("Number is greater than 30"),
    }

    // match with destructuring
    let tuple = (1, 2, 3);
    match tuple {
        (x, y, z) if x == y && y == z => println!("All elements are equal: {}, {}, {}", x, y, z),
        (x, y, z) if x < y && y < z => println!("Elements are in ascending order: {}, {}, {}", x, y, z),
        (x, y, z) if x > y && y > z => println!("Elements are in descending order: {}, {}, {}", x, y, z),
        (x, y, z) => println!("Elements are mixed: {}, {}, {}", x, y, z),
    }

    // match with enums
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }

    // match with Option
    let optional_value: Option<i32> = Some(10);
    match optional_value {
        Some(value) => println!("Optional value is: {}", value),
        None => println!("Optional value is None"),
    }

    // match with Result
    let result: Result<i32, &str> = Ok(20);
    match result {
        Ok(value) => println!("Result is Ok with value: {}", value),
        Err(error) => println!("Result is Err with error: {}", error),
    }

    // Function call
    println!("Area of the square: {}", area_of_square(5));

    // Function with parameters
    let side_length: u64 = 4;
    let area = area_of_square(side_length);
    println!("The area of the square with side length {} is: {}", side_length, area);

    // Function with return type
    let side: u64 = 6;
    let area_result = area_of_square(side);
    println!("The area of the square with side {} is: {}", side, area_result);

    let personel: Person = Person {
        name: "Batuhan".to_string(),
        surname: "Korkmaz".to_string(),
        age: 23,
        work: "Software Engineer".to_string()
    };

    println!("Name: {}, Surname: {}, Age: {}, Work: {}", 
             personel.name, personel.surname, personel.age, personel.work);

    // Vehicle instance
    let mut togg: Vehicle = Vehicle {
        name: "Togg".to_string(),
        capacity: 88.5,
    };
    
    let cap_of_togg: f32 = togg.get_capacity();
    println!("The capacity of {} is: {}", togg.name, cap_of_togg);
    togg.add_charge(20.0);
    println!("After charging, the capacity of {} is: {}", togg.name, togg.get_capacity());

    // enum example
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    let color: Color = Color::Green;
    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }

    // Using the debug trait
    println!("Color: {:?}", color);
}

// Function to calculate the area of a square
fn area_of_square(side: u64) -> u64 {
    let result: u64 = side * side;
    result
}

struct Person {
    name: String,
    surname : String,
    age: u8,
    work: String
}

struct Vehicle {
    name: String,
    capacity: f32
}

impl Vehicle {
    pub fn get_capacity(&self) -> f32 {
        self.capacity
    }

    fn add_charge(&mut self, value: f32) {
        self.capacity += value;

    }
}