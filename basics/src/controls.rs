pub fn run() {
    println!("If/else blok:");
    // If/else blok -> jako v ostatních jazycích ale podmínka nemusí být ohraničená závorky
    let n = 5;

    if n < 0 {
        println!("{} is negative.", n);
    } else if n > 0 {
        println!("{} is positive.", n);
    } else {
        println!("{} is zero.", n);
    }

    // Nekonečný cyklus pomocí klíč. slova "loop"
    println!("Loop blok:");

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    // Z nekonečného loopu lze i předávat hodnotu
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // Nested loops -> musí se používat labels abychom věděli, ze kterého loopu chceme vyskočit
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // While cyclus
    // A counter variable
    println!("While blok:");

    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // For loop with range
     // `n` will take the values: 1, 2, ..., 100 in each iteration
     for n in 1 .. 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Ta samá podmínka se dá zapsat takto:   for n in 1 .. 101 ->  for n in 1 .. =100

    // Iteration
    println!("Iteration blok:");
    // iter - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    // iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    println!("names: {:?}", names);

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    // Další  match patern is the same as switch
    println!("Match block is the same as switch blok.\n\n");
}

