pub fn run() {
    let logical: bool = true;
    let integer: i32 = 17;
    let a_float: f64 = 0.5;
    let mut mutable_int = integer;

    // Rust have type String and str ->
    // String is the dynamic heap, so use it for storing data, which will be modify
    //  str is inmutable string for constants and something like this
    println!("Mutable int: {}", mutable_int);
    mutable_int = 7;

    println!("Bool: {0}, Float(64): {1}, Int(32): {2}, Mutable int after change: {3}",
        logical,
        a_float,
        integer,
        mutable_int
    );

    // Takzéž jde udělat, že typ proměnné přiřadím hned při inicializaci pomocí suffixu
    let possitive_number = 1u8;
    let float_n = 4.5f64;
    println!("size of `possitive_number` in bytes: {}", std::mem::size_of_val(&possitive_number));
    println!("size of `float_n` in bytes: {}", std::mem::size_of_val(&float_n));

    // Casting
    let float_number: f64 = 72.5;
    println!("Casting numbers: from {} to {} (unsigned int) and {} (char)", float_number, float_number as u32, (float_number as u8)as char);

    // Type interference
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("Our vector: {:?}", vec);

    // New name for existing types
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // Definition of unit (=void) variable
    let unit = ();

    // Konstanta
    const ID: i32 = 01;
    println!("Konstanta: {}", ID);

    // Tuples = n-tice -> pouze struktura z různými typy
    println!("Tuple: {:?}", (true, 49, "abc", 'd'));

    // Může být i rozdělena a přiřazena jednotlivým proměnným
    fn give_me_numbers() -> (u32, u32, u32) {
        (1, 2 , 3)
    }
    let (first, second, third) = give_me_numbers();

    println!("First number: {}", first);
    println!("Second number: {}", second);
    println!("Third number: {}", third);

    // Array definition with initialize -> type and length
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", array[0]);
    println!("Second element of the array: {}", array[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", array.len());

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];

    // Slices (kousky) = podobná struktura jako pole ale jejich délka není známa během kompilace. Je to dvousložková struktura, kdy první data obsahuje pointer na data
    // a druhá na délku slice
    // Používají se na půjčení vybrané části pole.¨
    // Značíse &[T]

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the selection of array as a slice.");
    let s = &array[1 .. 4];
    println!("Slice: {:?}\n\n",s);
    
        
}