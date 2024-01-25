use std::io;

fn main() {
    loop{
        println!("Choose a number");
        println!("1. Sum\n 2. Difference\n 3. Product\n 4. Division\n");
        let mut entry= String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read input");
    
        let entry:u32 = match entry.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut w= String::new();
        io::stdin()
            .read_line(&mut w)
            .expect("Input a value");

        let w: u32= match w.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut b= String::new();
        io::stdin()
            .read_line(&mut b)
            .expect("Input a value");

        let b: u32= match b.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if entry ==1 {
            sum(w, b);
        }
        else if entry == 2{
            difference(w, b);
        }
        else if entry==3{
            product(w, b);
        }
        else if entry ==4{
            chief_quotient(w, b);
        }
        else if entry == 5{
            remainder(w, b);
        }
        else {
            println!("You have not entered a relevant choice");
        };
    }
}
fn sum(w: u32,b: u32){
    println!("sum ={}",w+b);
}
fn difference(w: u32,b:u32){
    if w<b{
        println!("Difference= -{}",b-w);
    }else{
    println!("Difference= {}",w-b);
    }
}
fn product(w: u32,b: u32){
    println!("Product= {}",w*b);
}
fn chief_quotient(w: u32,b: u32){
    println!("Quotient= {}",w/b); 
}
fn remainder(w: u32,b: u32){
    println!("Remainder= {}",w%b);
}
