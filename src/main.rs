use std::io;
use std::process::Command;

pub mod calculator;
pub mod file_search;

macro_rules! parse
{
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

macro_rules! get_char 
{
    ($x:expr, $n:expr) => //Careful with size
    {
        {
            let chars: Vec<char> = $x.to_string().trim().chars().collect();
            chars[$n]
        }
    };
    ($x:expr) => ($x.to_string().trim().chars().collect())
}

fn main() 
{
    let mut input = String::new();
    println!("Hi!");
    loop 
    {
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string().to_lowercase();
        match input.as_str() 
        {
            "help" =>
            {
                println!("Possible Commands:");
                println!("to_ascii\nascii_to_text\nto_binary\nbinary_to_text\ncalculator\nsearch\nquit\n");
            }
            "to_ascii" =>
            {
                input.clear();
                println!("Type text!:");
                io::stdin().read_line(&mut input).unwrap();
                let ascii = to_ascii(&input.trim());
                println!();
                println!("{}", ascii.0);
                println!();
            }
            "ascii_to_text" =>
            {
                input.clear();
                println!("Type ascii-values (seperated by space)!:");
                io::stdin().read_line(&mut input).unwrap();
                let text = ascii_as_str_to_text(&input.trim());
                println!();
                println!("{}", text);
                println!();
            }
            "to_binary" =>
            {
                input.clear();
                println!("Type text!:");
                io::stdin().read_line(&mut input).unwrap();
                let binary = to_binary(&input.trim());
                println!();
                println!("{}", binary);
                println!();
            }
            "binary_to_text" =>
            {
                input.clear();
                println!("Type binary (seperated by spaces)!:");
                io::stdin().read_line(&mut input).unwrap();
                let text = binary_as_str_to_text(&input.trim());
                println!();
                println!("{}", text);
                println!();
            }
            "search" =>
            {
                input.clear();
                println!("URL (0) / google-search (1) / filesearch (2)!:");
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                match input.as_str() 
                {
                    "0" =>
                    {
                        input.clear();
                        io::stdin().read_line(&mut input).unwrap();
                        let _ = open_url_in_browser(&input);
                        println!();
                    }
                    "1" =>
                    {
                        input.clear();
                        io::stdin().read_line(&mut input).unwrap();
                        let _ = google_search(&input);
                        println!();
                    }
                    "2" =>
                    {
                        file_search::initialize();
                    }
                    _ => println!("0, 1 or 2")
                }
            }
            "calculator" =>
            {
                calculator::initialize();
            }
            "quit" =>
            {
                println!("Bye");
                return;
            }
            _ => 
            {
                println!("Invalid command! Type \"help\" for help");
                println!();
            }
        }
        input.clear();
    }
}

fn to_ascii(str: &str) -> (String, Vec<u8>)
{
    let chars: Vec<char> = get_char!(str);
    let mut output = String::new();
    let mut vec = Vec::new();
    for c in chars.iter()
    {
        output += &(*c as u32).to_string();
        output += ";";
        vec.push(*c as u8);
    }
    (output.trim().to_string(), vec)
}

fn ascii_as_str_to_text(str: &str) -> String //Turns: 240 159 146 150 into 💖
{
    let inputs: Vec<&str> = str.split(";").collect();
    let ascii: Vec<u8> = inputs.iter().map(|x| parse!(x, u8)).collect();
    ascii_to_text(&ascii)
}

fn ascii_to_text(ascii: &Vec<u8>) -> String 
{
    let str = std::str::from_utf8(&ascii).expect("utf-8 sequence invalid");//&[240, 159, 146, 150]
    str.to_string()
}



fn to_binary(text: &str) -> String //Ascii
{
    let mut output = String::new();
    for character in text.chars() 
    {
        let ascii = character as u8;
        let binary = format!("{:08b}", ascii);
        output.push_str(&binary);
        output.push(' ');
    }
    output.trim().to_string()
}

fn binary_as_str_to_text(binary: &str) -> String
{
    let mut text = Vec::new();

    for chunk in binary.split(" ") 
    {
        let decimal = u8::from_str_radix(chunk, 2).expect("Invalid binary");
        text.push(decimal);
    }

    String::from_utf8(text).expect("utf-8 sequence invalid")
}


fn open_url_in_browser(url: &str) -> Result<(), std::io::Error> //Only Windows I think
{
    Command::new("cmd").args(&["/C", "start", "", url]).spawn()?;
    Ok(())
}

fn google_search(query: &str) -> Result<(), std::io::Error> 
{
    let url = format!("https://www.google.com/search?q={}", query);

    open_url_in_browser(&url)
}