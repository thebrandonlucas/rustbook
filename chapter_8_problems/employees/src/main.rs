use std::{collections::HashMap, io::stdin};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum Department {
    Accounting,
    Sales,
    IT,
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    department: Department,
}

impl Employee {
    fn new(n: &str, d: Department) -> Self {
        Self {
            name: n.to_string(),
            department: d,
        }
    }
}

fn add_employee(map: &mut HashMap<Department, Vec<Employee>>) {
    let mut name = String::new();

    println!("Enter the employee's name:");
    stdin().read_line(&mut name).expect("Failed to read name");

    println!("Please choose employee's department:");
    println!("1 - Accounting");
    println!("2 - Sales");
    println!("3 - IT");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => panic!("Failed to parse number: {e}"),
    };

    let department = match input {
        1 => Department::Accounting,
        2 => Department::Sales,
        3 => Department::IT,
        i32::MIN..=0_i32 => panic!("{input} is not one of the options!"),
        4_i32..=i32::MAX => panic!("{input} is not one of the options!"),
    };

    let new_employee = Employee::new(name.trim(), department.clone());

    map.entry(department)
        .and_modify(|v| v.push(new_employee.clone()))
        .or_insert(vec![new_employee]);
}

fn list_employees_in_department(
    map: &mut HashMap<Department, Vec<Employee>>,
    department: Department,
) {
    println!("{:?}", department);
    for employee in map.entry(department).or_insert(vec![]).iter() {
        println!("{:?}", employee);
    }
}

fn main() {
    let mut employees_departments: HashMap<Department, Vec<Employee>> = HashMap::new();

    loop {
        let mut choice = String::new();

        println!("Choose an action: ");
        println!("1 - Add Employee");
        println!("2 - List Employees by Department");
        stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();
        if choice.eq(&String::from("q")) {
            println!("Goodbye!");
            break;
        }
        let choice: i32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{choice}");

        if choice == 1 {
            add_employee(&mut employees_departments);
        } else if choice == 2 {
            println!("Choose department to list:");
            println!("1 - Accounting");
            println!("2 - Sales");
            println!("3 - IT");

            let mut choice = String::new();
            stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: i32 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Failed to parse"),
            };

            let department: Department = match choice {
                1 => Department::Accounting,
                2 => Department::Sales,
                3 => Department::IT,
                i32::MIN..=0_i32 => {
                    println!("{choice} is not one of the options!");
                    continue;
                }
                4_i32..=i32::MAX => {
                    println!("{choice} is not one of the options!");
                    continue;
                }
            };

            list_employees_in_department(&mut employees_departments, department);
        }
    }
}
