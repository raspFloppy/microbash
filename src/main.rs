use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Child, Command};

struct CommandLine {
    command: String,
    args: Vec<String>,
}

impl CommandLine {
    fn new(input: String) -> Self {
        let mut parts = input.split_whitespace();

        CommandLine {
            command: parts.next().unwrap_or("").to_string(),
            args: parts.map(|arg| arg.to_owned()).collect(),
        }
    }

    fn get_command(&self) -> &str {
        self.command.trim()
    }

    fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn cd_command(&self) {
        if self.args.len() > 1 {
            eprintln!("cd: too many arguments");
            return;
        }

        let new_dir;
        let path;

        match self.args.first() {
            Some(arg) if arg == "-" => {
                path = env::var("OLDPWD").unwrap();
                new_dir = Path::new(&path);
                println!("{}\n", env::var("OLDPWD").unwrap());
            }
            Some(arg) if arg == "~" => {
                path = env::var("HOME").unwrap();
                new_dir = Path::new(&path);
            }
            Some(arg) => {
                path = arg.clone();
                new_dir = Path::new(&path);
            }
            None => {
                path = env::var("HOME").unwrap();
                new_dir = Path::new(&path);
            }
        }

        match env::set_current_dir(new_dir).is_ok() {
            true => {
                env::set_var("OLDPWD", env::var("PWD").unwrap());
                env::set_var("PWD", env::current_dir().unwrap());
            }
            false => {
                eprintln!("cd: No such file or directory: {}", path);
            }
        }
    }

    fn execute(&self) -> Result<Child, std::io::Error> {
        let command = Command::new(self.command.clone())
            .args(self.args.clone())
            .spawn()?;

        Ok(command)
    }
}

fn main() -> io::Result<()> {
    loop {
        let pwd = env::current_dir()?;
        print!("{}$ ", pwd.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let cli = CommandLine::new(input);
        let command = cli.get_command().trim();

        match command.trim() {
            "exit" => break,
            "cd" => {
                cli.cd_command();
            }
            _ => {
                if command.is_empty() {
                    continue;
                } else {
                    #[allow(unused_variables)]
                    match cli.execute() {
                        Ok(child) => {
                            let output = child.wait_with_output()?;
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            print!("{}", stdout);
                        }
                        Err(error) => {
                            eprintln!("microbash: command not found: {}", command);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
