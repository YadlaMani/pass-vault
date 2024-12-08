mod passentry;
use passentry::{ServiceInfo, read_passwords_from_file, prompt};

fn clr() {
    print!("{}[2J", 27 as char);
    print!("\x1B[H"); // Move the cursor to the top left
}

fn main() {
    clr();
    let ascii = r#"
 __________                        ____   ____            .__   __   
 \______   \_____    ______ ______ \   \ /   /____   __ __|  |_/  |_ 
  |     ___/\__  \  /  ___//  ___/  \   Y   /\__  \ |  |  \  |\   __\
  |    |     / __ \_\___ \ \___ \    \     /  / __ \|  |  /  |_|  |  
  |____|    (____  /____  >____  >    \___/  (____  /____/|____/__|  
                \/     \/     \/                 \/                              
    "#;
    println!("{ascii}");

    loop {
        println!("Password manager menu:");
        println!("1. Add entry");
        println!("2. List entries");
        println!("3. Search entry");
        println!("4. Exit");
        
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service:"),
                    prompt("Username:"),
                    prompt("Password:"),
                );
                entry.write_to_file();
                println!("Entry added successfully!");
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Error reading passwords: {}", error);
                    Vec::new()
                });
                for service in &services {
                    println!(
                        "Service: {}\n- Username: {}\n- Password: {}",
                        service.service, service.username, service.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Error reading passwords: {}", error);
                    Vec::new()
                });
                let search = prompt("Search:");
                for service in &services {
                    if service.service.eq_ignore_ascii_case(search.as_str()) {
                        println!(
                            "Service: {}\n- Username: {}\n- Password: {}",
                            service.service, service.username, service.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
        println!("\n\n");
    }
}
