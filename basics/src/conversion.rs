pub fn run() {
    //  Aesy conversion from str to String
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("str: {} and String from str: {}", my_str, my_string);

    // Můžeme si ale definovat i konverzi vlastního typu
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Teď to samé pomocí klíčového slova Into -> tuto funkci zavoláme na typ A, který chceme přiřadit jinému typu (typ B), který ale musíme definovat při inicializaci jako zde Number
    // tento typ B ale musí mít implemntovanou funkci from, kde je definováné jak převod z typu A do typu B probíhá a jak ho lze provést
    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // To samé platí pro funkce TryFrom a TryInto, které to ale nejprve zkusí a pokud typ převést nejde, tak vyhodí Error -> více v knize
    #[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {

    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Converting to String
    //  Pro převod nějakého typu do Stringu je nejjednoduší implementovat funkci ToString()
    // Místo toho abychom to dělali takto na přímo, tak raději budeme implementovat fmt::Display trait (vlastnost), která automaticky poskytuje funkci ToString 
    // a umožňuje také formátování pomocí funkce print

    use std::fmt;

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parisng a string to a number
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}\n\n", sum);
}