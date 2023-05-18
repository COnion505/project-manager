use common::{PMError, PMSetting, ProjectManager};


fn main(){
    let s = input("project name");
    println!("input value: {s}");
}

pub fn input(message: &str)-> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{message}: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("failed input.");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

