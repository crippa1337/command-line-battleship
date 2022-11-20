use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy)]
enum Square {
    Empty,
    Shot,
    Hit,
    Ship,
}

fn main() {
    // Creates a hashmap with different letters corresponding to different indices on the board
    let letters: HashMap<String, usize> = HashMap::from([
        (String::from("a"), 0),
        (String::from("b"), 1),
        (String::from("c"), 2),
        (String::from("d"), 3),
        (String::from("e"), 4),
        (String::from("f"), 5),
        (String::from("g"), 6),
        (String::from("h"), 7),
        (String::from("i"), 8),
        (String::from("j"), 9),
    ]);

    let mut player_arr = [[Square::Empty; 10]; 10];
    let mut comp_arr = [[Square::Empty; 10]; 10];
    print_board(&player_arr, false);
    println!("--------------------------------------\nINPUT IS TAKEN AS SUCH: <LETTER><NUMBER>\nEXAMPLE: a6, B7, j3\n--------------------------------------");

    // Initial ship placement
    for i in 1..=6u8 {
        if i <= 3 {
            println!("Place Ship #{i}, horizontally.");
            let cord: [usize; 2] = handle_input(&letters, "horizontal");
            player_arr[cord[1]][cord[0]] = Square::Ship;
            player_arr[cord[1]][cord[0] - 1] = Square::Ship;
            player_arr[cord[1]][cord[0] + 1] = Square::Ship;
        } else if i >= 4 {
            println!("Place Ship #{i}, vertically.");
            let cord: [usize; 2] = handle_input(&letters, "vertical");
            player_arr[cord[1]][cord[0]] = Square::Ship;
            player_arr[cord[1] - 1][cord[0]] = Square::Ship;
            player_arr[cord[1] + 1][cord[0]] = Square::Ship;
        }
        clear_screen();
        print_board(&player_arr, false);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_board(board: &[[Square; 10]; 10], comp_board: bool) {
    use ansi_term::Color::{Cyan, Red, RGB};

    println!("      A   B   C   D   E   F   G   H   I   J");
    for (row_id, row) in board.iter().enumerate() {
        print!(
            "{}\n {}  ",
            RGB(100, 100, 100).paint("    +---+---+---+---+---+---+---+---+---+---+"),
            row_id
        );
        for &square in row {
            print!("{}", RGB(100, 100, 100).paint("| "));
            match square {
                Square::Empty => print!("  "),
                Square::Shot => print!("{}", Cyan.paint("O ")),
                Square::Hit => print!("{}", Red.bold().paint("X ")),
                Square::Ship => {
                    if comp_board {
                        print!("  ")
                    } else {
                        print!("# ")
                    }
                }
            }
        }
        println!("{}", RGB(100, 100, 100).paint("|"));
    }
    println!(
        "{}",
        RGB(100, 100, 100).paint("    +---+---+---+---+---+---+---+---+---+---+")
    );
}

fn input_to_coordinate(input: &String, letters: &HashMap<String, usize>) -> [usize; 2] {
    let buf = input.trim().to_lowercase();
    let mut input = buf.chars();
    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let x = letters.get(&x.to_string()).unwrap();
    let y = y.to_digit(10).unwrap() as usize;
    // Index 0 is x, index 1 is y
    return [*x, y];
}

fn handle_input(letters: &HashMap<String, usize>, rotation: &str) -> [usize; 2] {
    loop {
        let mut input = String::new();

        loop {
            input.clear();
            stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input.len() == 2 {
                for (i, c) in input.chars().enumerate() {
                    if i == 0 && letters.contains_key(&c.to_string()) {
                        println!("doing the first one trols C={} ROTAION={}", c, rotation);
                        if rotation == "horizontal" {
                            if c == 'a' || c == 'j' {
                                println!("That's out of bounds!");
                                break;
                            }
                        }
                    } else if i == 1 && c.is_digit(10) {
                        println!("SECOND ONE C={} ROTAION={}", c, rotation);
                        if rotation == "vertical" {
                            if c == '0' || c == '9' {
                                println!("That's out of bounds!");
                                break;
                            }
                        }
                    } else {
                        println!("'{input}' is an invalid input");
                        break;
                    }
                }
                return input_to_coordinate(&input, &letters);
            } else {
                println!("'{input}' is an invalid input");
            }
        }
    }
}
