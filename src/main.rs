use std::io::Write;

fn new_print(x: &str) {
    print!("{x}");
    std::io::stdout().flush().unwrap();
}

fn input() -> String {
    let mut x: String = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new() -> Self {
        let name: String = User::name();
        let age: u32 = User::age();
        Self { name, age }
    }

    fn name() -> String {
        new_print("Enter name: ");
        let mut name: String = input();
        while name.len() > 20 || name.parse::<i32>().is_ok() {
            println!("invalid name");
            new_print("Enter name: ");
            name = input();
        }
        name
    }

    fn age() -> u32 {
        new_print("Enter age: ");
        let mut age: String = input();
        while age.parse::<u32>().is_err() {
            println!("invalid age");
            new_print("Enter age: ");
            age = input()
        }
        age.parse::<u32>().unwrap()
    }
}

fn list(users: &[User]) -> bool {
    if !users.is_empty() {
        for (mut count, user) in users.iter().enumerate() {
            count += 1;
            println!("{}) name: {}", count, user.name);
            println!("   age: {}", user.age);
        }
        true
    } else {
        println!("no users");
        false
    }
}

fn select(users: &[User]) -> usize {
    new_print("select user: ");
    let mut index: String = input();
    while index.parse::<usize>().is_err()
        || index.parse::<usize>().unwrap() > users.len()
        || index.parse::<usize>().unwrap() < 1
    {
        println!("invalid user");
        new_print("select user: ");
        index = input();
    }
    let mut index: usize = index.parse::<usize>().unwrap();
    index -= 1;
    index
}

fn main() {
    let mut users: Vec<User> = Vec::new();
    loop {
        new_print("> ");
        let prompt: String = input();
        if prompt == "help" {
            for cmd in ["mk", "cg -n", "cg -a", "ls", "rm"] {
                println!("- {}", cmd);
            }
        } else if prompt == "mk" {
            users.push(User::new());
        } else if prompt == "ls" {
            list(&users);
        } else if prompt == "cg -n" {
            if list(&users) {
                let index: usize = select(&users);
                users[index].name = User::name();
            }
        } else if prompt == "cg -a" {
            if list(&users) {
                let index: usize = select(&users);
                users[index].age = User::age();
            }
        } else if prompt == "rm" {
            if list(&users) {
                users.remove(select(&users));
            }
        } else if prompt == "cls" {
            print!("\x1B[2J\x1B[1;1H");
        } else if prompt == "exit" {
            break;
        } else {
            println!("invalid");
        }
    }
}
