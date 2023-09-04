use std::mem;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right 
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}


pub fn run() {
    // Boxes
    println!("\nBoxes:");

     // (all the type annotations are superfluous)
    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytes on the stack",
             mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&rectangle));

    // box size == pointer size
    println!("Boxed point occupies {} bytes on the stack",
             mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on the stack",
             mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on the stack",
             mem::size_of_val(&box_in_a_box));

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
             mem::size_of_val(&unboxed_point));

    // ------------------------------------------------------------------------------------------------------------------------------
    // Strings
    println!("\nStrings:");
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string, hence no new
    // allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

    // ------------------------------------------------------------------------------------------------------------------------------
    // Vectors
    println!("\nVectors:");
     // Iterators can be collected into vectors
     let collected_iterator: Vec<i32> = (0..10).collect();
     println!("Collected (0..10) into: {:?}", collected_iterator);
 
     // The `vec!` macro can be used to initialize a vector
     let mut xs = vec![1i32, 2, 3];
     println!("Initial vector: {:?}", xs);
 
     // Insert new element at the end of the vector
     println!("Push 4 into the vector");
     xs.push(4);
     println!("Vector: {:?}", xs);
 
     // Error! Immutable vectors can't grow
    //  collected_iterator.push(0);
     // FIXME ^ Comment out this line
 
     // The `len` method yields the number of elements currently stored in a vector
     println!("Vector length: {}", xs.len());
 
     // Indexing is done using the square brackets (indexing starts at 0)
     println!("Second element: {}", xs[1]);
 
     // `pop` removes the last element from the vector and returns it
     println!("Pop last element: {:?}", xs.pop());
 
     // Out of bounds indexing yields a panic
    //  println!("Fourth element: {}", xs[3]);
     // FIXME ^ Comment out this line
 
     // `Vector`s can be easily iterated over
     println!("Contents of xs:");
     for x in xs.iter() {
         println!("> {}", x);
     }
 
     // A `Vector` can also be iterated over while the iteration
     // count is enumerated in a separate variable (`i`)
     for (i, x) in xs.iter().enumerate() {
         println!("In position {} we have value {}", i, x);
     }
 
     // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
     // over in a way that allows modifying each value
     for x in xs.iter_mut() {
         *x *= 3;
     }
     println!("Updated vector: {:?}", xs);

    //  --------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // Option
    println!("\nOption type:");
    // An integer division that doesn't `panic!`
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            // Failure is represented as the `None` variant
            None
        } else {
            // Result is wrapped in a `Some` variant
            Some(dividend / divisor)
        }
    }
    
    // This function handles a division that may not succeed
    fn try_division(dividend: i32, divisor: i32) {
        // `Option` values can be pattern matched, just like other enums
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed!", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            },
        }
    }
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    // println!("{:?} unwraps to {:?}", none, none.unwrap());

    //  --------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // Result
    println!("\nResult type:");
    mod checked {
        #[derive(Debug)]
        enum MathError {
            DivisionByZero,
            NonPositiveLogarithm,
            NegativeSquareRoot,
        }
    
        type MathResult = Result<f64, MathError>;
    
        fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }
    
        fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }
    
        fn ln(x: f64) -> MathResult {
            if x <= 0.0 {
                Err(MathError::NonPositiveLogarithm)
            } else {
                Ok(x.ln())
            }
        }
    
        // Intermediate function
        fn op_(x: f64, y: f64) -> MathResult {
            // if `div` "fails", then `DivisionByZero` will be `return`ed
            let ratio = div(x, y)?;
    
            // if `ln` "fails", then `NonPositiveLogarithm` will be `return`ed
            let ln = ln(ratio)?;
    
            sqrt(ln)
        }
    
        pub fn op(x: f64, y: f64) {
            match op_(x, y) {
                Err(why) => panic!("{}", match why {
                    MathError::NonPositiveLogarithm
                        => "logarithm of non-positive number",
                    MathError::DivisionByZero
                        => "division by zero",
                    MathError::NegativeSquareRoot
                        => "square root of negative number",
                }),
                Ok(value) => println!("{}", value),
            }
        }
    }
    
    //  --------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // HashMap
    println!("\nHashMap type:");
    fn call(number: &str) -> &str {
        match number {
            "798-1364" => "We're sorry, the call cannot be completed as dialed. 
                Please hang up and try again.",
            "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
                What can I get for you today?",
            _ => "Hi! Who is this again?"
        }
    }
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley"); 

    // `HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }

    //  --------------------------------------------------------------------------------------------------------------------------------------------------------------------
    // Reference count = RC
    println!("\nRC type:");
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a is cloned to rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            
            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            
            println!("--- rc_b is dropped out of scope ---");
        }
        
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
    }
}