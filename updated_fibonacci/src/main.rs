use std::io;

fn main() {
    let mut j=0;
    let mut k=1;
    let mut x;

    println!("Enter The Number of Iterations You Want To Run");

    loop{
        let mut repetitions = String::new();

        io::stdin()
            .read_line(&mut repetitions)
            .expect("Enter the number of repetitions");

        let repetitions: u32= match repetitions.trim().parse(){
            Ok(num)=> num,
            Err(_)=> continue,
        };
    
        for _i in 0..=repetitions{
            x=j+k;
            j=k;
            k=x;
            println!("{}", j);
        }
        break;
    }
}
