//day3
//use coord system to find symbols: then find attached number and their right/left neighbours
//func around a symbol is number or keep scanning...?
//check around if a number
//if not move onto next box around symbol 
//scan right and left of number to get full number
//collect the number for summing
//mark the numbers as seen
//move onto next coord of symbol
//repeat
//adapt code for real data or any
//try to add tests + guards
//factor in the edges

//stores the symbol coords to search around
struct SymbolCoord  {
    x: usize,
    y: usize,
}


//This is where I am in matrix
struct Current  {
    x: usize,
    y: usize,
}

fn main() {
   //avoid using and may junk, cant get rid of newlines
    //so tried diff approach below
    //let data: Vec<_> = include_str!("test.txt").chars().collect();

    //This is the current character I am searching
    //not what I windowing into around 
    //build a func to do that
    let mut cursor = Current  {
        x: 0,
        y: 0,
    };


    //all the symbols, numbers and dots
    //Main data structure I am searching
    //should be able to navigate around this
    //if i can correlate with the seen matrix
    let grid: Vec<_> = include_str!("test.txt").lines().collect();

    println!("grid....");
    for line in &grid  {
        println!("{:?}", line);    
    }

    //quick reference:
    /*
    "467..114.."
    "...*......"
    "..35..633."
    "......#..."
    "617*......"
    ".....+.58."
    "..592....."
    "......755."
    "...$.*...."
    ".664.598.."
    */

    //let check if i can find values in the grid
    let first_item = &grid[cursor.y];
    let first_item = &first_item[cursor.x..cursor.x+1];
    println!("This should be 4: {}", first_item);

    cursor.x = 1;
    
    let second = &grid[cursor.y];
    let second =  &second[cursor.x..cursor.x+1];
    println!("This should be 6: {}", second);

    cursor.y = 1;
    cursor.x = 3;

    let third = &grid[cursor.y];
    let third = &third[cursor.x..cursor.x+1];
    println!("This should be *: {}", third);


    //This builds the seen matrix
    //Used to store numbers that have been seen 
    //and captured to avoid double counting values
    //This will be checked before looking for a number
    let mut seen: Vec<Vec<bool>> = vec![vec![false;10];10]; 
    seen[0][0] = true;
    seen[3][5] = true;

    //check it works as expected
    for line in seen {
        println!("{:?}", line);
    }

    //used to store the numbers to sum for the answer
    //may be able to make redundant
    //need a helper to scan left and right and find numbers
    let mut numbers: Vec<u32> = Vec::new();

    //used to store the coords of the symbols
    //they are the ones I will search around
    let mut coords: Vec<SymbolCoord> = Vec::new();

    
    //I am using it here to build the cursor coordinates
    //checking the cursor lines up with grid and seen structures
    //check this accesses the seen matrix and the grid matrix correctly
    cursor.x = 0;
    cursor.y = 0;
    for line in grid {
        for _ in line.chars()  {
            println!("cursor x:{} y:{}", cursor.x, cursor.y);
            cursor.x += 1;
            cursor.x %= 10;
        }

        cursor.y +=1;
        
    }
}


//These three functions check if a grid element 
//is a type
fn is_number(sym: char) -> bool {
    if sym.is_digit(10) {
        return true;
    }
    return false;
}
fn is_symbol(sym: char) -> bool {
    let symbols = "#&*%/-=+$@".to_string();
    for each in symbols.chars() {
        if sym == each {
            return true;
        }
    }
    return false;
}

//these should be gone now
//i converted to a grid
fn is_newline(sym: char) -> bool {
    if sym.is_ascii_control() {
        return true;
    }
    return false;
}

