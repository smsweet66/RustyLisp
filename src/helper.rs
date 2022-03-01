use std::collections::HashMap;
use std::process::exit;
use substring::Substring;
use crate::Variable;

// breaks up a string into multiple substrings by spaces
// ignores spaces within quotes
pub(crate) fn string_sep(string: String) -> Vec<String>
{
    let mut substrings: Vec<String> = Vec::new();
    let mut i = 0;
    while i < string.len()
    {
        while string.chars().nth(i).unwrap().is_ascii_whitespace()
        {
            i = i + 1;
            if i == string.len()
            { break }
        }

        let start = i;
        if i == string.len()
        { break }
        else if string.chars().nth(i).unwrap() == '\"'
        {
            i = i + 1;
            while string.chars().nth(i).unwrap() != '\"'
            { i = i + 1 }

            i = i + 1;
        }
        else if string.chars().nth(i).unwrap() == '['
        {
            i = i + 1;
            let mut temp = 1;
            while temp > 0
            {
                if string.chars().nth(i).unwrap() == '['
                { temp = temp + 1 }
                else if string.chars().nth(i).unwrap() == ']'
                { temp = temp - 1 }

                i = i + 1;
            }
        }
        else if string.chars().nth(i).unwrap() == '('
        {
            i = i + 1;
            let mut temp = 1;
            while temp > 0
            {
                if string.chars().nth(i).unwrap() == '('
                { temp = temp + 1 }
                else if string.chars().nth(i).unwrap() == ')'
                { temp = temp - 1 }

                i = i + 1;
            }
        }
        else
        {
            while i < string.len() && !string.chars().nth(i).unwrap().is_ascii_whitespace()
            { i = i + 1 }
        }

        substrings.push(string.substring(start, i).to_string());
        i = i + 1;
    }

    return substrings;
}

fn operation(op: String, local_names: &Vec<String>, locals: &mut Vec<Variable>, globals: &mut HashMap<String, Variable>) -> String
{
    let substrings = string_sep(op);
    match &substrings[0] as &str
    {
        "print" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let output = interpret_line(substrings[1].to_string(), local_names, locals, globals);
                println!("{}", Variable::from_string(output, local_names, locals, globals).to_string());
            }
            return String::new();
        }
        "+" | "-" | "*" | "/" | "%" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let a = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                let b = Variable::from_string(interpret_line(substrings[2].to_string(), local_names, locals, globals), local_names, locals, globals);
                match a
                {
                    Variable::Number(f1) => {
                        match b
                        {
                            Variable::Number(f2) => {
                                match substrings[0].as_str()
                                {
                                    "+" => return (f1 + f2).to_string(),
                                    "-" => return (f1 - f2).to_string(),
                                    "*" => return (f1 * f2).to_string(),
                                    "/" => return (f1 / f2).to_string(),
                                    "%" => return (f1 % f2).to_string(),
                                    _ => {
                                        println!("Invalid operation!");
                                        exit(1);
                                    }
                                }
                            },
                            _ => {
                                println!("Cannot perform arithmetic operations on non number value!");
                                exit(15);
                            }
                        }
                    },
                    _ => {
                        println!("Cannot perform arithmetic operations on non number value!");
                        exit(15);
                    }
                }
            }
        },
        "=" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            } else {
                let a = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                let b = Variable::from_string(interpret_line(substrings[2].to_string(), local_names, locals, globals), local_names, locals, globals);
                return if a.eq(&b)
                { "t".to_string() } else { "[]".to_string() }
            }
        },
        "<" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let a = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
            let b = Variable::from_string(interpret_line(substrings[2].to_string(), local_names, locals, globals), local_names, locals, globals);
            match a
            {
                Variable::Number(f1) => {
                    match b
                    {
                        Variable::Number(f2) => {
                            return if f1 - f2 < -0.000000001
                            { String::from('t') } else { String::from("[]") }
                        },
                        _ => {
                            println!("Cannot perform arithmetic operations on non number value!");
                            exit(15);
                        }
                    }
                },
                _ => {
                    println!("Cannot perform arithmetic operations on non number value!");
                    exit(15);
                }
            }
        },
        ">" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let a = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
            let b = Variable::from_string(interpret_line(substrings[2].to_string(), local_names, locals, globals), local_names, locals, globals);
            match a
            {
                Variable::Number(f1) => {
                    match b
                    {
                        Variable::Number(f2) => {
                            return if f1 - f2 > 0.000000001
                            { String::from('t') } else { String::from("[]") }
                        },
                        _ => {
                            println!("Cannot perform arithmetic operations on non number value!");
                            exit(15);
                        }
                    }
                },
                _ => {
                    println!("Cannot perform arithmetic operations on non number value!");
                    exit(15);
                }
            }
        },
        "set" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let name = substrings[1].clone();
            if !name.chars().nth(0).unwrap().is_ascii_alphabetic()
            {
                println!("Variable name must start with a letter!");
                exit(4);
            }
            else if name == "nil" || name == "t"
            {
                println!("A reserved word cannot be the name of a variable!");
                exit(5);
            }
            else
            {
                let value = interpret_line(substrings[2].clone(), local_names, locals, globals);
                let var = Variable::from_string(value, local_names, locals, globals);
                if local_names.contains(&name)
                { locals.insert(local_names.iter().position(|s| s == &name).unwrap(), var) } else { globals.insert(name, var); }
            }

            return "".to_string();
        },
        "if" => {
            if substrings.len() != 4
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let condition = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                match condition
                {
                    Variable::True => return interpret_line(substrings[2].to_string(), local_names, locals, globals),
                    Variable::List(l) => {
                        if l.len() == 0
                        { return interpret_line(substrings[3].to_string(), local_names, locals, globals) } else {
                            println!("Empty list expected, got populated list!");
                            exit(9);
                        }
                    }
                    _ => {
                        println!("Invalid type returned!  T or [] expected");
                        exit(9);
                    }
                }
            }
        },
        "while" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            loop
            {
                let condition = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                match condition
                {
                    Variable::True => { interpret_line(substrings[2].to_string(), local_names, locals, globals); },
                    Variable::List(l) => {
                        if l.len() == 0
                        { break }
                        else
                        {
                            println!("Empty list expected, got populated list!");
                            exit(9);
                        }
                    }
                    _ => {
                        println!("Invalid type returned!  T or [] expected");
                        exit(9);
                    }
                }
            }

            return "".to_string();
        },
        "begin" => {
            if substrings.len() < 2
            {
                println!("begin must have at least 2 arguments!");
                exit(3);
            }
            else
            {
                for i in 1..substrings.len()
                { interpret_line(substrings[i].to_string(), local_names, locals, globals); }

                return "".to_string();
            }
        },
        "cons" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let car = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                let cdr = Variable::from_string(interpret_line(substrings[2].to_string(), local_names, locals, globals), local_names, locals, globals);
                let mut cons: Vec<Variable> = Vec::new();
                match cdr
                {
                    Variable::List(l) => {
                        cons.reserve(l.len() + 1);
                        cons.push(car);
                        for var in l
                        { cons.push(var); }
                    }
                    _ => {
                        cons.reserve(2);
                        cons.push(car);
                        cons.push(cdr);
                    }
                }

                return Variable::List(cons).to_string();
            }
        },
        "car" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let cons = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                match cons
                {
                    Variable::List(l) => return l[0].to_string(),
                    _ => {
                        println!("car can only be performed on a list!");
                        exit(10);
                    }
                }
            }
        },
        "cdr" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let cons = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                match cons
                {
                    Variable::List(l) => {
                        return if l.len() == 1
                        { "[]".to_string() } else {
                            let mut cdr: Vec<Variable> = Vec::with_capacity(l.len() - 1);
                            for i in 1..l.len()
                            { cdr.push(l[i].clone()); }

                            Variable::List(cdr).to_string()
                        }
                    },
                    _ => {
                        println!("car can only be performed on a list!");
                        exit(10);
                    }
                }
            }
        },
        "number?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            } else {
                let var = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                return match var
                {
                    Variable::Number(_) => "t".to_string(),
                    _ => "[]".to_string()
                }
            }
        },
        "symbol?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                return if local_names.contains(&substrings[0])
                { "t".to_string() }
                else
                {
                    let container = globals.get(&substrings[1]);
                    match container
                    {
                        Some(_) => "t".to_string(),
                        None => "[]".to_string()
                    }
                }
            }
        },
        "list?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            } else {
                let var = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                return match var
                {
                    Variable::List(_) => "t".to_string(),
                    _ => "[]".to_string()
                }
            }
        },
        "null?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            } else {
                let var = Variable::from_string(interpret_line(substrings[1].to_string(), local_names, locals, globals), local_names, locals, globals);
                return match var
                {
                    Variable::List(l) => {
                        if l.len() == 0
                        { "t".to_string() } else { "[]".to_string() }
                    },
                    _ => "[]".to_string()
                }
            }
        },
        _ => {
            let var;
            if local_names.contains(&substrings[0])
            { var = locals[local_names.iter().position(|s| s == &substrings[0]).unwrap()].clone(); }
            else
            { var = globals[&substrings[0]].clone(); }

            match var
            {
                Variable::Func(f) => {
                    let mut arguments: Vec<Variable> = Vec::with_capacity(substrings.len() - 1);
                    for i in 1..substrings.len()
                    { arguments.push(Variable::from_string(interpret_line(substrings[i].clone(), local_names, locals, globals), local_names, locals, globals)); }

                    f.run(&mut arguments, globals);
                    return "".to_string();
                },
                _ => {
                    println!("{} is not a valid function!", substrings[0]);
                    exit(1);
                }
            }
        }
    }
}

pub fn interpret_line(line: String, local_names: &Vec<String>, locals: &mut Vec<Variable>, globals: &mut HashMap<String, Variable>) -> String
{
    return if line.chars().nth(0).unwrap() == '('
    { operation(line.substring(1, line.len() - 1).to_string(), local_names, locals, globals) }
    else
    { line }
}