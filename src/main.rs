
//import
mod functions;

//main function
fn main() {

    
    println!("Start--");
    println!("Input a number, to show different results");
    
    //call the fn input_line to get the user choice
    let choice = functions::input_line();
    println!("-input choice-");

    //parse the choice from String to u32
    let num_choice: u32 = choice.parse().unwrap();

    //Control flow with diferrent options for the user.
    match num_choice{
        0 => {
            functions::run();
        },
        1 => {
            functions::loops();
        },
        2 => {
            functions::enums_testing();
        },
        3 => {
            functions::conditionals();
        },
        _ => {

        }
    }

    println!("--End");
}
