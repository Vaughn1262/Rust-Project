use std::io;


fn main(){
    //sets initial run condition
    let mut running = 1;

    //Makes 2 vectors to store passwords and sites
    let mut namevec = vec![];
    let mut passvec = vec![];

    //runs as long as the program isn't exited
    while running != 0 {

        //displays the Menu anytime that the program isn't ended. 
        println!("1. Add a password: ");
        println!("2. Remove a password: ");
        println!("3. View a password: ");
        println!("4. View how many passwords are stored: ");
        println!("5. Quit the program: ");
        
        //Gets user input
        let mut optinput = String::new();
        println!("Please select an option(1-5). ");
        io::stdin().read_line(&mut optinput).expect("failed to read line");

        //Trims off any newline or space characters at the end.
        let optinput = optinput.trim_end();
        
        //Adds a password and a website to associate with that password
        if optinput == "1"{
            println!("Please give the website name: ");
            let mut webinput = String::new();
            io::stdin().read_line(&mut webinput).expect("failed to read line");
            namevec.push(webinput);
            println!("Please give a password for the site ");
            let mut passinput = String::new();
            io::stdin().read_line(&mut passinput).expect("failed to read line");
            passvec.push(passinput);
        
        //Allows the user to remove a certain websites password. Also removes the website.
        } else if optinput == "2"{
            println!("What websites password would you like to remove? ");
            let mut reminput = String::new();
            io::stdin().read_line(&mut reminput).expect("failed to read line");
            let remi = namevec.iter().position(|s| s == &reminput).unwrap();
            namevec.remove(remi);
            passvec.remove(remi);
        
        //Allows the user to view a password for a specific website.
        } else if optinput == "3"{
            println!("What websites password would you like to view? ");
            let mut viewinput = String::new();
            io::stdin().read_line(&mut viewinput).expect("failed to read line");
            let viewi = namevec.iter().position(|s| s == &viewinput).unwrap();
            let viewpass = &passvec[viewi];
            print!("{}\n",viewpass);

        //Counts how many passwords the user has stored.  
        } else if optinput == "4"{
            let mut count = 0;
            for i in &namevec{
                count += 1;
            }

        // Prints how many passwords the user has stored.
            println!("{}", count);

        //Quits the program if the user doesn't choose 1-4   
        } else {
            println!("you quit");
            running = 0;
        }

    }
}