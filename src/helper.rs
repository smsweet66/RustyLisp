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
        { i = i + 1 }

        let mut start = i;
        if string.chars().nth(i).unwrap() == '\"'
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

// given a string, will extract the float value
// works if string is a float or the name of a float variable
fn get_float_value(value: String, variables: &HashMap<String, Variable>) -> f64
{
    let container = value.parse::<f64>();
    match container
    {
        Ok(v) => return v,
        _ => {
            let temp = variables.get(value.as_str());
            match temp
            {
                Some(v) => {
                    match v
                    {
                        Variable::Number(f) => return *f,
                        _ => {
                            println!("{} is not a float", value);
                            exit(2);
                        }
                    }
                }
                None => {
                    println!("{} is not a valid variable!", value);
                    exit(3);
                }
            }
        }
    }
}

fn is_type(type_string: &str, var: Variable) -> &str
{
    return match type_string
    {
        "num" => {
            match var
            {
                Variable::Number(_) => "t",
                _ => "nil"
            }
        },
        _ => {
            println!("{} is not a valid type!", type_string);
            exit(11);
        }
    }
}

fn operation(op: String, variables: &mut HashMap<String, Variable>) -> String
{
    let substrings = string_sep(op);
    match &substrings.get(0).unwrap() as &str
    {
        "print" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let output = interpret_line(substrings.get(1).unwrap().to_string(), variables);
                println!("{}", Variable::from_string(output, variables).to_string());
            }
            return String::new();
        }
        "+"|"-"|"*"|"/"|"%" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let a = get_float_value(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                let b = get_float_value(interpret_line(substrings.get(2).unwrap().to_string(), variables), variables);
                match substrings.get(0).unwrap().as_str()
                {
                    "+" => return (a + b).to_string(),
                    "-" => return (a - b).to_string(),
                    "*" => return (a * b).to_string(),
                    "/" => return (a / b).to_string(),
                    "%" => return (a % b).to_string(),
                    _ => {
                        println!("Invalid operation!");
                        exit(1);
                    }
                }
            }
        },
        "=" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let a = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                let b = Variable::from_string(interpret_line(substrings.get(2).unwrap().to_string(), variables), variables);
                return if a.eq(&b)
                { "t".to_string() } else { "nil".to_string() }
            }
        },
        "<" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let a = get_float_value(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
            let b = get_float_value(interpret_line(substrings.get(2).unwrap().to_string(), variables), variables);
            return if a - b < -0.000000001
            { String::from('t') } else { String::from("nil") }
        },
        ">" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let a = get_float_value(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
            let b = get_float_value(interpret_line(substrings.get(2).unwrap().to_string(), variables), variables);
            return if a - b > 0.000000001
            { String::from('t') } else { String::from("nil") }
        },
        "set" => {
            if substrings.len() != 3
            {
                println!("Wrong number of arguments!");
                exit(3);
            }

            let name = substrings.get(1).unwrap().clone();
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
                let value = interpret_line(substrings.get(2).unwrap().clone(), variables);
                let var = Variable::from_string(value, variables);
                variables.insert(name, var);
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
                let condition = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                match condition
                {
                    Variable::Bool(b) => {
                        return if b
                        { interpret_line(substrings.get(2).unwrap().to_string(), variables) }
                        else
                        { interpret_line(substrings.get(3).unwrap().to_string(), variables) }
                    },
                    _ => {
                        println!("Invalid type returned!  Bool expected");
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
                let condition = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                match condition
                {
                    Variable::Bool(b) => {
                        if b
                        { interpret_line(substrings.get(2).unwrap().to_string(), variables); }
                        else
                        { break; }
                    }
                    _ => break
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
                { interpret_line(substrings.get(i).unwrap().to_string(), variables); }

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
                let car = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                let cdr = Variable::from_string(interpret_line(substrings.get(2).unwrap().to_string(), variables), variables);
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
                let cons = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                match cons
                {
                    Variable::List(l) => return l.get(0).unwrap().to_string(),
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
                let cons = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                match cons
                {
                    Variable::List(l) => {
                        return if l.len() == 1
                        { Variable::Bool(false).to_string() }
                        else
                        {
                            let mut cdr: Vec<Variable> = Vec::with_capacity(l.len() - 1);
                            for i in 1..l.len()
                            { cdr.push(l.get(i).unwrap().clone()); }

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
            }
            else
            {
                let var = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                return is_type("num", var).to_string();
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
                let container = variables.get(substrings.get(1).unwrap());
                return match container
                {
                    Some(_) => "t".to_string(),
                    None => "nil".to_string()
                }
            }
        },
        "list?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let var = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                return match var
                {
                    Variable::List(_) => "t".to_string(),
                    _ => "nil".to_string()
                }
            }
        },
        "null?" => {
            if substrings.len() != 2
            {
                println!("Wrong number of arguments!");
                exit(3);
            }
            else
            {
                let var = Variable::from_string(interpret_line(substrings.get(1).unwrap().to_string(), variables), variables);
                return match var
                {
                    Variable::Bool(b) => {
                        if !b
                        { "t".to_string() }
                        else
                        { "nil".to_string() }
                    },
                    Variable::List(l) => {
                        if l.len() == 0
                        { "t".to_string() }
                        else
                        { "nil".to_string() }
                    },
                    _ => "nil".to_string()
                }
            }
        },
        _ => {
            let var = variables.get(substrings.get(0).unwrap()).unwrap().clone();
            match var
            {
                Variable::Func(f) => {
                    let mut arguments: Vec<Variable> = Vec::with_capacity(substrings.len()-1);
                    for i in 1..substrings.len()
                    { arguments.push(Variable::from_string(interpret_line(substrings.get(i).unwrap().clone(), variables), variables)); }

                    f.run(arguments, variables);
                    return "".to_string();
                },
                _ => {
                    println!("{} is not a valid function!", substrings.get(0).unwrap());
                    exit(1);
                }
            }
        }
    }
}

pub fn interpret_line(line: String, variables: &mut HashMap<String, Variable>) -> String
{
    return if line.chars().nth(0).unwrap() == '('
    { operation(line.substring(1, line.len() - 1).to_string(), variables) }
    else
    { line }
}