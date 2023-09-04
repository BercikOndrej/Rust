// Ukázka funkcí
pub fn run() {
    // Add function
    let a = 5;
    let b = 5.7;
    println!("Example of add function: {} + {} = {}", a, b, add(a,b));

    //  Reverse function
    let tupple = (true, 1);
    println!("Tupple: {:?} and reverse tuple by function: {:?}", tupple, reverse_tuple(tupple));

    // -----------------------------------------------------------------------------------------------------------------------------------------------------------
    // Methods and associated function
    println!("Methods and associated fuction example:");
    // Some functions are connected to a particular type. These come in two forms: associated functions,
    // and methods. Associated functions are functions that are defined on a type generally, while methods
    // are associated functions that are called on a particular instance of a type.

    struct Point {
        x: f64,
        y: f64,
    }
    
    // Implementation block, all `Point` associated functions & methods go in here
    impl Point {
        // This is an "associated function" because this function is associated with
        // a particular type, that is, Point.
        //
        // Associated functions don't need to be called with an instance.
        // These functions are generally used like constructors.
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
    
        // Another associated function, taking two arguments:
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    

    impl Rectangle {
        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // `self` gives access to the struct fields via the dot operator
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            // `abs` is a `f64` method that returns the absolute value of the
            // caller
            ((x1 - x2) * (y1 - y2)).abs()
        }
    
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
    
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
    
        // This method requires the caller object to be mutable
        // `&mut self` desugars to `self: &mut Self`
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
    
            self.p1.y += y;
            self.p2.y += y;
        }
    }


    // `Pair` owns resources: two heap allocated integers
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // This method "consumes" the resources of the caller object
        // `self` desugars to `self: Self`
        fn destroy(self) {
            // Destructure `self`
            let Pair(first, second) = self;
    
            println!("Destroying Pair({}, {})", first, second);
    
            // `first` and `second` go out of scope and get freed
        }
    }

    let rectangle = Rectangle {
        // Associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    //pair.destroy();
    // TODO ^ Try uncommenting this line

    // -----------------------------------------------------------------------------------------------------------------------------------------------------------
    // Closures -> uzávěry = funkce, které si pamatují své místo vzniku
    println!("Closures:");
    let outer_var = 42;
    
    // A regular function can't refer to variables in the enclosing environment
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO: uncomment the line above and see the compiler error. The compiler
    // suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // Once closure's type has been inferred, it cannot be inferred again with another type.
    //println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    // -----------------------------------------------------------------------------------------------------------------------------------------------------------
    // Funkce vyššího řádu
    println!("Higher order fuctions:");
     // Functional approach
     let upper = 1000;

     let sum_of_squared_odd_numbers: u32 =
     (0..).map(|n| n * n)                             // All natural numbers squared
          .take_while(|&n_squared| n_squared < upper) // Below upper limit
          .filter(|&n_squared| is_odd(n_squared))     // That are odd
          .sum();                                     // Sum them
 println!("functional style: {}", sum_of_squared_odd_numbers);

}

// -----------------------------------------------------------------------------------------------------------------------------------------------------------
// Ather function
fn add(first: i32, second: f64) -> f64 {
    // převedení typu
    // pokud nedáme za poslední výraz v bloku a tedy i ve funkci středník, tak se za návratovou hodnotu funkce zvolí výsledek výrazu na posledním řádku
    // Pokud se ale středník napíše, tak návratová hodnota je () = void
    first as f64 + second
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// Funkce mohou brát i tuply
fn reverse_tuple(pair: (bool, i32)) -> (i32, bool) {
    // převzet členů páru a piřazení je svým proměnným
    let (bool_param, str_param) = pair;
    (str_param, bool_param)
}