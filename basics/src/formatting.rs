pub fn run() {
    // Normal formatting
    println!("Hello {}", "Andrew");

    // Position formatting
    println!("Hello {1} and {0}", "Adriana", "Andrew");

    // Named formatting
    println!("Hello {O} and {A}", A = "Adriana", O = "Andrew");

    // Traits
    println!("Number: {}\n -> binary {:b}\n -> hexadecimal: {:x}\n -> octal: {:o}", 10, 10, 10, 10);

    // Combine traits
    println!("All combine: {:?}", (true, 12, "Good"));

    // We can padd numbers with zeros -> šipka ukazuje kam bude číslo posunuto
    println!("Zeros before number: {number:0>5}, and zeros after number: {number:0<5}", number = 7);

    // To samé lze napsat takto
    println!("Zeros before number: {number:0>width$}, and zeros after number: {number:0<width$}\n\n", number = 7, width = 5);

}