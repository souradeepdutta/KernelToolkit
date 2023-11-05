#![allow(dead_code)]
#![allow(unused_variables)]

//use std::thread;
use std::io;
//use std::time::Duration;
use console::Term;
use console::style;
use dialoguer::Select;
use std::process::Command;

fn vulnerability_analyze() {
    println!("Getting current kernel versions");
    println!("Top {} for your kernel", style("3 Vulnerabilities").red());

     Command::new("./scrape2/main")
            .spawn()
            .expect("command failed to start");   // Execute `ls` in the current directory of the program.

}

fn cve_research() {
    println!("Please enter the cve number!");
    let mut cve_number = String::new();
    io::stdin()
        .read_line(&mut cve_number)
        .expect("Failed to read");
     cve_number.pop();
     Command::new("./scrape/scrape")
            .arg(cve_number)
            .spawn()
            .expect("command failed to start");   // Execute `ls` in the current directory of the program.
}

fn kernel_compile() {
    let items = vec!["1) 6.6-rc7","2) 6.5.9","3) 6.1.60","4) 5.15.137","5) 5.10.199","6) 5.4.259","7) 4.19.297","8) 4.14.328"];

    let mut selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();
    println!("You chose: {}", items[selection]);
    selection = selection+1;

     Command::new("./compile.sh")
            .arg(selection.to_string())
            .spawn()
            .expect("compile command failed to start");   // Execute `ls` in the current directory of the program.
}

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
    // thread::sleep(Duration::from_millis(1000));
    let items = vec!["Vulnerability Analyser", "CVE Research", "Kernel Compile"];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();
    println!("You chose: {}", items[selection]);

    match selection {
        0 => {
            vulnerability_analyze();
        },
        1 => {
            cve_research();
        },
        2 => {
            kernel_compile();
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
