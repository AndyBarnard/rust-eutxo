use std::process;
use std::env;

use lib;

fn main() {
    println!("Welcome to this eutxo blockchain");
    
    lib::init();
}


//TODO in main: make CLI tool that allows users to build and submit a transaction. 
// input utxos will be represented by a tuple: txh and idx
//this will work like:
// build-tx --txhs 0x123 0x456 0x789 --idxs 0 4 2 --redeemer 69 --data "asdf"
