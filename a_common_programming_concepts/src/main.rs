use std::io;
fn main() {
    /*
    ========= 12/09/2022 Review Questions for next day ==========
    1. Are variables mutable or immutable by default in Rust?
    2. How would you create a mutable variable in rust?
    3. How would you write a constant in Rust?
    4. What is the naming convention for writing constants in Rust?
    5. When declaring a constant, can you use an expression, eg 60 * 60 * 3 on the right side?
    6. When declaring a constant, what is absolutely necessary to do on the left side?
    7. Can you explain shadowing in Rust?
    8. Can you write a simple example of shadowing?
    9. Can you write a slightly more complex example of shadowing?
    */
    // question 1 & 2
    let name_a = "Barry";
    println!("Hello, world!, {name_a}");
    let mut name_b = "Bob";
    println!("Hello {name_b}");
    name_b = "Sarah";
    println!("Hi {name_b}");

    //question 3, 4 and 5
    const HOURS_IN_REGULAR_YEAR: u32 = 365 * 24;
    println!("There are {HOURS_IN_REGULAR_YEAR} hours in a regular year");

    // question 6
    /* Shadowing is when the same variable name, using let is used to store a value of a different type,
    but the variable remains of immutable type. As below for question 8 */
    let minutes_in_reg_year = "525600";
    println!("There are {minutes_in_reg_year} minutes in a regular year and ");
    let minutes_in_reg_year: u32 = minutes_in_reg_year.parse().expect("Parse error!");
    let seconds_in_reg_year = minutes_in_reg_year * 60;
    println!("hence there are {seconds_in_reg_year} seconds in a regular year.");
    // question 7
    let x = 90;
    print!("x is first this value, {x}.");
    let x = -90;
    {
        let x = x * -2;
        println!("\nIn this scope x is {x}.");
    }
    print!("But now x is {x} in the outer scope.");

    /*
    ========= 13/09/2022 Review Questions for next day ==========
    1. Do you always have to write a type annotation?
    2. How can you write a type annotation? Please give a code example?
    3. What is a scalar type in Rust and can you name the 4 types?
    4. What is the difference between signed and unsigned integers?
    5. What and how is _ used in integers? Give an example?
    6. What is a common place you might use ‘usize’ or ‘isize’?
    7. What is integer overflow?
    8. What is a standard floating point number?
    9. Are all floating point numbers signed or unsigned?
    */

    /* == Answers 14/09 == */
    //1. No, some type inference is done by Rust but all types must be known at compile time
    //2 & 3.
    let x: u32 = 78_954; //integer
    let y: f64 = 456.45; //float
    let c: char = 'c'; //char *note the single quotes
    let t: bool = true; // boolean
    println!(
        "These are the four scalar types in Rust.
    1. integer --> {x}
    2. float   --> {y}
    3. char    --> {c}
    4. boolean --> {t}"
    );
    //4. Signed integers are integers that accept a negative number where as unsigned are 0 or positive.
    let _signed: i16 = -4;
    let _unsigned: u16 = 7;
    //5. _ is used as a convenience space separator for readability --> see x: u32 above.
    //6. usize or isize are often used for indexing a collection
    //7. integer overflow is when by some programming calculation the integer exceeds it's bit size, it is dealt with
    //   in various ways but most throw an error and should be dealt with gracefully, there are methods available for
    //   that purpose.
    //8 & 9. See above for standard floating point, f64 and they are always signed.

    /*
    ========= 14/09/2022 Review Questions for next day ==========
    1.  The char type is specified how?
    2.  Are they unicode? Are emoji, kanji etc available?
    3.  Other that scalar types what are the other primitive types in Rust?
    4.  Which type is fixed in length and cannot grow or shrink? Can you write an example?
    5.  Is destructuring available to this type? Can you write an example?
    6.  In python we access a value in this type using x[0] syntax, write example for Rust.
    7.  What type represents and empty tuple, ()?
    8.  Arrays and tuples differ in what ways in Rust?
    9.  Are Arrays fixed or flexible in length?
    10. If you want a flexible array, what type would you use?
    11. Are arrays allocated to the stack or the heap?
    12. How can you explicitly write the type of an array?
    13. If you want an array of same values how could you write that? Give an example.
    14. How can you access the second value in an array, please write the code.
    15. If you try to access an array value out of index, what happens if at runtime?
    16. The above behaviour is the same as C? Why is this a good thing?
    */

    /* == Answers 15/09 & 16/09 == */
    // 1 & 2. Char types are specified with single quotes as below and are unicode.
    let _b: char = '4';

    //3. The other primitive types are compount types.

    //4. Tuples are of fixed length and cannot grow or shrink. Type annotations are optional.
    let _t = (1, 'b', "trust");
    let tup: (u8, char, bool) = (1, 't', true);

    //5. Yes destructuring is available
    let (_k, _d, _h) = tup;
    // note
    let (u, ..) = tup;
    println!("u is --> {u}");
    println!("tup is ==> {tup:?}"); // note the pretty print :?

    //6. Another way to access the values in a tuple is as follows.
    let c = tup.2;
    println!("c is --> {c}");

    //7. The 'unit' type, returned by an empty return, or empty value is written as follows
    let _unit_t = ();

    //8. Arrays cannot hold values of different types where as tuples can. Array are also accessed
    // in a different way

    //9. Arrays, like tuples are fixed in length

    //10. Flexible array like strucktures are provided by the vector type.
    let mut flex_arr = vec![5, 4, 3, 2, 1];
    println!(
        "Length: {}, Capacity : {}",
        flex_arr.len(),
        flex_arr.capacity()
    );
    flex_arr.pop();
    println!("Vector now --> {flex_arr:?}");

    //11. Arrays are allocated to the stack and are a useful structure to hold
    // collections of known fixed length, such as 'days_of_the_week'

    //12.  Array types are written as follows
    let arr: [i32; 5] = [10, 9, 8, 7, 6];

    //13: Array of same values has a short cut syntax which looks like this
    let _same: [i32; 5] = [4; 5]; // same as let _same: [i32; 5] = [4, 4, 4, 4, 4];

    //14. Choose the second val in array as below, similar to js and python
    let _second = arr[1];

    //15. If you try to access a value out of index, a runtim error --> panic occurs. <-- Good!!

    //16. In other languages, such as C, there is a possibility that invalid memory is accessed.
    // Rust prevents this from happening and protects against this point of possible security error.

    /*  ========= 15/09/2022 && 16/09/2022 Review Questions for next day ==========
    1. What case does Rust use for variables and functions?
    2. Are type annotations required for all parameters?
    3. What is the dif between statements and expressions in Rust?
    4. In python, x = y = 6 is valid code. How about Rust? Explain!
    */

    // == Answers 19/09  ==
    // 1. snake case is used.
    //2. Yes type annotations are required for all parameters in Rust.
    //3. Statements end with a semi colon and do not evaluate to anything where
    //   as  expressions do evaluate to something
    //4. y does not return a value so there is nothing for x to bind to.

    // ========= Questions 19/9 ===========
    // 1. Are return types necessary with functions in Rust, please give an example.
    // 2. Is return keyword always necessary in Rust? Examples please.
    // 3. Is the following code correct? If so, say why and if not can you correct it?
    // fn plus_one(x: i32) -> i32 {
    //      x + 1;
    //  }
    // ------- Control Flow ---------
    // 4. Can you write a basic function which checks a number and has three optional paths, greater
    // than 5, less than 5 or equal 5 and depending on the umber entered by the user & let the user know.
    // 5. Is this valid code in Rust? Please explain and correct if necessary.
    // fn is_zero(x: i32) -> bool {
    //    if x {
    //      true
    //    } else {
    //        false
    //    }
    // }
    // 6. Is 'if' an expression or a statement? What are the implications in code.
    // 7. If you have too many else if expressions what could you use instead?

    // == Answers 20/09  ==
    // 1 & 2. Yes the return type is neccessary, except when it is the unit type ().
    // return types are not always necessary as functions are expresions so as in
    // plus_one below, as the semi colon is missing the function implicily returns x + 1
    // The code in 3. is incorrect as it returns () not i32 as there is a semi-colon. The
    // code below is correct.
    let result = plus_one(39);
    println!("Result of plus one --> {result} ");
    // 5.  Pointles function to illustrate branching
    pointless_function();
    // 6. The if expresssion is not correct because if expressions must be comparisons resulting in a boolean.
    // .  In the example above if x is if x:i32 where x is not a boolean but a signed 32 bit integer
    // Please see corrected is_zero below
    let num = is_zero(-8);
    println!("Number is zero == {num} ");

    //  ========= 20/09/2022 Review Questions for next day ==========
    // ------------Loops -------------
    //1. What three loop key words are there?
    //2. Can you give an example of the first kind of loop with key words 'continue' and 'break'
    //3. If you write code after break word what are you doing, please give an example.
    //4. If you label a loop, how might you use it?
    //5. Give an example of a while loop.
    //6. Give an example of a for loop.
    //7. Which loops tend to be most prevelant in Rust code and why?

    // ========= Example programs ===========
    // 1. Conver temperatures to fahrenheit or celsius
    let from_celsius = convert_temperature(30.0, 'c');
    println!("30 degrees C in fahrenheit is {from_celsius}");
    let from_fahrenheit = convert_temperature(101.9, 'f');
    println!("101.9 F in celsius is {from_fahrenheit}");

    let r1 = "string literal";
    let r2 = r1;
    // the following code works because r1 is a string literal so copy is called
    println!("this works, {r1}");
    println!("this works, {r2}");
} // fn main

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn pointless_function() {
    let mut entry = String::new();
    let number: i32 = loop {
        println!("Please input your number.");

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        let entry: i32 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break entry;
    };
    if number > 5 {
        println!("Number  {number} is greater than 5");
    } else if number < 5 {
        println!("Number  {number} is less than 5");
    } else {
        println!("Number is 5");
    };
    println!("Duh!!");
}

fn is_zero(x: i32) -> bool {
    if x == 0 {
        true
    } else {
        false
    }
}

fn convert_temperature(temp: f64, c_or_f: char) -> String {
    let mut result: f64 = 0.0;
    if c_or_f == 'c' {
        result = temp * 1.8 + 32.0;
    } else if c_or_f == 'f' {
        result = (temp - 32.0) * 0.5556;
    } else {
        println!("The second argument must be 'c' or 'f' to indicate the temperature entered.")
    }
    let result = result.to_string();
    return result;
}
