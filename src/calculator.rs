use std::io;

const OPERATORS: &str = "+-*/";

macro_rules! parse
{
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn initialize()
{
    println!("CALCULATOR:");
    let mut input = String::new();
    loop 
    {
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string().to_lowercase();
        match input.as_str()
        {
            "help" =>
            {
                println!("Possible Commands:");
                println!("Type Operations (for example: 1+1) !!Does only support \"+-*/\" and does not follow Pegma (ust calculates from left to right)!! \nquit (quits calculator, returns to previous state)\n");
            }
            "quit" =>
            {
                println!("Back to previous State");
                println!();
                return;
            }
            _ => 
            {
                parse(&input);
                println!();
            }
        }
        input.clear();
    }
}

fn parse(input: &str) //A lot of code here is probably quite unneccesary (meaning you can shorten it a lot))
{
    let temp: Vec<&str> = input.split_whitespace().collect();
    let mut operation = String::new();
    for s in temp.iter() { operation.push_str(*s); }
    let chars: Vec<char> = operation.chars().collect();
    let mut output = String::new();

    for c in chars.iter()
    {
        match *c
        {
            '0' => output.push(*c),
            '1' => output.push(*c),
            '2' => output.push(*c),
            '3' => output.push(*c),
            '4' => output.push(*c),
            '5' => output.push(*c),
            '6' => output.push(*c),
            '7' => output.push(*c),
            '8' => output.push(*c),
            '9' => output.push(*c),
            _ => 
            {
                output.push(' ');
                if OPERATORS.contains(*c)
                {
                    output.push(*c);
                    output.push(' ');
                }
                else 
                {
                    println!("Symbol not known: {}", c);   
                }
            }
        }
    }

    let symbols: Vec<&str> = output.split_whitespace().collect();
    let mut op = Vec::new();
    let mut di = Vec::new();
    for symbol in symbols.iter()
    {
        let chars: Vec<char> = symbol.chars().collect();
        let mut digits = String::new();
        for c in chars.iter()
        {
            if OPERATORS.contains(*c)
            {
                op.push(*c);
            }
            else 
            {
                digits.push(*c);
            }
        }
        if !digits.is_empty()
        {
            di.push(parse!(digits, f32));
        }
    }

    let result = di.iter().skip(1).zip(op.iter()).fold(di[0], |acc, (&x, &y)| 
    {
        match y 
        {
            '+' => acc + x,
            '-' => acc - x,
            '*' => acc * x,
            '/' => acc / x,
            _ => acc,
        }
    });

    println!("{} = {}", input, result);
}