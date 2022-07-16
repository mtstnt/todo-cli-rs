use std::{io::{stdin, stdout, Write}, cell::RefCell};

struct Todo {
    title: String,
    content: String,
    is_completed: bool
}

impl Todo {
    fn new(title: String, content: String) -> Self {
        Todo { title, content, is_completed: false }
    }
}

struct Category {
    name: String,
    todos: Vec<Todo>
}

impl Category {
    fn new(name: String) -> Self {
        Category { name, todos: vec![] }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn print(v: &str) {
    print!("{}", v);
    stdout().flush().unwrap();
}

fn menu(todo: &mut Vec<Category>) -> bool {
    println!("1. Add Category");
    println!("2. Add Todo");
    println!("3. Set Todo Completed");
    println!("4. Remove Todo");
    println!("5. Remove Category");
    println!("6. Exit");
    print("Choice: ");
    let input = get_input();

    let choice_number = input.parse::<i32>().expect("Input is not a number");
    match choice_number {
        1 => add_category(todo),
        2 => add_todo(todo),
        3 => set_completed(todo),
        4 => remove_todo(todo),
        5 => remove_category(todo),
        6 => return false,
        _ => ()
    }
    true
}

fn add_category(todos: &mut Vec<Category>) {
    println!("Adding new category...");
    print("Category name: ");
    let category_name = get_input();
    if !todos.iter().any(|el| el.name == category_name) {
        todos.push(Category::new(category_name.clone()));
        return;
    }
    println!("Category {} already exists", category_name);
}

fn add_todo(todos: &mut Vec<Category>) {
    println!("Adding new todo...");
    for (idx, cat) in todos.iter().enumerate() {
        println!("{}. {}", &idx, &cat.name);
    }
    print("Category choice: ");
    let choice = get_input().parse::<i32>().expect("Input is not a number");
    let category = todos.get_mut(choice as usize).expect("Out of bounds!");

    print("Todo title: ");
    let todo_title = get_input();

    print("Todo content: ");
    let todo_content = get_input();

    category.todos.push(Todo::new(todo_title, todo_content));
}

fn set_completed(todos: &mut Vec<Category>) {
    println!("Pick category: ");
    for (idx, cat) in todos.iter().enumerate() {
        println!("{}. {}", &idx, &cat.name);
    }
    let cat_choice = get_input().parse::<i32>().expect("Input is not a number");
    let category = todos.get_mut(cat_choice as usize).expect("Out of bounds!");

    println!("Pick todos: ");
    for (idx, todo) in category.todos.iter().enumerate() {
        println!("{}. {}, {}", &idx, &todo.title, &todo.content);
    }
    let todo_choice = get_input().parse::<i32>().expect("Input is not a number");

    let mut todo = category.todos.get_mut(todo_choice as usize).expect("Out of bounds of todos for category");
    todo.is_completed = true;
}

fn remove_todo(todos: &mut Vec<Category>) {
    println!("Pick category: ");
    for (idx, cat) in todos.iter().enumerate() {
        println!("{}. {}", &idx, &cat.name);
    }
    let cat_choice = get_input().parse::<i32>().expect("Input is not a number");
    let category = todos.get_mut(cat_choice as usize).expect("Out of bounds!");

    println!("Pick todos: ");
    for (idx, todo) in category.todos.iter().enumerate() {
        println!("{}. {}, {}", &idx, &todo.title, &todo.content);
    }
    let todo_choice = get_input().parse::<i32>().expect("Input is not a number");

    category.todos.remove(todo_choice as usize);
}

fn remove_category(todos: &mut Vec<Category>) {
    println!("Pick category: ");
    for (idx, cat) in todos.iter().enumerate() {
        println!("{}. {}", &idx, &cat.name);
    }
    let cat_choice = get_input().parse::<i32>().expect("Input is not a number");
    let _ = todos.get_mut(cat_choice as usize).expect("Out of bounds!");

    todos.remove(cat_choice as usize);
}

fn main() {
    let mut todo_categories: Vec<Category> = vec![];
    loop {
        println!("Todos: ");
        let mut i = 1;
        for t in &todo_categories {
            println!("{}. {}", &i, t.name);
            for (idx, u) in t.todos.iter().enumerate() {
                println!("\t{}. {}: {}, ({})", &idx + 1, u.title, u.content, match u.is_completed {
                    true => "Done",
                    false => "Not done",
                });
            }
            i += 1;
        }
        println!("=====================================");

        if !menu(&mut todo_categories) {
            break;
        }

        print("\x1B[2J\x1B[1;1H");
    }

    println!("Thankyou!");
}
