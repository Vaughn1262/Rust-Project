use std::io;


fn main(){
    let mut running = 1;
    let mut namevec = vec![];
    let mut passvec = vec![];

    while running != 0 {
        println!("1. Add a password: ");
        println!("2. Remove a password: ");
        println!("3. View a password: ");
        println!("4. View how many passwords are stored: ");
        println!("5. Quit the program: ");
        let mut optinput = String::new();
        println!("Please select an option(1-5). ");
        io::stdin().read_line(&mut optinput).expect("failed to read line");
        let optinput = optinput.trim_end();

        if optinput == "1"{
            println!("Please give the website name: ");
            let mut webinput = String::new();
            io::stdin().read_line(&mut webinput).expect("failed to read line");
            namevec.push(webinput);
            println!("Please give a password for the site ");
            let mut passinput = String::new();
            io::stdin().read_line(&mut passinput).expect("failed to read line");
            passvec.push(passinput);

        } else if optinput == "2"{
            println!("What websites password would you like to remove? ");
            let mut reminput = String::new();
            io::stdin().read_line(&mut reminput).expect("failed to read line");
            let remi = namevec.iter().position(|s| s == &reminput).unwrap();
            namevec.remove(remi);
            passvec.remove(remi);

        } else if optinput == "3"{
            println!("What websites password would you like to view? ");
            let mut viewinput = String::new();
            io::stdin().read_line(&mut viewinput).expect("failed to read line");
            let viewi = namevec.iter().position(|s| s == &viewinput).unwrap();
            let viewpass = &passvec[viewi];
            print!("{}\n",viewpass);
            
        } else if optinput == "4"{
            let mut count = 0;
            for i in &namevec{
                count += 1;
            }
            println!("{}", count);
            
        } else {
            println!("you quit");
            running = 0;
        }

    }
}