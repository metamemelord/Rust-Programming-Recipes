use failure::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let txs = errors::get_txs("test_data/transactions.json").expect("Could not load transactions");
    for t in txs {
        println!("{:?}", t);
    }
        match errors::get_first_tx("test_data/transactions.json", "Rave") {
          Ok(v) => println!("{:?}", v),
          Err(e) => println!("Error: {:?}, Backrtrace: {:?}", e, e.backtrace()),
        }
    Ok(())
}

