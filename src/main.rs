use rand::Rng;
use std::io;

fn main() {
    let mut a1 = "-";
    let mut a2 = "-";
    let mut a3 = "-";
    let mut b1 = "-";
    let mut b2 = "-";
    let mut b3 = "-";
    let mut c1 = "-";
    let mut c2 = "-";
    let mut c3 = "-";
    let mut user_char = "X";
    let mut comp_char = "O";
    let mut input_string = String::new();
    println!("Would you like to be X or O?: ");
    input_string.clear();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string = input_string.to_lowercase();
    if input_string == "x"{
        user_char = "X";
        comp_char = "O";
    }else if input_string == "o"{
        comp_char = "X";
        user_char = "O";
    }
    loop {
        let winner  = check_winner(a1,a2,a3,b1,b2,b3,c1,c2,c3,comp_char,user_char);
        if winner == "U"{
            println!("You win! Well done");
            break;
        }else if winner == "C"{
            println!("Oh dear! The computer won");
            break;
        }
        display_grid(a1,a2,a3,b1,b2,b3,c1,c2,c3);
        println!("It's your turn!");
        loop{
            let user_input = get_user_input();
            if user_input=="a1"&&a1!=comp_char{a1=user_char;break;}
            else if user_input=="a2"&&a2!=comp_char{a2=user_char;break;}
            else if user_input=="a3"&&a3!=comp_char {a3=user_char; break; }
            else if user_input=="b1"&&b1!=comp_char {b1=user_char; break; }
            else if user_input=="b2"&&b2!=comp_char {b2=user_char; break; }
            else if user_input=="b3"&&b3!=comp_char {b3=user_char; break; }
            else if user_input=="c1"&&c1!=comp_char {c1=user_char; break; }
            else if user_input=="c2"&&c2!=comp_char {c2=user_char; break; }
            else if user_input=="c3"&&c3!=comp_char {c3=user_char; break; }
            else{println!("Invalid move!");continue;}
        }
        let winner  = check_winner(a1,a2,a3,b1,b2,b3,c1,c2,c3,comp_char,user_char);
        if winner == "U"{
            println!("You win! Well done");
            break;
        }else if winner == "C"{
            println!("Oh dear! The computer won");
            break;
        }
        display_grid(a1,a2,a3,b1,b2,b3,c1,c2,c3);
        println!("It's the computer's turn!");
        loop{
            let computer_input = get_computer_input();
            if computer_input=="a1"&&a1!=user_char&&a1!=comp_char{a1=comp_char;break;}
            else if computer_input=="a2"&&a2!=user_char&&a2!=comp_char{a2=comp_char;break;}
            else if computer_input=="a3"&&a3!=user_char&&a3!=comp_char {a3=comp_char;break;  }
            else if computer_input=="b1"&&b1!=user_char&&b1!=comp_char {b1=comp_char;break;  }
            else if computer_input=="b2"&&b2!=user_char&&b2!=comp_char {b2=comp_char;break;  }
            else if computer_input=="b3"&&b3!=user_char&&b3!=comp_char {b3=comp_char;break;  }
            else if computer_input=="c1"&&c1!=user_char&&c1!=comp_char {c1=comp_char;break;  }
            else if computer_input=="c2"&&c2!=user_char&&c2!=comp_char {c2=comp_char;break;  }
            else if computer_input=="c3"&&c3!=user_char&&c3!=comp_char {c3=comp_char;break;  }
        }
    }
}

fn display_grid(a1:&str, a2:&str, a3:&str, b1:&str, b2:&str, b3:&str, c1:&str, c2:&str, c3:&str) {
    println!(" 1 2 3");
    println!("A {} {} {}",a1,a2,a3);
    println!("B {} {} {}",b1,b2,b3);
    println!("C {} {} {}",c1,c2,c3);
}

fn get_user_input() -> String {
    let mut row_letter = String::new();
    let mut row_number = String::new();
    println!("Please enter the row letter: ");
    io::stdin().read_line(&mut row_letter).unwrap();
    row_letter = row_letter.trim().to_lowercase();
    println!("Please enter the row number: ");
    io::stdin().read_line(&mut row_number).unwrap();
    row_number = row_number.trim().parse().unwrap();
    format!("{}{}", row_letter,row_number)
}

fn get_computer_input() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..3);
    let alt_number = rng.gen_range(1..3);
    let mut row_letter = String::new();
    let mut row_number = String::new();
    if number == 1{row_letter = "a".parse().unwrap();}
    else if number == 2{row_letter = "b".parse().unwrap();}
    else if number == 3{row_letter = "c".parse().unwrap();}
    row_number = alt_number.to_string();
    format!("{}{}", row_letter,row_number)
}

fn check_winner(a1:&str, a2:&str, a3:&str, b1:&str, b2:&str, b3:&str, c1:&str, c2:&str, c3:&str, c_sym:&str, u_sym:&str) -> & 'static str {
    if a1 == c_sym && a2 == c_sym && a3 == c_sym { "C" }
    else if a1 == u_sym && a2 == u_sym && a3 == u_sym { "U" }
    else if b1 == c_sym && b2 == c_sym && b3 == c_sym { "C" }
    else if b1 == u_sym && b2 == u_sym && b3 == u_sym { "U"  }
    else if c1 == c_sym && c2 == c_sym && c3 == c_sym { "C" }
    else if c1 == u_sym && c2 == u_sym && c3 == u_sym { "U" }
    else if a1 == c_sym && b2 == c_sym && c3 == c_sym { "C" }
    else if a1 == u_sym && b2 == u_sym && c3 == u_sym { "U" }
    else if c1 == c_sym && b2 == c_sym && a3 == c_sym { "C" }
    else if c1 == u_sym && b2 == c_sym && c3 == c_sym { "U"  }
    else {"unknown"}
}