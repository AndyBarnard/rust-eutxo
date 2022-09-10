use anyhow::Error;
use std::collections::HashMap;

use crate::init_blockchain;
use crate::utxo;
use crate::validator;

//todo: how to make a type Value which is a Hashmap of a Hashmap (i.e. not a single value)

//TxInfo: a pending transaction. Is passed to a validator fxn as "tx context"

// struct TxInfo {
//     inputs: [TxInInfo],
//     outputs: [TxOut],
//     redeemers: HashMap<ScriptPurpose, Redeemer>,
//     datums: HashMap<DatumHash, Datum>,
//     fee: u32,
//     mint: Value,
//     txHash: TxHash,
// }

// type Value = HashMap<HashMap<String, String>>;

// struct Pending_Tx {
//     inputs: Vec<TxInInfo>,
//     outputs: Vec<TxOut>,
//     fee: Value,
//     mint: Value,
//     valid_range: u32,
//     data: (DatumHash, Datum),
//     txh: TxHash
// }


struct TxOut {
    address: String,
    value: Value,
    datum: Option<DatumHash>,
}

type TxHash = String;

struct Transaction {
    inputs: Vec<Utxo>,
    outputs: Vec<TxOut>,
    redeemers: Vec<UtxoKey>,
    data: HashMap<DatumHash, Datum>,
    mint: u32, // if positive, then minting tokens. if negative, then burning tokens
    fee: u32,
}

impl Transaction {

//In plutus, Tx type has [TxOut] which contains 3 fields: Address, Value, and Maybe DatumHash.
// maybe I can just pass that to build_tx then create the remaining fields to create the output utxos?

    pub fn build_tx(
        inputs: Vec<UtxoRef>,  //can be referred to with tuple (txh, idx)
        outputs: Vec<UtxoInfo>, //does this need to be passed as an arg? couldn't it just be done in the fxn body?
        redeemer: Vec<UtxoKey>,  //one redeemer for each input utxo
        data: HashMap<DatumHash, Datum>,
        mint: Option<u32>,
        fee: Option<u32>,
    ) -> Transaction {
        //calculate txHash and build Utxos for the outputs of this tx
        //using the input UtxoRefs to ensure every tx has a unique txhash should suffice (b/c you can't duplicate a utxo)



        Transaction {
            inputs,
            outputs,
            redeemer,
            data,
            mint,
            fee,
        }
    }

//Utxo -> UtxoInfo -> UtxoLock -> Validator(T)
//&input_utxo -> utxo_info -> utxo_lock -> Validator(validate(&self))

    //TODO: when executing tx, the validator/ others are sitting at the UTXO already...
    //so just pass the Transaction and run the corresponding validator?

    //If this was haskell I think the return type would be: IO [Utxo] b/c we're changing the state of the network
    fn execute_tx(&self) -> anyhow::Result<Vec<Utxo>> {
        if can_spend_input_utxos(&self) {
            
        }

        Error("couldnt execute tx")
    }

    fn can_spend_input_utxos(&self) -> bool {
        for &input_utxo in self.inputs {
            if &input_utxo.utxo_info.utxo_lock.Validator(validate(&self)) {
                continue;
            } else {
                false
            }

            true
        }
    }

    fn build_and_execute_test_tx() {
        let test_tx = build_tx(
            vec![build_genesis_utxo()],
            vec![TxOut {
                address: "0x",
                value: 100,
                datum: None,
            }],
        )?;

        execute_tx(&test_tx);
    }
}

// struct TxOut {
//     address: String,
//     value: Value,
//     datum: Option<DatumHash>,
// }


///----------------------------------------------------------///
///                  Example Wallet Tx                       ///
///----------------------------------------------------------///


// struct Wallet {}

// impl Wallet {
//     //to build a tx, first I need to build the Utxos for output (input utxos will already exist)
//     fn sample_wallet_tx() {
//         let inputs = vec![TxHash::from("sample tx hash")];  //won't be TxHash literal, but will be pulled
//         //from some data structure holding all the TxHashes... a HashMap? Set?
//         let idxs = vec![0];
//         let values = vec![0];
//         let datums = vec![Datum];
//         let validator = Sample_Validator::new();

//         let utxos = build_utxos(
//             inputs,
//             idxs,
//             values,
//             datums,
//             validator
//         );
//     }
// }

// fn build_utxo(
//     txh: TxHash,        //the txh from the tx that produced this Utxo (different for input/output utxos)
//     idx: u32,           //which of the (>= 1) utxos is this from the output tx?
//     value: u32,         //value locked at this utxo
//     datum: &Datum,      //Datum or DatumHash? TODO
//     validator: &impl Validates, //any type with an appropriate validate() fxn. constructed in user's wallet
// ) -> Self {

struct Sample_Validator;

impl Sample_Validator {
    fn new() -> Self {
        fn validate(&tx: Transaction) -> bool {
            //when we validate, we want the tx "context"... in plutus there's a distinction between pending/final Tx
            // (with TxInfo vs Tx types) but for now we'll treat all pending Txs the same 
            true;
        }
    }
}

    //TODO: since a Transaction already has a Vec<Utxo> for the inputs, the validation fxn should only need
    //to take a Transaction as a single param? or not...may be a different way to structure it?

    //fxn needs tx context, redeemer, datum, value
    //(txcontext, redeemer) from tx and (datum, value) from utxo (but the Transaction contains all input utxos anyway...)
    //can do smth with self.value and self.datum
    //can extract the redeemer from the transaction