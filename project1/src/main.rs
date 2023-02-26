// CRUD Application
// CREATE
// READ
// UPDATE
// DELETE

use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: i32,
}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }
    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }
    fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }
    fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }
    fn edit(&mut self, name: &str, age: i32) -> bool {
        match self.class.get_mut(name) {
            Some(name) => {
                name.age = age;
                true
            }
            None => false,
        }
    }
}

pub mod manager {
    use crate::*;

    pub fn add_student(students: &mut Students) {
        println!("Enter name of student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter name of student");
        let age = match get_int() {
            Some(number) => number,
            None => return,
        };
        let student = Student { name, age };
        students.add(student)
    }

    pub fn view(students: &Students) {
        for student in students.view_all() {
            println!("{:?}", student);
        }
    }

    pub fn remove_student(students:&mut Students){
        for student in students.view_all(){
            println!("{:?}",student);
        }
        let name = match get_input(){
            Some(input)=>input,
            None => return
        };
        if students.remove(&name) {
            println!("remove student")
        }else {
            println!("not found")
        }
    }

    pub fn edit_students(students:&mut Students){
        for student in students.view_all(){
            println!("{:?}",student);
        }
        let name = match get_input(){
            Some(input)=>input,
            None => return
        };
        let age = match get_int(){
            Some(input)=>input,
            None => return
        };
        if students.edit(&name,age){
            println!("student has edit")
        } else{
            println!("not found")
        }
    }
}

fn get_int() -> Option<i32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please try again");
    }
    let input = buffer.trim().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}
enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    RemoveStudent,
}
impl Manager {
    fn show() {
        println!("");
        println!("== Mananger Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Remove Student");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::RemoveStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choice(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),
            Some(Manager::EditStudent) => manager::edit_students(&mut students),
            Some(Manager::RemoveStudent) => manager::remove_student(&mut students),
            None => return,
        }
    }
}