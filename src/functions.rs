//import std lib
use std::io;

//test fn

pub fn run(){
    //simple print
    println!("Hello, World!!");

    //positional formating
    println!("{0}, this is formating. Use the {1:?} Bye, {0}!", "Michael", "index\n");
}

#[allow(unused_parens)] //this is for shut up the warnings about () in the if statement. 
pub fn conditionals(){
    //simple int var
    let my_num: i32 = 25;
    
    //control flow
    if(my_num > 0){
        println!("it's bigger than zero!");
    }
    else if(my_num < 0){
        println!("it's smaller than zero!");
    }
    else{
        println!("{}", my_num);
    }
}

pub fn loops(){
    let mut i: u32 = 30;
    while i > 0 {
        if i % 5 == 0 {
            println!("This is the {}th loop.", i);
        }
        i -= 1;
    }

    let games = vec!["Hokuto", "Watch Dogs", "Sniper Elite", "Death Stranding"];
    for g in games.iter(){
        println!("Im playing: {}.", g);
    }

    for (index, g) in games.iter().enumerate(){
        println!("Im playing: {1}, this is index {0}.", index, g);
    }
}

#[allow(dead_code)]

pub fn enums_testing(){
    
    enum Directions{
        Up, 
        Down, 
        Left, 
        Right
    }

    let movement:Directions = Directions::Up;

    match movement{
        Directions::Up => println!("Move Up"),
        Directions::Down => println!("Move Down"),
        Directions::Left => println!("Move L"),
        Directions::Right => println!("Move R"),
    }
}

//funtion to read line.
pub fn input_line() -> String{

    let mut choice = String::new();

    match io::stdin().read_line(&mut choice){
        Ok(_) => {
            //this will remove \r\n from the string input. its posible to use 2 instead: choice.pop();
            choice.truncate(choice.len() - 2); 
            choice
        },
        Err(_) => {
            "0".to_string()
        }
    }
}