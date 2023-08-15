use super::super::structure;
extern crate queues;
use queues::*;

pub struct Status{
    variables : Vec<structure::Variable>,
    is_fix : Vec<bool>, //variables
    labels: Vec<Vec<i32>> //labels[i] is the variables index of element i
}

impl Status{
    fn print(&self){
        println!("Status");
        println!("{:?}", self.variables);
        println!("{:?}", self.is_fix);
        println!("{:?}", self.labels);
    }
}

pub(crate) struct Problem{
    status : Status,
    kirchhoff_matrix : Vec<Vec<f64>>,
    mitigation_matrix : Vec<Vec<f64>>,
}

impl Problem{
    /**
    Returns the print of this [`Problem`].
    */
    pub(crate) fn print(&self){
        self.status.print();
        println!("Kirchhoff matrix");
        print_2dim_vec(&self.kirchhoff_matrix);
        println!("Mitigation matrix");
        print_2dim_vec(&self.mitigation_matrix);
    }
}

fn print_2dim_vec(v2 : &Vec<Vec<f64>>){
    for v in v2{
        for x in v{
            print!("{:>+.2}    " ,x);
        }
        println!("");
    }
}

pub(crate) fn get_problem(circuit: structure::Circuit) -> Problem{
    let status = get_status(&circuit);
    let row =status.variables.len();
    let mut column : i32 =0;
    for b in &status.is_fix{
        if !b{
            column+=1;
        }
    }
    // row element number and column element number

    let kirchhoff_matrix = get_kirchhoff_matrix(&circuit, &status.labels,row, column as usize);
    let mitigation_matrix = get_mitigation_matrix(&circuit, &status.labels, row , row);

    Problem { status, kirchhoff_matrix, mitigation_matrix }
}


fn get_status(circuit : &structure::Circuit)-> Status {
    let mut is_fix: Vec<bool> = Vec::new();
    let mut variables : Vec<structure::Variable> = Vec::new(); 
    let mut labels : Vec<Vec<i32>> = Vec::new();

    for element in &circuit.elements{
        let (v,b)=structure::get_variables(element);
        let initial_size= variables.len() as i32;
        variables.extend(v);
        is_fix.extend(b);
        let final_size=variables.len() as i32;
        
        let mut label: Vec<i32> = Vec::new();
        for index in initial_size..final_size{
            label.push(index);
        }
        labels.push(label);
    }

    Status{
        variables,
        is_fix,
        labels
    }

}

fn get_kirchhoff_matrix(circuit:&structure::Circuit, status_labels : &Vec<Vec<i32>>,row: usize,column: usize) -> Vec<Vec<f64>>{
    let mut kirchhoff_matrix : Vec<Vec<f64>> = vec![vec![0.0;row];column];
    let connections = &circuit.connections;
    let mut max_index = 0;

    for (index,element) in circuit.elements.iter().enumerate(){
        let connection=connections[index];
        if connection.0-1 >max_index{
            max_index = connection.0-1;
        }
        if connection.1-1 > max_index{
            max_index = connection.1-1;
        }

        kirchhoff_matrix[(connection.0-1)as usize][status_labels[index][element.i_position()] as usize] = 1.0;
        kirchhoff_matrix[(connection.1-1) as usize][status_labels[index][element.i_position()] as usize] = -1.0;
    }

    let mut kirchhoff_loop_index = max_index;

    for index in 0..kirchhoff_matrix[kirchhoff_loop_index as usize].len(){
        kirchhoff_matrix[kirchhoff_loop_index as usize][index as usize] = 0.0;
    }


    let mut node_element_vector :Vec<Vec<(i32, bool)>> = vec![vec![];(max_index+2) as usize]; // node_element_vector[node] = {elements, linked to the node}

    for (index,connection) in connections.iter().enumerate(){
        node_element_vector[connection.0 as usize].push((index as i32, true));
        node_element_vector[connection.1 as usize].push(((index as i32), false));
    }



    let mut visit: Vec<bool> = vec![false;circuit.elements.len()];
    
    let mut dependant_cycles: Vec<Vec<(i32,bool)>> = vec![];

    loop{
        let mut unused_element_index: Option<usize> = None;

        // println!("dependant_cycles: {:?}, visit : {:?}", dependant_cycles, visit);

        for (element_index, visited) in visit.iter().enumerate(){
            if !visited{
                unused_element_index = Some(element_index);
                break;
            }
        }

        let unused_element_index = match unused_element_index {
            Some(x) => x as i32,
            None => break
        };
        visit[unused_element_index as usize] = true;

        let mut stack: Vec<Vec<(i32, bool)>> = vec![];
        let mut loop_visit: Vec<bool> = vec![false; circuit.elements.len()+1];
        loop_visit[0] = true;

        stack.push(vec![(unused_element_index,true)]);
        loop_visit[unused_element_index as usize] = true;
        
        let cycle_found: Option<Vec<(i32,bool)>> = loop{

            let unfinished_loop= match stack.pop(){
                None => break None,
                Some(unfinished_loop) => unfinished_loop
            };
            
            let &(last_element,is_aligned): &(i32,bool) = unfinished_loop.last().expect("kirchhoff loop find issue");
            let mut flag = false;
            


            if !is_aligned{
                for &next_elements in node_element_vector[connections[last_element as usize].0 as usize].iter(){
                    if next_elements.0 == unused_element_index{
                        if unfinished_loop.len() >1{
                            flag = true;
                            break;
                        }
                    }

                    if !loop_visit[next_elements.0 as usize]{
                        loop_visit[next_elements.0 as usize] = true;
                        let mut new_loop = unfinished_loop.clone();
                        new_loop.push(next_elements);
                        let next_loop = new_loop;
                        stack.push(next_loop);

                    }
                }
            } else {
                for &next_elements in node_element_vector[connections[last_element as usize].1 as usize].iter(){
                    if next_elements.0 == unused_element_index{
                        if unfinished_loop.len() >1{
                            flag = true;
                            break;
                        }
                    }

                    if !loop_visit[next_elements.0 as usize]{
                        loop_visit[next_elements.0 as usize] = true;
                        let mut new_loop = unfinished_loop.clone();
                        new_loop.push(next_elements);
                        let next_loop = new_loop;
                        stack.push(next_loop);
                    }
                }
            }

            if flag{
                break Some(unfinished_loop);
            }
        };

        match cycle_found{
            None => {},
            Some(v) => {
                for &element_index in &v{
                    visit[element_index.0 as usize]= true;
                }
                dependant_cycles.push(v);
            }
        }
    }


    for cycle in dependant_cycles{
        for element_index in cycle{
            let voltage_drop_vector= if element_index.1{
                circuit.elements[element_index.0 as usize].voltage_drop()
            } else{
                let mut x:Vec<f64> = vec![];
                for y in circuit.elements[element_index.0 as usize].voltage_drop(){
                    x.push(-y);
                }
                x
            };

            for (target, value) in status_labels[element_index.0 as usize].iter().zip(voltage_drop_vector){
                kirchhoff_matrix[kirchhoff_loop_index as usize][*target as usize] = value;
            }
        }
        kirchhoff_loop_index+=1;
    }


    kirchhoff_matrix
}



fn get_mitigation_matrix(circuit:&structure::Circuit, status_labels : &Vec<Vec<i32>>, row: usize,column : usize)-> Vec<Vec<f64>>{
    let mut mitigation_matrix : Vec<Vec<f64>> = vec![vec![0.0;row];column];
    for (element_index,element) in circuit.elements.iter().enumerate(){
        let mitigation_block=element.mitigation_vector();
        status_labels[element_index]
    }
    mitigation_matrix
}