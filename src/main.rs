use std::thread;
use std::io;
use std::time::Duration;
use console::Term;
use console::style;
use dialoguer::Select;
use std::process::Command;
fn 
fn main(){
    let term = Term::stdout();
    let _= term.write_line(" 
 ___  ____                                __    _________               __   __        _   _    
|_  ||_  _|                              [  |  |  _   _  |             [  | [  |  _   (_) / |_  
  | |_/ /   .---.  _ .--.  _ .--.  .---.  | |  |_/ | | \\_|.--.    .--.  | |  | | / ]  __ `| |-' 
  |  __'.  / /__\\\\[ `/'`\\][ `.-. |/ /__\\\\ | |      | |  / .'`\\ \\/ .'`\\ \\| |  | '' <  [  | | |   
 _| |  \\ \\_| \\__., | |     | | | || \\__., | |     _| |_ | \\__. || \\__. || |  | |`\\ \\  | | | |,  
|____||____|'.__.'[___]   [___||__]'.__.'[___]   |_____| '.__.'  '.__.'[___][__|  \\_][___]\\__/  
                           ");
    //thread::sleep(Duration::from_millis(2000));
    /*
    match term.clear_line() {
        Ok(_) => println!("cleared"),
        Err(error) => {println!("{error}")}
    }
    */
    println!("A cli tool for {} hacking", style("kernel").cyan());
    thread::sleep(Duration::from_millis(1000));
    let items = vec!["Vulnerability Analyser", "CVE Research", "Kernel Compile"];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();
    println!("You chose: {}", items[selection]);

    match selection {
        1 => {
            println!("Please enter the cve number!");
            let mut cve_number = String::new();
            io::stdin()
                .read_line(&mut cve_number)
                .expect("Failed to read");
        }
        _=>{println!("try again")}
    }

    
    // Execute `ls` in the current directory of the program.
    /*
    let mut list_dir = Command::new("ls");
    list_dir.status().expect("process failed to execute");
    */

    /*
 Command::new("apt")
        .arg("install")
        .spawn()
        .expect("sh command failed to start");   // Execute `ls` in the current directory of the program.
*/
}
