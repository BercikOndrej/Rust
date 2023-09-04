use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Enum směrů, který řekne, do které strany lesa se máme dívat
enum Direction {
    Up,
    Right,
    Down,
    Left
}

// Funkce sloužící k převedení Stringu čísel na nezáporná čísla
fn convert_line(line: String) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).expect("It is not a digit") as u8)
        .collect()
}

fn load_map(path: &Path) -> Vec<Vec<u8>> {
    let display = path.display();

    // Otevření souboru
    let file = File::open(&path).expect(&format!("Program couldn't open {}.", display));

    // Načtení řádků do vektoru
    let reader = BufReader::new(file);
    reader.lines()
    .map(|line|convert_line(line.expect("Error in reading lines.")))
    .collect()
}

// Funkce, která vypočítá počet stromů
fn count_visible_trees(matrix: &Vec<Vec<u8>>) -> u32 {
    // Prvně spočítá okrajové stromy -> ty jsou všechny viditelné
    let border_trees_count = count_border_trees(&matrix);
    let inner_trees_count = count_inner_trees(&matrix);
    border_trees_count + inner_trees_count
}

// Funkce počítající okrajové stromy
fn count_border_trees(matrix: &Vec<Vec<u8>>) -> u32 {
    let height = matrix.len();
    // Víme, že se jedná o čtvercovou matici
    u32::try_from(4 * height - 4).unwrap()
}

// Funkce počítající vnitřní stromy
// -> funkce projde každyý strom kromě okrajových a zjistí zda je viditelný alespoň z jednoho směru
fn count_inner_trees(matrix: &Vec<Vec<u8>>) -> u32 {
    // let mut count: u32 = 0;
    // for row in 1 .. (matrix.len() - 1) {
    //     for col in 1 .. matrix[1].len() - 1 {
    //         if is_tree_visible(row, col, &matrix) {
    //             count+= 1;
    //         }
    //     } 
    // }
    // count

    // To samé ale přepsané iterátorem
    (1..matrix.len() - 1)
        .flat_map(|row| 
            (1..matrix[0].len() - 1)
            .filter(move|col| is_tree_visible(row, *col, &matrix)) 
        )
        .count()
        .try_into()
        .unwrap()
}

// Funkce zjišťující, zda je strom viditelný z jakéhokoli směru
fn is_tree_visible(x: usize, y: usize, matrix: &Vec<Vec<u8>>) -> bool {
    // Jakmile najdeme směr, ze kterého je viditelný, hledání ukončíme
    is_visible(x, y, matrix, Direction::Up) || 
    is_visible(x, y, matrix, Direction::Right) || 
    is_visible(x, y, matrix, Direction::Down) || 
    is_visible(x, y, matrix, Direction::Left) 
}

// Funkce zjišťující viditelnost stromu z jednoho směru -> směr je předán
fn is_visible(x: usize, y: usize,  matrix: &Vec<Vec<u8>>, direction: Direction) -> bool {
    let actual_tree = matrix[x][y];
    let mut slice: Vec<u8> = Vec::new();
    let max_tree_in_direction = match direction {
        Direction::Up => matrix
            .iter()
            .take(x)
            .map(|line| line[y])
            .max()
            .unwrap(),

        Direction::Down => matrix
            .iter()
            .skip(x + 1)
            .map(|line| line[y])
            .max()
            .unwrap(),

        Direction::Right => matrix[x]
            .iter()
            .skip(y + 1)
            .max()
            .unwrap()
            .to_owned(),

        Direction::Left => matrix[x]
            .iter()
            .take(y)
            .max()
            .unwrap()
            .to_owned(),
    };

    actual_tree > max_tree_in_direction
}

// -----------------------------------------------------------------------------------------------------------------------------------------------------------------------
// Testovací funkce

// Test funkce na výpis matice
fn test_matrix_print(matrix: &Vec<Vec<u8>>) {
    for row in 0 .. matrix.len() {
        for col in 0 .. matrix[0].len() {
            print!("{} ", matrix[row][col]);
        }
        println!();
    }
}

// Test funkce, která vrací testovací matici
fn make_test_matrix() -> Vec<Vec<u8>> {
    let m1: Vec<u8> = vec![3, 0, 3, 7, 3 ];
    let m2: Vec<u8> = vec![2, 5, 5, 1, 2];
    let m3: Vec<u8> = vec![6, 5, 3, 3, 2];
    let m4: Vec<u8> = vec![3, 3, 5, 4, 9];
    let m5: Vec<u8> = vec![3, 5, 3, 9, 0];

    vec![m1, m2, m3, m4, m5]
}
fn main() {
    // Načtení souboru mapa.txt
    let map_path = Path::new("mapa.txt");
    let map = load_map(map_path);

    // Výpis výsledku
    println!("V lesy jde vidět {} stromů.", count_visible_trees(&map));

    // Testy
    // Testovací matice
    let test_matrix = make_test_matrix();
    // Výpis lze udělat i pomocí dbg! macra
    dbg!(&test_matrix);

    
    assert_eq!(count_border_trees(&test_matrix), 16);
    assert_eq!(count_inner_trees(&test_matrix), 5);
    assert_eq!(count_visible_trees(&test_matrix), 21);

    // Testovací print
    test_matrix_print(&test_matrix);
}