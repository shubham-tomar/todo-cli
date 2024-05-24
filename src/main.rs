
#![allow(unused_imports)]

mod todo;
use structopt::StructOpt;
use todo::Todo;

#[derive(StructOpt)]
enum Command {
    #[structopt(about = "Add a new todo item")]
    Add {
        #[structopt(help = "The item to be added")]
        item: String
    },
    #[structopt(about = "Remove a todo item")]
    Remove {
        #[structopt(help = "The index of the item to remove")]
        index: usize
    },
    #[structopt(about = "Edit a todo item")]
    Edit {
        #[structopt(help = "The index of the item to edit")]
        index: usize,
        new_content: String
    },
    #[structopt(about = "List all todo items")]
    List
}

fn main() {
    let cmd = Command::from_args();
    let mut todo = Todo::load().expect("Failed to load Todo List");


    match cmd {
        Command::Add {item} => {
            todo.add(item);
            todo.list();
            todo.save().expect("Failed to save");
        }
        Command::Remove {index} => {
            match todo.remove(index) {
                Ok(_) => {
                    todo.save().expect("Failed to save");
                    println!("Item removed");
                    todo.list();
                },
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
        Command::Edit {new_content, index} => {
            match todo.edit(index, new_content) {
                Ok(_) => {
                    todo.save().expect("Failed to save");
                    println!("Item Edited");
                    todo.list();
                },
                Err(err) => {
                    println!("{}", err)
                }
            }

        }
        Command::List => {
            todo.list()
        }
    }
}
