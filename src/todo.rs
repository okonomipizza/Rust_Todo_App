use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use colored::Colorize;

const TODO_PATH: &str = "todos.txt";

#[derive(Debug)]
pub enum CommandError {
    UnknownCommand(),
}

#[derive(Debug)]
pub enum Command {
    Add,
    Change,
    Delete,
    List,
    Help,
}

impl Command {
    pub fn from(input: &str) -> Result<Self, CommandError> {
        match input {
            "add" => Ok(Command::Add),
            "change" => Ok(Command::Change),
            "delete" => Ok(Command::Delete),
            "list" => Ok(Command::List),
            "help" => Ok(Command::Help),
            _ => Err(CommandError::UnknownCommand()),
        }
    }
}

fn read_file_to_string(file_name: &str) -> String {
    let path = Path::new(file_name);
    if path.exists() {
        match fs::read_to_string(file_name) {
            Ok(content) => content,
            Err(_) => {
                "".to_string()
            }
        }
    } else {
        match  File::create(file_name) {
            Ok(_) => {
                "".to_string()
            },
            Err(_) => {
                "".to_string()
            }
        } 
    }
}

#[derive(Debug)]
struct Todo {
    id: i32,
    todo: String,
    completed: bool,
}

pub trait Summary {
    fn stringify(&self) -> String;
}

impl Summary for Todo {
    fn stringify(&self) -> String {
        format!("{}, {}, {}", self.id, self.todo, self.completed)
    }
}

fn convert_completed_bool_to_icon(completed: bool) -> String {
    if completed {
        "Done".to_string()
    } else {
        "Not Yet".to_string()
    }
}

fn vec_to_string(todo_vec: Vec<Todo>) -> String{
    let mut data = String::new();
    let len = todo_vec.len();
    for (index, todo) in todo_vec.into_iter().enumerate() {
        data.push_str(&todo.stringify());
        if index < len - 1 {
            data.push_str("\n");
        }
    }
    data
}


fn convert_string_to_todo(todo_string: &str) -> Todo {
    // {id: i32, todo: string, completed: bool} --"1, lean to use rust, false"
    let parts: Vec<&str> = todo_string.split(',').collect();

    let id: i32 = parts[0].trim().parse().expect("ID should be a number");
    let todo: &str = parts[1].trim();
    let completed: bool = parts[2].trim().parse().expect("is_done should be a boolean");

    Todo {
        id,
        todo: todo.to_string(),
        completed,
    }
}

fn save(data: String) {
        let mut file = File::create(TODO_PATH).expect("msg");
        file.write_all(data.as_bytes()).expect("msg");
}


pub fn add(args: &Vec<String>) {
    // 追加したいtodoコンテンツが入力されているかチェック
    if args.len() == 3 {
        let typed_todo: String = args[2].clone();
        let mut todos = create_vec_of_existed_todos();
        let new_id = todos.len() + 1;


        let todo_to_add = Todo {
            id: new_id as i32,
            todo: typed_todo,
            completed: false,
        };
        todos.push(todo_to_add);

        let updated_data = vec_to_string(todos);
        save(updated_data);
    } else {
        println!("you need to write contents to insert");
        std::process::exit(0);
    }
}

pub fn change(args: &Vec<String>) {
    if args.len() == 3 {
        if let Ok(change_id) = args[2].parse::<usize>() {
            let todos: Vec<Todo> = create_vec_of_existed_todos();
            let mut new_todos: Vec<Todo> = vec!();
            for (index, mut todo) in todos.into_iter().enumerate() {
                if index == change_id - 1 {
                    
                    todo.completed = !todo.completed;
             
                }
                new_todos.push(todo);
            }
            let updated_data = vec_to_string(new_todos);
            save(updated_data);
        } else {
            println!("Invalid id you typed");
            let_users_know_help_command();
            std::process::exit(0);
        }
    } else {
        println!("command arguments are too many or not enough");
        let_users_know_help_command();
        std::process::exit(0);
    }
}

pub fn delete(args: &Vec<String>) {
    if args.len() == 3 {
        if let Ok(delete_id) = args[2].parse::<usize>() {
            let todos = create_vec_of_existed_todos();
            let mut new_todos: Vec<Todo> = vec!();
            for (index, mut todo) in todos.into_iter().enumerate() {
                if index == delete_id -1 {
                    continue;
                }
                if index > delete_id -1 {
                    todo.id -= 1;
                    new_todos.push(todo);
                } else {
                    new_todos.push(todo);
                }
            }
            let updated_data = vec_to_string(new_todos);
            save(updated_data);
        } else {
            println!("Invalid id you typed");
            
        };
    } else {
        println!("command arguments are too many or not enough");
        let_users_know_help_command();
        std::process::exit(0);
    }
}



pub fn list() {
    let todos = create_vec_of_existed_todos();
    // ユーザーへTodoリストを表示する
    for todo in todos {
        println!("{}: {} {}", todo.id, todo.todo, if todo.completed {convert_completed_bool_to_icon(todo.completed).green()} else {convert_completed_bool_to_icon(todo.completed).red()});
    }   
}

pub fn print_help() {
    println!(r#"How to use
You can use 4 commands
1. 'add'
    This command allow you to insert new todo.
    ex) 'cargo run add homework' 
2. 'change'
    This command allow you to change the todo completed or not completed
    ex) 'cargo run change 3'
2. 'delete'
    This command allow you to delete existed todo by designate id.
    You can get to know by using 'list' command.
    ex) 'cargo run delete 3'
3. 'list'
    This command allow you to show todo list.
    You can see all todos and the status.
    "#);
}

fn let_users_know_help_command() {
    println!("If you need help, type 'cargo run help'")
}

fn create_vec_of_existed_todos() -> Vec<Todo>{
    let data: String = read_file_to_string(TODO_PATH);
    let todo_vec = data.lines().collect::<Vec<&str>>();

    let mut todos: Vec<Todo> = vec!();
    for todo_string in todo_vec {
        todos.push(convert_string_to_todo(todo_string));
    }
    todos
}