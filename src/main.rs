mod kirchhoff_law;
use kirchhoff_law::structure;
use kirchhoff_law::solver;

use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input_str = fs::read_to_string(file_path);
    
    let input = match input_str{
        Err(_) => {
            println!("Input Error");
            return;
        },
        Ok(input) => {input}
    };

    let Some(circuit) = structure::get_circuit(input) else {println!("Input Error"); return;};

    circuit.print();

    let problem= solver::set_equation::get_problem(circuit);
    
    problem.print();

}
