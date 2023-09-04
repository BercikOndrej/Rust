// A crate is a compilation unit in Rust. Whenever rustc some_file.rs is called,
// some_file.rs is treated as the crate file. If some_file.rs has mod declarations
// in it, then the contents of the module files would be inserted in places where mod
// declarations in the crate file are found, before running the compiler over it. 
// In other words, modules do not get compiled individually, only crates get compiled.

// A crate can be compiled into a binary or into a library. By default, rustc will produce
// a binary from a crate. This behavior can be overridden by passing the --crate-type flag to lib.

// --------------------------------------------------------------------------------------------------------------
// Crating library crate
pub fn public_function() {
    println!("called cate's `public_function()`");
}

fn private_function() {
    println!("called crate's `private_function()`");
}

pub fn indirect_access() {
    print!("called crate's `indirect_access()`, that\n> ");

    private_function();
}

// Poté vyhodnocením těchto výrazů vytvoříme knihovnu zvanou "libcrate.lib"
// $ rustc --crate-type=lib crate.rs
// $ ls lib*
// library.rlib


// Libraries get prefixed with "lib", and by default they get named after their
// crate file, but this default name can be overridden by passing the --crate-name
// option to rustc or by using the crate_name attribute.

// Jak použít knihovnu, dodatečné konfigurace, použití knihovny napsané v jiném jazyce najdu na stránkách rustu