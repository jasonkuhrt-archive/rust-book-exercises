use std::collections::HashMap;

fn main() {
    let mut org: HashMap<String, Vec<String>> = HashMap::new();
    let mut members: HashMap<String, String> = HashMap::new();
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let commands: Vec<&str> = input.split_whitespace().collect();
        if commands[0] == "add" {
            let member_name = commands[2];
            let department_name = commands[4];
            let department = org.entry(department_name.to_owned()).or_insert(Vec::new());
            department.push(member_name.to_string());
            members.insert(member_name.to_string(), department_name.to_owned());
        } else if commands[0] == "get" && commands[1] == "department" {
            let department_name = commands[2];
            let department = org.get(department_name);
            println!("members in {:?}: {:?}", department_name, department);
        } else if commands[0] == "get" && commands[1] == "member" {
            let member_name = commands[2];
            let department_name = members.get(member_name);
            println!("{:?} is in department {:?}", member_name, department_name);
        } else {
            continue;
        }
    }
}
