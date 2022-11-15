
// Parameterless function with no return
fn foo() {
    println!("foo");
}

// Parameterless function with return type bool
fn bar() -> bool {
    // return via expression
    true
}

// Function with parameters and return type
fn foobar(value: u8, label: char) -> bool {
    println!("{} {}", value, label);

    return false;
}

// Function taking an ownership of heap-based object
fn take_ownership(text: String) {
    println!("{}", text);
}

// Function taking a reference to heap-based object
fn take_reference(text: &String) {
    println!("{}", text);
}

// Function taking a mutable reference to heap-based object
fn take_mutable_reference(text: &mut String) {
    text.push_str("!");
    println!("{}", text);
}

// Function returning dangling reference
// This won't even compile: missing lifetime specifier
// fn dangling_reference() -> &String {
//     &String::from("Dangling");
// }

fn main()
{
    println!("Rust basics");

    // Variables and mutability
    {
        // Immutable variable
        let x = 5;
        
        // Won't compile - immutable variable
        //x = 6;

        // Mutable variable - can be changed
        let mut y = 10;
        println!("{y}");

        y = x;
        println!("{}", y);

        // Shadowing - variable can shadow previously variable with the same name!
        let y = String::from("text");
        println!("{}", y);

        // Constants - always immutable
        const PI: f64 = 3.14159265359;
        println!("{PI}");
    }

    // Data types
    {
        // Number literals
        let decimal : u64 = 100_000;
        println!("{}", decimal);

        // Binary literal
        let binary = 0b11110100;
        println!("{}", binary);

        // Octal literal
        let octal = 0o731;
        println!("{}", octal);

        // Hex literal
        let hex = 0xfca1;
        println!("{}", hex);

        // Byte literal
        let byte = b'z';
        println!("{}", byte);

        // Overflow detection
        // Won't compile: attempt to compute `u8::MAX + 1_u8`, which would overflow
        //let sum : u8 = 255 + 1;

        // Will panic with 'Should not overflow!'
        //let sum : u8 = (255_u8).checked_add(1).expect("Should not overflow!");

        // Will work fine
        let sum : u8 = (254_u8).checked_add(1).expect("Should not overflow!");
        println!("{}", sum);

        // Bool (1 byte) and char (4 bytes!)
        let flag: bool = false;
        if !flag {
            let c: char = 'X';
            println!("{}, {}", flag, c);
        }

        // Tuples
        let tuple: (i32, char, f64) = (255, 'A', 1.23);

        // Destructuring the tuple
        let (x, y, z) = tuple;
        println!("{}, {}, {}", x, y, z);

        // Accessing tuple via an index - prints A
        println!("{}", tuple.1);

        // Array
        let array = ['a', 'b', 'c', 'd'];
        println!("{}, {}", array[0], array[3]);

        // Won't compile - index out of bounds: the length is 4 but the index is 4
        //println!("{}", array[4]);

        // Array with given type and size
        let array: [u8; 3] = [0, 128, 255];
        for element in array {
            println!("{}", element);
        }
    }

    // Functions
    {
        foo();
        println!("{}", bar());
        foobar(5, 'C');
    }

    // Control flows
    {
        // If
        let mut idx = 5;
        if idx == 0 {
            println!("Zero index");
        }
        else {
            println!("Index: {}", idx);
        }

        let flag: bool = if idx > 0 { true } else { false };
        println!("Flag: {}", flag);

        // Loop can return!
        let result = loop {
            println!("{}", idx);
            idx = idx - 1;

            if idx == 0 {
                break 'Q';
            }
        };
        println!("Loop result: {}", result);

        // Loop labels - to break specific loop
        let mut counter1 = 0;
        let mut counter2 = 5;
        'top_loop: loop {
            println!("counter1: {}", counter1);
            counter1 += 1;

            loop {
                println!("counter2: {}", counter2);
                counter2 -= 1;

                if counter2 < 3 {
                    break;
                }
            }

            if counter1 > 2 {
                break 'top_loop;
            }
        }

        // While
        let mut idx = 3;
        while idx > 0 {
            println!("{}", idx);
            idx -= 1;
        }

        // For
        let array = ['a', 'b', 'c'];
        for element in array {
            println!("{}", element);
        }

        // For
        for element in 5..7 {
            println!("{}", element);
        }
    }

    // Ownership
    {
        let s1 = String::from("Literal");
        let s2 = s1;

        // Won't compile: value borrowed here after move
        //println!("{}", s1);
        println!("{}", s2);

        // Deep copy - cloning
        let s3 = s2.clone();
        println!("{}", s3);

        let s4 = String::from("Object");
        take_ownership(s4);
        // Won't compile: borrow of moved value: `s4`
        //println!("{}", s4);

        // Just a reference
        let s5 = String::from("Reference");
        take_reference(&s5);
        println!("{}", s5);

        // Mutable reference
        let mut s6 = String::from("Mutable Reference");
        take_mutable_reference(&mut s6);

        let mut s7 = String::from("Another mutable");
        let s8 = &mut s7;

        // Won't compile: cannot borrow `s7` as mutable more than once at a time
        //let s9 = &mut s7;
        println!("{}", s8);

        // Here it will work - s9 is out of scope before creating next reference
        {
            let s9 = &mut s7;
            println!("{}", s9);
        }

        let mut s10 = &mut s7;
        println!("{}", s10);

        // Mixing mutable and immutable references
        let s11 = &s10;
        let s12 = &s10;

        // Won't compile: cannot borrow `s10` as mutable because it is also borrowed as immutable
        //let s13 = &mut s10;

        println!("{}, {}", s11, s12);

        // Now works
        let s13 = &mut s10;
        println!("{}", s13);

        
    }

}
