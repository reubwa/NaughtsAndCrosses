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
    let mut user_turns = 0;
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
        }else if winner == "draw"{
            println!("It's a draw! That's good, I guess...");
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
        user_turns += 1;
        let winner  = check_winner(a1,a2,a3,b1,b2,b3,c1,c2,c3,comp_char,user_char);
        if winner == "U"{
            println!("You win! Well done");
            break;
        }else if winner == "C"{
            println!("Oh dear! The computer won");
            break;
        }else if winner == "draw"{
            println!("It's a draw! That's good, I guess...");
            break;
        }
        display_grid(a1,a2,a3,b1,b2,b3,c1,c2,c3);
        println!("It's the computer's turn!");
        let mut loop_iterations = 0;
        loop{
            let computer_input = get_computer_input(a1,a2,a3,b1,b2,b3,c1,c2,c3,loop_iterations,user_char,user_turns);
            if computer_input=="a1"&&a1!=user_char&&a1!=comp_char{a1=comp_char;break;}
            else if computer_input=="a2"&&a2!=user_char&&a2!=comp_char{a2=comp_char;break;}
            else if computer_input=="a3"&&a3!=user_char&&a3!=comp_char {a3=comp_char;break;  }
            else if computer_input=="b1"&&b1!=user_char&&b1!=comp_char {b1=comp_char;break;  }
            else if computer_input=="b2"&&b2!=user_char&&b2!=comp_char {b2=comp_char;break;  }
            else if computer_input=="b3"&&b3!=user_char&&b3!=comp_char {b3=comp_char;break;  }
            else if computer_input=="c1"&&c1!=user_char&&c1!=comp_char {c1=comp_char;break;  }
            else if computer_input=="c2"&&c2!=user_char&&c2!=comp_char {c2=comp_char;break;  }
            else if computer_input=="c3"&&c3!=user_char&&c3!=comp_char {c3=comp_char;break;  }
            loop_iterations += 1;
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

fn get_computer_input(a1:&str, a2:&str, a3:&str, b1:&str, b2:&str, b3:&str, c1:&str, c2:&str, c3:&str, loop_iterations:i32, u_sym:&str, u_turns:i32) -> String {
    if u_turns == 1 {
        let mut usq:&str = "";
        if a1==u_sym {usq="a1"}
        else if a2==u_sym {usq="a2"}
        else if a3==u_sym {usq="a3"}
        else if b1==u_sym {usq="b1"}
        else if b2==u_sym {usq="b2"}
        else if b3==u_sym {usq="b3"}
        else if c1==u_sym {usq="c1"}
        else if c2==u_sym {usq="c2"}
        else if c3==u_sym {usq="c3"}
        gen_initial_computer_turn(usq)
    }
    else if u_turns > 1 && loop_iterations == 0 {
        let user_chain = match_user_two(a1,a2,a3,b1,b2,b3,c1,c2,c3,u_sym);
        if user_chain == "a1a2"{"a3".to_string()}
        else if user_chain == "a2a3"{"a1".to_string()}
        else if user_chain == "b1b2"{"b3".to_string()}
        else if user_chain == "b2b3"{"b1".to_string()}
        else if user_chain == "c1c2"{"c3".to_string()}
        else if user_chain == "c2c3"{"c1".to_string()}
        else if user_chain == "a1b2"{"c3".to_string()}
        else if user_chain == "b2c3"{"a1".to_string()}
        else if user_chain == "a3b2"{"c1".to_string()}
        else if user_chain == "c1b2"{"a3".to_string()}
        else if user_chain == "a1b1"{"c1".to_string()}
        else if user_chain == "b1c1"{"a1".to_string()}
        else if user_chain == "a2b2"{"c2".to_string()}
        else if user_chain == "b2c2"{"a2".to_string()}
        else if user_chain == "a3b3"{"c3".to_string()}
        else if user_chain == "b3c3"{"a3".to_string()}
        else if user_chain == "a1b2"{"c3".to_string()}
        else if user_chain == "b2c3"{"a1".to_string()}
        else if user_chain == "a3b2"{"c1".to_string()}
        else if user_chain == "b2c1"{"a3".to_string()}
        else { get_computer_rand() }
    }
    else{get_computer_rand()}
}

fn match_user_two(a1:&str, a2:&str, a3:&str, b1:&str, b2:&str, b3:&str, c1:&str, c2:&str, c3:&str, u_sym:&str) -> & 'static str {
    if a1==u_sym && a2==u_sym {"a1a2"}
    else if a2==u_sym && a3==u_sym {"a2a3"}
    else if b1==u_sym && b2==u_sym {"b1b2"}
    else if b2==u_sym && b3==u_sym {"b2b3"}
    else if c1==u_sym && c2==u_sym {"c1c2"}
    else if c2==u_sym && c3==u_sym {"c2c3"}
    else if a1==u_sym && b2==u_sym {"a1b2"}
    else if b2==u_sym && c3==u_sym {"b2c3"}
    else if a3==u_sym && b2==u_sym {"a3b2"}
    else if c1==u_sym && b2==u_sym {"c1b2"}
    else if a1==u_sym && b1==u_sym {"a1b1"}
    else if b1==u_sym && c1==u_sym {"b1c1"}
    else if a2==u_sym && b2==u_sym {"a2b2"}
    else if b2==u_sym && c2==u_sym {"b2c2"}
    else if a3==u_sym && b3==u_sym {"a3b3"}
    else if b3==u_sym && c3==u_sym {"b3c3"}
    else if a1==u_sym && b2==u_sym {"a1b2"}
    else if b2==u_sym && c3==u_sym {"b2c3"}
    else if a3==u_sym && b2==u_sym {"a3b2"}
    else if b2==u_sym && c1==u_sym {"b2c1"}
    else{"none"}
}

fn gen_initial_computer_turn(usq:&str) -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..2);
    if usq=="b2"{
        let number = rng.gen_range(1..4);
        if number == 1{"a1".to_string()}
        else if number == 2{"c3".to_string()}
        else if number == 3{"a3".to_string()}
        else if number == 4{"c1".to_string()}
        else{get_computer_rand()}
    }
    else if usq=="a2"{
        if number == 1{"a1".to_string()}
        else if number == 2 {"a3".to_string()}
        else{get_computer_rand()}
    }
    else if usq=="b1"{
        if number == 1{"a1".to_string()}
        else if number == 2 {"c1".to_string()}
        else{get_computer_rand()}
    }
    else if usq=="c2"{
        if number == 1{"c1".to_string()}
        else if number == 2 {"c3".to_string()}
        else{get_computer_rand()}
    }
    else if usq=="b3"{
        if number == 1{"a3".to_string()}
        else if number == 2 {"c3".to_string()}
        else{get_computer_rand()}
    }
    else{get_computer_rand()}
}

fn get_computer_rand() -> String {
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
    else if b1 == u_sym && b2 == u_sym && b3 == u_sym { "U" }
    else if c1 == c_sym && c2 == c_sym && c3 == c_sym { "C" }
    else if c1 == u_sym && c2 == u_sym && c3 == u_sym { "U" }
    else if a1 == c_sym && b2 == c_sym && c3 == c_sym { "C" }
    else if a1 == u_sym && b2 == u_sym && c3 == u_sym { "U" }
    else if c1 == c_sym && b2 == c_sym && a3 == c_sym { "C" }
    else if c1 == u_sym && b2 == u_sym && a3 == u_sym { "U" }
    else if a1 == c_sym && b1 == c_sym && c1 == c_sym { "C" }
    else if a1 == u_sym && b1 == u_sym && c1 == u_sym { "U" }
    else if a2 == c_sym && b2 == c_sym && c2 == c_sym { "C" }
    else if a2 == u_sym && b2 == u_sym && c2 == u_sym { "U" }
    else if a3 == c_sym && b3 == c_sym && c3 == c_sym { "C" }
    else if a3 == u_sym && b3 == u_sym && c3 == u_sym { "U" }
    else {
        if a1!="-"&&b1!="-"&&c1!="-"&&a2!="-"&&b2!="-"&&c2!="-"&&a3!="-"&&b3!="-"&&c3!="-"{ "draw" }
        else{"unknown"}
    }
}