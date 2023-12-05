use std::usize;

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
    //issue if i still have newlines
    //may need this string to slice out the numbers
    //maybe can avoid that with just creating a value and concatinating
    //avoid and may junk     
    let data: Vec<_> = include_str!("test.txt").chars().collect();

    //This is the current character I searching
    //not what I windowing into 
    let mut cursor = Current  {
        x: 0,
        y: 0,
    };


    //Main data structure I am searching
    //should be able to navigate around this
    //if i cant correlate with the seen matrix
    let grid: Vec<_> = include_str!("test.txt").lines().collect();

    println!("grid....");
    for line in &grid  {
        println!("{:?}", line);    
    }

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
    //let check if i can find a value in the grid
    //can i find the one above and the one below
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

    for line in seen {
        println!("{:?}", line);
    }

    //used to store the numbers to sum
    //may be able to make redundant
    let mut numbers: Vec<u32> = Vec::new();

    //used to store the coords of the symbols
    //they are the ones I will search around
    let mut coords: Vec<SymbolCoord> = Vec::new();

    
    //This is the index into the string 
    //I am using it here to build the cursor coordinates
    //maybe put them into a vector
    //problem is it contains newlines so this may end up buggy
    //Keep the code below, it is handy to build coords.
    let mut index: usize = 0;

    for each in data {
        //println!("index: {} cursor x:{} y:{}", index, cursor.x, cursor.y);
        index += 1;
        cursor.x +=1 % 10;
        cursor.y = index / 10;

        
    }
}


//These three functions check if an char
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

fn is_newline(sym: char) -> bool {
    if sym.is_ascii_control() {
        return true;
    }
    return false;
}

