
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

}
