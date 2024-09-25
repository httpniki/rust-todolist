#![allow(dead_code)]
mod modules;

use crossterm::style::Stylize;
use modules::actions::Actions;
use modules::todos::{Todo, Todos};
use std::io::{self, Write};

fn print_todos(todos: Vec<Todo>) -> () {
    if todos.len() == 0 {
        return println!("{}", "Doesn't have any TODOs".red());
    }

    if todos.len() > 0 {
        for (index, todo) in todos.iter().enumerate() {
            println!("~ TODO #{} [ID: {}] {}", index + 1, todo.id, todo.content);
        }
    }
}


fn main() {
    let actions = Actions::init();
    let mut todos = Todos::init();

    let add_index = actions.actions_list.add.index.to_string();
    let list_index = actions.actions_list.list.index.to_string();
    let remove_index = actions.actions_list.remove.index.to_string();
    let help_index = actions.actions_list.help.index.to_string();
    let exit_index = actions.actions_list.exit.index.to_string();

    println!("{}", "--------------------------------".blue());
    println!("{}", "     Welcome to TODO list!".blue());
    println!("{}", "--------------------------------".blue());

    println!("{}", "* Available commands:".blue());
    actions.print_actions();

   loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == list_index {
            print_todos(todos.todos.clone());
            continue;
        }

        if input == add_index {
            let mut content = String::new();

            println!("{}", "* Enter the content of the TODO:".blue());

            print!("Content > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut content).unwrap();

            todos.create_todo(content.trim().to_string());

            println!("{}", "The TODO has been created!".green());
            continue;
        }

        if input == remove_index {
            let mut id = String::new();
            print_todos(todos.todos.clone());

            if todos.todos.len() == 0 {
                continue;
            }

            println!("{}", "* Enter the ID of the TODO to remove:".blue());

            print_todos(todos.todos.clone());

            print!("ID > ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut id).unwrap();

            let id = id.trim().parse::<u8>().unwrap();
            let is_exists = todos.todos.iter().find(|&todo| todo.id == id);

            match is_exists {
                Some(_) => {
                    todos.delete_todo(id);
                    println!("{}", "TODO deleted!".green());
                },
                None => println!("{}", "TODO not found.".red()),
            };

            continue;
        }

        if input == "help" || input == help_index {
            println!("{}", "* Available commands:".blue());              
            actions.print_actions();
            continue;
        }

        if input == "exit" || input == exit_index {
            break;
        }

        println!(
            "{}",
            "Invalid input! Try using help to see the available commands".red()
        );
    }
}
