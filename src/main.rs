extern crate keys;

use keys::generator::Generator;
use keys::generator::Random;
use keys::Network::Mainnet;


fn main() {
    let rand = Random::new(Mainnet);
    match rand.generate() {
        Ok(res) => println!("{}", res),
        Err(_) => println!("do nothing"),
    };
    
}
