use regex::Regex;
use std::iter::zip;


#[derive(Debug)]
pub enum Element{
    R(f64),
    C(f64),
    V(f64)
}

impl Element{
    pub(crate) fn i_position(&self)-> usize {
        match *self{
            Element::R(_) => 0 as usize,
            Element::C(_) => 0 as usize,
            Element::V(_) => 0 as usize
        }
    }

    pub(crate) fn voltage_drop(&self) -> Vec<f64>{
        match *self{
            Element::R(r) => vec![r],
            Element::C(c) => vec![0.0,1.0/c],
            Element::V(v) => vec![0.0,-v],
        }
    }

    pub(crate) fn mitigation_vector(&self) -> Vec<Vec<f64>>{
        match *self{
            Element::R(_) => vec![vec![0.0]],
            Element::C(_) => vec![vec![0.0,0.0],vec![1.0,0.0]],
            Element::V(_) => vec![vec![0.0,0.0],vec![0.0,0.0]]
        }
    }
}

#[derive(Debug)]
pub enum Variable{
    I(f64),
    Q(f64),
    V(f64)
}

pub fn get_variables(element : &Element) -> (Vec<Variable>, Vec<bool>){
    match element{
        Element::R(_) => (vec![Variable::I(0.0)], vec![false]),
        Element::C(_) => (vec![Variable::I(0.0), Variable::Q(0.0)], vec![false, true]),
        Element::V(_) => (vec![Variable::I(0.0), Variable::V(0.0)], vec![false, true])
    }
}//"I" must be the first variables

pub struct Circuit{
    pub elements : Vec<Element>,
    pub connections : Vec<(i32,i32)>,
    pub labels : Vec<i32>
}

impl Circuit{
    pub(crate) fn print(&self){
        for ((element, connection), label) in zip(zip(&self.elements, &self.connections), &self.labels){
            println!("{:?}{:.2} is connected between {}, {}", element, label, connection.0, connection.1);
        }
    }
}

// r"(?P<type>[A-Z])(?P<index>[0-9]*)\((?P<value>[0-9]*)\) : (?P<connection_index_1>[0-9]*),(?P<connection_index_2>[0-9]*)
pub fn get_circuit(input_string : String)-> Option<Circuit> {

    let re= Regex::new(r"(?P<type>[A-Z]*)(?P<index>[0-9.]*)\((?P<value>[0-9.]*)\) : (?P<connection_index_1>[0-9]*),(?P<connection_index_2>[0-9]*)").unwrap();

    let mut elements: Vec<Element>= Vec::new();
    let mut connections : Vec<(i32,i32)> = Vec::new();
    let mut labels : Vec<i32> = Vec::new();

    for element_string in (&input_string[..]).split('\n'){

        let element_data:Vec<_> = re.captures_iter(element_string).map(|caps|{
            let element_type = caps.name("type");
            let element_index = caps.name("index");
            let element_value = caps.name("value");
            let connection_index_1 = caps.name("connection_index_1");
            let connection_index_2 = caps.name("connection_index_2");
            (element_type, element_index, element_value, connection_index_1, connection_index_2)
        }).collect();

        let (Some(t), Some(i), Some(v), Some(j1), Some(j2)) = element_data[0] else{println!("Check Input {}", element_string); return None;};

        let element_value: f64 = v.as_str().parse().unwrap();
        let element_type = match t.as_str(){
            "R" => Element::R(element_value),
            "C" => Element::C(element_value),
            "V" => Element::V(element_value),
            _ => {
                println!("Check Input {}", element_string); return None;
            }
        };
        let element_index: i32 = i.as_str().parse().unwrap();

        let connection_index_1: i32 = j1.as_str().parse().unwrap();
        let connection_index_2: i32 = j2.as_str().parse().unwrap();

        elements.push(element_type);
        connections.push((connection_index_1,connection_index_2));
        labels.push(element_index);
    }

    Some(Circuit{
        elements,
        connections,
        labels,
    })


}