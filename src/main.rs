use std::io;
use std::cmp;

fn main() {

    let mut tasks = Vec::new();
    println!("/ / / / / / / / / / / / / / / / / / / / / / / / / / / / ");
    println!("Welcome to your TODO list! You have {} tasks remaining.", 0);
    println!("Please input '+' to add a new TODO entry");
    println!("Please input 'list' to list current TODO entries");
    println!("Please input '-' to delete a TODO entry @ pos");
    println!("/ / / / / / / / / / / / / / / / / / / / / / / / / / / / ");


    loop{
        

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        match input {
            "+" => {
                println!("Please input your TODO");
                let mut todo = String::new();
                io::stdin().read_line(&mut todo)
                    .expect("Failed to read");
                todo = todo.trim().to_string();
                addTODO(todo, &mut tasks);
            }
            "list" => {
                //print a number followed by the TODO for that index of the vector
                println!("Welcome to your TODO list! You have {} tasks remaining.", tasks.len());

                for (index, task) in tasks.iter().enumerate(){
                    println!("{} {}", index + 1, task);
                }

            }
            "-" => {
                println!("Please specify which number TODO you would like to remove");
                let mut index = String::new();
                io::stdin().read_line(&mut index)
                    .expect("Failed to read");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                removeTODO(index - 1, &mut tasks);

            }
            _ => {
                println!("UNRECOGNIZED COMMAND.");
            }


        }

        println!("/ / / / / / / / / / / / / / / / / / / / / / / / / / / / ");
        println!("Please input '+' to add a new TODO entry");
        println!("Please input 'list' to list current TODO entries");
        println!("Please input '-' to delete a TODO entry @ pos");
        println!("/ / / / / / / / / / / / / / / / / / / / / / / / / / / / ");


    }

}

fn addTODO(added: String, target: &mut Vec<String>){
    let todo = added;

    target.push(todo);
}
fn removeTODO(index: usize, target: &mut Vec<String>){
    target.remove(index);
}
