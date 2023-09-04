pub fn run() {
    // Konstanty -> const and static
    // const vyložene pro konstanty
    // static je pro proměnné, které jsou portencionálně mutable a budou se postupem času měnit -> souvisí to z životností proměnné kvůli paměti
    const DEFAULT_LENGTH: u32 = 100;
    static DEFAULT_CITY_NAME: &str = "Prague";
    
    println!("\nConstants:");
    println!("Constant DEFAULT_LENGTH: {} and static DEFAULT_CITY_NAME: {}", DEFAULT_LENGTH, DEFAULT_CITY_NAME);

    // Structs -> 3 typy: Tuples, které už znám, obyčejne C structs a Unit structs, které nemájí žádné "fieldy" a jsou užitečné pro dědičnost
    // This directive is for boproblem printing
    #[derive(Debug)]
    struct Person {
        // Zde se může měnit jméno, takže použiji String místo str, který je inmutable
        name: String,
        age: u8,
    }

    // Struct initialization
    let my_person = Person {
        name: String::from("Andrew"),
        age: 23
    };

    println!("\nStructs:");

    println!("Structure fields printing: name: {} and age: {}", my_person.name, my_person.age);
    println!("Whole structure printing: {:?}", my_person);

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
 
    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
 
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Strukturu můžeme vytvořit i bez vytváření proměnné
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 10.5, y: 15.2 },
        bottom_right: bottom_right,
    };

    println!("\nEnums:");
    enum Number {
        Zero,
        One,
        Two,
        Five,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("It's only print index of enum. Let's see on number five:");
    println!("five is {}\n\n", Number::Five as i32);


}