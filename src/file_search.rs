//Only on Windows

use std::io::{self, Error};
use std::process::Command;
use std::collections::HashMap;//"C:\\Program Files (x86)\\Steam"

use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

pub fn initialize()
{
    let mut apps: HashMap<String, String> = HashMap::new();
    let arr = load();
    if arr.is_some()
    {
        let words = arr.unwrap();
        for w in words.iter()
        {
            apps.insert(w.0.to_string(), w.1.to_string());
        }
    }
    
    println!("FILE SEARCH:");
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
                println!("files\nadd\nopen\nlist\nquit (quits calculator, returns to previous state)\n");
            }
            "files" =>
            {
                let e = file();
                if e.is_err()
                {
                    println!("You dont have the explorer in your path (The one in windows)!\nIf you want to open it anyways you have to add the filelocation to \"add\"");
                }
                println!();
            }
            "add" =>
            {
                println!("Here you can add a path to an application, you want to open: Type in the [Name], you want to call it and then the path (new line) ( Either: C:\\ or C:/)");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let name = input.to_string();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let path = input.to_string();
                let k = save(&name, &path);
                if k.is_ok()
                {
                    apps.insert(name, path);
                }
                else 
                {
                    println!("Could not save");
                }
            }
            "open" =>
            {
                println!("Type in name of application!:");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                let p = apps.get(&input);
                if p.is_some()
                {
                    let path = p.unwrap().to_string();
                    let k = open(&path);
                    if k.is_err()
                    {
                        println!("Could not open");
                    }
                }
                else 
                {
                    println!("You have to save the program first using \"add\"");
                }
            }
            "list" =>
            {
                for i in apps.iter()
                {
                    println!("{}: {}", i.0, i.1);
                }
            }
            "quit" =>
            {
                println!("Back to previous State");
                println!();
                return;
            }
            _ => 
            {
                
                println!();
            }
        }
        input.clear();
    }
}

fn file() -> Result<(), std::io::Error> 
{
    Command::new("explorer.exe").spawn()?;
    Ok(())
}

fn open(path: &str) -> Result<(), std::io::Error>
{
    // Command::new(path).spawn()?;C:\\Users\PC\\Pictures\\Planet.png
    let temp = &["/C", "start", path];
    Command::new("cmd")
        .args(temp)//&["/C", "start", r#""C:\Program Files (x86)\Steam""#]
        .spawn()?;
    Ok(())
}

fn save(name: &str, path: &str) -> Result<File, Error>
{
    let mut file = OpenOptions::new().write(true).append(true).create(true).open("res/paths.pa")?;
    file.write_all(name.as_bytes())?;
    file.write_all(b";")?;
    file.write_all(path.as_bytes())?;
    file.write_all(b";")?;
    Ok(file)
}

fn load() -> Option<Vec<(String, String)>>
{
    let f = File::open("res/paths.pa");
    let mut arr = Vec::new();
    if f.is_ok()
    {
        let mut file = f.unwrap();
        let mut str = String::new();
        let _ = file.read_to_string(&mut str);
        let words: Vec<&str> = str.split(";").collect();

        for chunk in words.chunks_exact(2)
        {
            if chunk.len() == 2 
            {
                let tuple = (chunk[0].to_string(), chunk[1].to_string());
                arr.push(tuple);
            }
        }
        
        return Some(arr);
    }
    None
}