use crate::commands::{ls::ls,cd::cd,cat::cat,echo::echo, exit::exit, cp::cp,pwd::pwd,mkdir::mkdir,clear::clear};

pub fn handle_command(command: &str, input: &str) -> String {
    match command {
        "echo" | r#""echo""# =>  echo(input),
        "pwd" => pwd(),
        "exit" => exit(),
         "mkdir" => mkdir(input),
          "clear" => clear(),
          "cp"=>cp(input),
          "cat"=>cat(input),
          "ls" => ls(input),
        "cd" => cd(input),
        _ => format!("Handler for '{}' not implemented", command),
    }
}