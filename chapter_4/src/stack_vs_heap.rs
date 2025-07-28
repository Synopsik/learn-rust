fn main() {

    // When a function is called, a stack frame is pushed to the stack
    // It holds the function’s local variables, parameters, and return address
    stack_memory();

    let returned_string = heap_memory();
    println!("{returned_string}");
}

fn stack_memory() {
    println!();
    println!("Stack Memory");
    println!("------------------------------------------------");

    let c = "!"; // var c, var b, and var a are pushed onto the stack
    let b = "world";
    let a = "Hello"; // LIFO - var a, b, and c when taken off the stack
    println!("{a} {b}{c}");
    let s = "Hello world!";

    {
        let r1 = &s; // Reference to read-only string literal location `a`
        println!("{r1}");
    } // Var r1 goes out of scope here.
    // Popped and released, we can now make new refs to it
} // Stack frame is popped along with all variables in ascending order
// The var's a, b, and c do NOT exist outside the function's scope

fn heap_memory () -> String {
    println!();
    println!("Heap Memory");
    println!("------------------------------------------------");

    let mut string = String::from("Hello ");
    string.push_str("world");
    string.push_str("!");

    println!("{string}");

    let r1 = &string;
    let r2 = &string;
    // let r3 = &mut string; // Error
    // Can't create a mutable ref when we already have an immutable ref

    println!("{r1} | and | {r2}"); // Pops r1 and r2 from stack-frame
    // Once immutable refs are used, we can create a mutable ref
    //
    let r3 = &mut string;
    println!("{r3}");

    string // Local variables are destroyed after the function's scope
    // must return var and NOT ref,
    // data must be transferred out of the function's scope
    // a ref &string would create a dangling pointer
} // local string var is destroyed after {} closes,
// data is copied to calling statements input