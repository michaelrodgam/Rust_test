

pub fn run(){
    //simple print
    println!("Hello, World!!");

    //positional formating
    println!("{0}, this is formating. Use the {1}. Bye, {0}!", "Michael", "index");
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