fn main() {
    println("Understanding Ownership")
}
// ================ Ownership ========================
// 1. Can you explain the diff between the stack and the heap
//2. What are the three rules of ownership in Rust
//3. Can you explain the four parts of this string 'let s1 = String::from("hi")?
//4. Can you explain what happens in the case we say 'let s2 = s1' in terms of the stack and the heap?
//5. Can you explain the error in the following programme?
/*
fn error_here_one() {
    let s1 = String::from("hi");
    let s2 = s1;
    println!("The way to casually say hello in English is to say {s1}")
}
*/
//6. In other languages this would be called a 'shallow copy'. In Rust it is called? Why?
//7. Using a method for deep copying 's1' how could you correct the above function?
//8. Is this ideal? Why not?
//9. Why then is this code ok?
/*
fn no_error() {
    let i1 = 4;
    let i2 = i1;
    println!("We can use both i1:{i1} and i2{i2}")
}
*/
//10. Explain the 'drop' function?
//11. Explain the 'Copy' trait? Which types can implement it?
//12. Can you annotate each of the following lines in each program  to say
// exactly what is happening
/*
// a.  =============================
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
// b.  =============================
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

*/

// ================ References and Borrowing ========================
//1. What is a reference, and how does it differ from a pointer?
//2. How do you indicate a reference?
//3. Are references mutable by default?
//4. How would you change a reference to be mutable?
//5. How many mutable references to a variable can you have at one time? Why?
//6. What is a data race?, What three behaviours cause data races?
//7. How does rust avoid data races?
//8 What is a Non Lexical Lifetimes?
//9. Can you explain why this is a dangling reference?
/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/
