use 

fn main() {
    println!("Hello, world!");
}

enum Coin{
    Bob,
    Kobole,
    Das,
    Mbao(EraOfProd),
}

fn shilling_value(coin: Coin) -> u8{
    match coin{
        Coin::Bob=>{ 
            println!("Saka ganji mbwa");
            1},
        Coin::Kobole=>5,
        Coin::Das=>10,
        Coin::Mbao(EraOfProd)=>{
            println!("Was produced from {:?}", EraOfProd);
            20
        },
    }
}

#[derive(Debug)]
enum EraOfProd{
    Kenyatta,
    Moi,
    Kibaki,
    Jayden
}

    