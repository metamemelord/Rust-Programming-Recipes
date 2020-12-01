use serde_derive::{Serialize, Deserialize};
pub mod error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_first_tx(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let txs = get_txs(fname)?;
    for t in txs {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err(error::TransactionError::Error("Could not find a Transaction").into())
}

pub fn _get_txs_long(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}

pub fn get_txs(fname: &str) -> Result<Vec<Transaction>, error::TransactionError> {
    /*
     // This code works
     Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )
    */

    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}
