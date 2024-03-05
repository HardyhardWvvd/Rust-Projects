use std::io;

fn main(){
    loop{
        let mut sign = String::new();
        io::stdin()
            .read_line(&mut sign)
            .expect("Give the instruction"); 
        match sign {
            sign::Red { 
                println!("Halt");
                halt
            }
            sign::Amber{ 
                println("Get Ready"); 
                wait
            }
            sign::Green{ 
                println("Go"); 
                go
            }
            
        }
    }
    enum Lights{
        Red(String),
        Amber(String),
        Green(String)
    }

}