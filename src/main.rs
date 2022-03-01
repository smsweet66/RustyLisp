mod helper;

use std::collections::HashMap;
use std::process::exit;
use substring::Substring;
use crate::helper::*;

// defines how lisp functions are handled in this interpreter
pub struct Func
{
    parameter_names: Vec<String>,
    body: String
}

impl Func
{
    // creates a Func object based with the given parameter names and body
    pub fn new(parameter_names: Vec<String>, body: String) -> Self
    { Self {parameter_names, body} }

    // creates a copy of the Func object
    pub fn clone(&self) -> Self
    { Self {parameter_names: self.parameter_names.clone(), body: self.body.clone()} }

    // runs the function described in the body based on the given parameter values
    pub fn run(&self, parameters: &mut Vec<Variable>, globals: &mut HashMap<String, Variable>)
    {
        if parameters.len() != self.parameter_names.len()
        {
            println!("Wrong number of arguments.  {} expected, received {}.", self.parameter_names.len(), parameters.len());
            exit(13);
        }

        interpret_line(self.body.clone(), &self.parameter_names, parameters, globals);
    }
}

// The basis for the interpreter's type system
pub enum Variable
{
    Number(f64),
    String(String),
    List(Vec<Variable>),
    Func(Func),
    True
}

impl Variable
{
    // makes a copy of the variable
    pub fn clone(&self) -> Self
    {
        return match self
        {
            Variable::Number(f) => Variable::Number(f.clone()),
            Variable::String(s) => Variable::String(s.clone()),
            Variable::List(l) => {
                let mut list: Vec<Variable> = Vec::with_capacity(l.len());
                for var in l
                { list.push(var.clone()) }

                Variable::List(list)
            },
            Variable::True => Variable::True,
            Variable::Func(f) => Variable::Func(f.clone())
        }
    }

    // checks if the variables are equal
    pub fn eq(&self, other: &Self) -> bool
    {
        return match self
        {
            Variable::Number(f1) => {
                match other
                {
                    Variable::Number(f2) => f1 == f2,
                    _ => false
                }
            },
            Variable::String(s1) => {
                match other
                {
                    Variable::String(s2) => s1 == s2,
                    _ => false
                }
            },
            Variable::List(l1) => {
                match other
                {
                    Variable::List(l2) => {
                        return if l1.len() != l2.len()
                        { false }
                        else
                        {
                            for i in 0..l1.len()
                            {
                                if !l1[i].eq(&l2[i])
                                { return false }
                            }

                            true
                        }
                    },
                    _ => false
                }
            },
            Variable::True => {
                match other
                {
                    Variable::True => true,
                    _ => false
                }
            }
            _ => false
        }
    }

    // creates a Variable object based on the given string
    // string can be a variable name or a literal
    pub fn from_string(input: String, local_names: &Vec<String>, locals: &Vec<Variable>, globals: &HashMap<String, Variable>) -> Self
    {
        return if input.chars().nth(0).unwrap() == '\"'
        { Variable::String(input.clone()) }
        else if input.chars().nth(0).unwrap() == '['
        {
            let substrings = string_sep(input.substring(1, input.len() - 1).to_string());
            let mut vec: Vec<Variable> = Vec::with_capacity(substrings.len());
            for s in substrings
            { vec.push(Variable::from_string(s, local_names, locals, globals)) }

            Variable::List(vec)
        }
        else if input == "t"
        { Variable::True }
        else
        {
            let value = input.parse::<f64>();
            match value
            {
                Ok(v) => Variable::Number(v.clone()),
                _ => {
                    if local_names.contains(&input)
                    { locals[local_names.iter().position(|s| s == &input).unwrap()].clone() }
                    else
                    {
                        let container = globals.get(input.as_str());
                        match container
                        {
                            Some(v) => v.clone(),
                            None => Variable::List(Vec::new())
                        }
                    }
                }
            }
        }
    }

    // returns the contents of the Variable as a string
    pub fn to_string(&self) -> String
    {
        return match self
        {
            Variable::Number(f) => f.to_string(),
            Variable::String(s) => s.to_string(),
            Variable::List(l) => {
                let mut output = "[ ".to_string();
                for var in l
                { output = output + var.to_string().as_str() + " " }

                output = output + "]";
                output
            },
            Variable::True => "t".to_string(),
            _ => "".to_string()
        }
    }
}

// the main function
// does the basic setup
fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2
    {
        println!("Incorrect number of arguments. \n\
                  Correct usage:  {} *.lsp", args[0]);
        exit(0);
    }
    let input = std::fs::read_to_string(&args[1]).expect("Could not read file!")
        .replace("()", "[]").to_lowercase();
    let split = string_sep(input);

    let mut globals: HashMap<String, Variable> = HashMap::new();
    for line in split
    {
        let substrings = string_sep(line.substring(1, line.len()-1).to_string());
        if substrings[0] == "define"    // define is handled specially, not with the other functions
        {
            if substrings.len() != 4
            {
                println!("Incorrect number of arguments for defining a function!");
                exit(12);
            }
            else
            {
                let temp = &substrings[2];
                let arguments = string_sep(temp.substring(1, temp.len()-1).to_string());
                let function = Func::new(arguments, substrings[3].clone());
                globals.insert(substrings[1].clone(), Variable::Func(function));
            }
        }
        else
        { interpret_line(line, &Vec::new(), &mut Vec::new(), &mut globals); }
    }
}