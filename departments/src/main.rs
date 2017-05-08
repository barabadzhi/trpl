#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Mutex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Department {
    Engeneering,
    Sales,
    CEO,
    HR,
    IT,
    Support,
    Inventory,
    Finance,
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Department::Engeneering => write!(f, "Engeneering"),
            Department::Sales => write!(f, "Sales"),
            Department::CEO => write!(f, "CEO"),
            Department::HR => write!(f, "HR"),
            Department::IT => write!(f, "IT"),
            Department::Support => write!(f, "Support"),
            Department::Inventory => write!(f, "Inventory"),
            Department::Finance => write!(f, "Finance"),
        }
    }
}

type Name = String;

lazy_static!{
    static ref COMPANY_DEPARTMENTS: Mutex<HashMap<Department, Vec<Name>>> = {
        Mutex::new(HashMap::new())
    };
}

fn main() {
    text_interface();
}

fn text_interface() {
    loop {
        let mut command = String::new();

        print!("Command: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).expect("failed to read line");

        let command: Vec<&str> = command.trim().split(" ").collect();

        match command.get(0) {
            Some(&"exit") => std::process::exit(0),
            Some(&"list") => {
                match command.get(1) {
                    Some(department) => list_department(department),
                    None => list_department(""),
                }
            }
            Some(&"add") => {
                match (command.get(1), command.get(3)) {
                    (Some(name), Some(department)) => {
                        add_employee(name, department);
                    }
                    _ => println!("failed to read name or department"),
                }
            }
            _ => print_help_message(),
        }
    }
}

fn parse_department(department: &str) -> Option<Department> {
    match String::from(department).to_lowercase().as_ref() {
        "engeneering" => Some(Department::Engeneering),
        "sales" => Some(Department::Sales),
        "ceo" => Some(Department::CEO),
        "hr" => Some(Department::HR),
        "it" => Some(Department::IT),
        "support" => Some(Department::Support),
        "inventory" => Some(Department::Inventory),
        "finance" => Some(Department::Finance),
        _ => None,
    }
}

fn list_employees(department: &Department, department_employees: &Vec<Name>) {
    println!("Employees of {} Department:", department);

    for employee in department_employees {
        println!("{}", employee);
    }
}

fn list_department(department: &str) {
    let company_departments = COMPANY_DEPARTMENTS.lock().unwrap();

    if let Some(department) = parse_department(department) {
        if let Some(department_employees) = company_departments.get(&department) {
            list_employees(&department, department_employees);
        }
    } else {
        for (department, department_employees) in company_departments.iter() {
            list_employees(&department, department_employees);
        }
    }
}

fn add_employee(name: &str, department: &str) {
    let name = String::from(name);

    if let Some(department) = parse_department(department) {
        let mut company_departments = COMPANY_DEPARTMENTS.lock().unwrap();

        match company_departments.get_mut(&department) {
            Some(department_employees) => {
                if let None = department_employees.iter().position(|n| n == &name) {
                    department_employees.push(name);
                }
                return;
            }
            None => {}
        }

        company_departments.insert(department, vec![name]);
    } else {
        println!("unknown department: {}", department);
        return;
    }
}

fn print_help_message() {
    println!("\nUsage:\n\tCommand: <command>\n");
    println!("Commands:");
    println!("\tadd <name> to <department>  Add employee to a department in the company");
    println!("\tlist                        List all employees by their departments");
    println!("\tlist <department>           List all employees of the department");
    println!("\thelp                        Display this help message");
    println!("\texit                        Exit an application");
}
