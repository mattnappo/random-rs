use std::io;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_string(operation: &str) -> Option<Self> {
        return match operation {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => {
                println!("Invalid operation!");
                None
            }
        };
    }
}

struct Calculator {
    x: f64,
    y: f64,
    operation: Operation,
}

impl Calculator {
    fn calculate(&self) -> Option<f64> {
        return match self.operation {
            Operation::Add => Some(self.x + self.y),
            Operation::Subtract => Some(self.x - self.y),
            Operation::Multiply => Some(self.x * self.y),
            Operation::Divide => match self.y {
                0.0 => None,
                _ => Some(self.x / self.y),
            },
        };
    }

    fn get_number() -> f64 {
        loop {
            let mut n = String::new();
            println!("Enter a number: ");
            io::stdin()
                .read_line(&mut n)
                .expect("Could not read a line");

            match n.trim().parse() {
                Ok(i) => return i,
                Err(_) => {
                    println!("That is not a number!");
                    continue;
                }
            };
        }
    }

    fn get_operation() -> Operation {
        loop {
            let mut operation = String::new();
            println!("Enter an operation: ");
            io::stdin()
                .read_line(&mut operation)
                .expect("Could not read a line");
            match Operation::from_string(operation.trim()) {
                Some(op) => return op,
                None => continue,
            }
        }
    }

    fn start() -> Self {
        return Self {
            x: Calculator::get_number(),
            y: Calculator::get_number(),
            operation: Calculator::get_operation(),
        };
    }
}

fn main() {
    println!("Welcome to Matt's Own Calculator");
    loop {
        let calculator = Calculator::start();
        println!("Calculation: {}", calculator.calculate().unwrap());
    }
}
