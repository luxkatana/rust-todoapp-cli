mod todoapi;
mod cli;
use std::process::exit;

use clap::Parser;
use cli::{Action, Cli};
use todoapi::*;

fn main() -> BoxResult<()> {
    let cli = Cli::parse();
    if !cli.path.exists() && !matches!(cli.command, Action::InitTodo {}) {
        eprintln!("Please init first a database-todo by using the InitTodo action!");
        exit(1);
    }
    let mut todoapp: TodoApp;
    match cli.command {
        Action::InitTodo {} => {
            TodoApp::setup(cli.path)?;
            println!("Database created");
            exit(0);
        },
        Action::AddPoint { description, checked } => {
            todoapp = TodoApp::from_existing_file(cli.path)?;
            let new = todoapp.new_todo_point(description, checked.is_some())?;
            println!("New point added with ID {}", new.sql_id);
        },
        Action::ShowAllPoints {} => {
            todoapp = TodoApp::from_existing_file(cli.path)?;
        },
        Action::SetChecked { change_to, id } => {
            todoapp = TodoApp::from_existing_file(cli.path)?;
            let mut todopoint = todoapp.todopoints.iter_mut()
                .find(|apoint| apoint.sql_id == id);
                
            if let Some(todox) = todopoint.as_deref_mut() {
                todox.finished(change_to)?;
            }
            else {
                eprintln!("There is no todopoint with that id!");
                exit(1);
            }

        },
        Action::ReinstallTable {}  => {
            TodoApp::setup(cli.path)?;
            println!("Finished");
            exit(0);
        },
        Action::RemovePoint { id } => {
            todoapp = TodoApp::from_existing_file(cli.path)?;
            todoapp.remove_point_by_id(id)?;
            println!("Todo removed");
        },
        Action::ChangeDescription { to, id } => {
            todoapp = TodoApp::from_existing_file(cli.path)?;
            let target = todoapp.todopoints.iter_mut()
                    .find(|element| element.sql_id == id);
            if let Some(target) = target {
                target.description(to)?;
                println!("Description changed");
            }
            else {
                eprintln!("Todopoint with that id does not exist!");
                exit(1);
            }

        },
    };
    
    for row in todoapp.print_all_todos()? {
        println!("{row}");
    } 

    Ok(())
}
