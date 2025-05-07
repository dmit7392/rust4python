use crate::car_tool_app::CarToolApp;
use std::io::{stdin, stdout, Write};

// 'a is a lifetime parameter that is used to ensure that the reference to the ColorToolApp
// instance is valid for the duration of the CommandLoop instance.
pub struct CommandLoop<'b> {
    app: &'b mut CarToolApp,
}

// 'a is a lifetime parameter that is used to ensure that the reference to the ColorToolApp
// instance is valid for the duration of the CommandLoop instance.
impl<'a> CommandLoop<'a> {
    pub fn new(app: &'a mut CarToolApp) -> CommandLoop<'a> {
        CommandLoop { app }
    }

    fn get_command() -> String {
        let mut command = String::new();

        print!("Enter command: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();

        command.trim().to_string()
    }

    fn command_exit() {
        println!("Exiting...");
    }

    fn command_unknown(command: &str) {
        println!("Unknown command: {}", command);
    }

    pub fn run(&mut self) {
        loop {
            // Self refers to the CommandLoop struct itself.
            // Self::get_command is like a class method in Python
            let command = Self::get_command();
            match command.as_str() {
                "remove" => match self.app.remove_car() {
                    Ok(size) => println!("removed"),
                    Err(err) => println!("can't remove - {err}"),
                },
                "save" => match self.app.save_cars() {
                    Ok(size) => println!("saved"),
                    Err(err) => println!("can't save - {err}"),
                },
                "load" => match self.app.load_cars() {
                    Ok(size) => println!("loaded"),
                    Err(err) => println!("can't load - {err}"),
                },
                "add" => match self.app.add_car() {
                    Ok(size) => println!("car added; total {size}"),
                    Err(err) => println!("try again - {err}"),
                }
                "show" => {
                    self.app.show_cars();
                }
                "exit" => {
                    CommandLoop::command_exit();
                    // Shortcut:
                    // Self::command_exit();
                    break;
                }
                _ => {
                    Self::command_unknown(command.as_str());
                }
            }
        }
    }
}
