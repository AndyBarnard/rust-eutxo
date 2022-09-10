//TODO: Datum should actually be DatumHash, a hash of the Datum using borsh

//TODO: all utxos sit at an address. So we should have a Vec<Address>


use crate::address;

struct Utxo {
    utxo_ref: UtxoRef,
    utxo_info: UtxoInfo,
}

struct UtxoRef {
    txh: TxHash, //so txhash is the way we're actually indexing utxos in terms of the whole chain
    idx: u32,    //index is 0 based, all the utxos output from above txh
}

impl UtxoRef {
    fn new(txh: &TxHash, idx: u32) -> Self {
        Self {
            txh,
            idx
        }
    }
}

struct UtxoInfo {
    value: u32,
    datum: Datum,
    utxo_lock: UtxoLock, 
    address: Address
}

//this utxo is either locked by a pkh (need private key to consume) or a validator (smart contract)

enum UtxoLock<T: Validates> {
    Validator(T),
    PubKeyHash
}

enum UtxoKey<T> {
    Redeemer(T),
    PrivateKey
}

type Datum = String;

type DatumHash = String;

type Address = String;

//TODO: a tx has a list of input utxos, ie. utxos that already exist. maybe in network.rs
// I can use some data structure to represent all utxos in the whole network
// like they're indexed in some way, some identifier (maybe txinref / txoutref)

//for the output of a tx, the Transaction type has only wellformed output utxos
//but we wait to execute the tx for those to actually get deployed to the network

//TODO: do I even need a separate Validator type? can't I just impl Validates for Utxo?
//the reason I cant do that is that then Utxo would always have the same validator implementation

trait Validates {
    fn validate(&tx: Transaction) -> bool;
    //fxn needs tx context, redeemer, datum, value
    //(txcontext, redeemer) from tx and (datum, value) from utxo (but the Transaction contains all input utxos anyway...)
    //can do smth with self.value and self.datum
    //can extract the redeemer from the transaction
}

impl Utxo {
    //TODO: should build_utxos be in the impl block for Utxo? does that make sense? shouldnt it be some separate utility fxn?

    //TODO: get data in the right shape to create UTXOs

    //TODO: maybe do the UTXO indexing in build_utxos? keep a counter? seems messy
    fn build_utxos(
        txh: TxHash, //must come from the tx, indexing can be done in build_utxos. dont pass as arg
        utxo_infos: Vec<UtxoInfo>,
    ) -> Vec<Utxo> {
        let mut set_of_utxos = Vec::new();

        for &utxo_info in utxo_infos {
            let built_utxo = build_utxo(&txh, &utxoInfo, set_of_utxos.len); //using length of vec as idx...gotta be a better way
            &set_of_utxos.push(built_utxo);
        }
    }

    //TODO: build_utxo (probably) works mostly the same for input / output UTXOs, but for txh,
    //for input utxos, the txh came from their producing tx, whereas for output utxos,
    //the txh comes from this tx. so maybe do pattern matching on txh field of build_utxo

    //build_utxo should only be called inside a Transaction. otherwise there shouldn't be a way to make utxos
    //other than the genesis Tx / utxo

    fn build_utxo(
        txh: &TxHash, //the txh from the tx that produced this Utxo (different for input/output utxos)
        utxo_info: &UtxoInfo,
        idx: u32,
    ) -> Self {
        let utxo_ref = UtxoRef::new(&txh, idx);

        Self {
            utxo_ref,
            utxo_info
        }
    }

    fn build_genesis_utxo() -> Utxo {
        Utxo {
            txh: "genesistxhash",
            idx: 0,
            value: 1_000_000,
            datum: "genesis text",
            validator: genesis_validator(),
        }
    }
}

fn genesis_validator() -> Validator {
    Validator { code: "testcode" }
}

//example redeemer: its an auction contract, and the bid is the redeemer. in the validator fxn
// we check that the bid is within the appropriate time slot else it's invalid

//TODO: when a Tx creates output utxos, it can create txh and idx by itself.
//then what you pass to Tx for outputs isn't Vec<Utxo>, but rather the following fields:
//value, datum, validator

//obviously every output utxo will get the same txhash, and we just loop through
//the number of output utxos incrementing by 1 for the idx.
//so the fxn build_utxos can just iterate over the number of outputs we pass to the Transaction::new()
//so what do we pass to Tx?... Vec<(value, datum, validator)> ...the size of this vec is
//the number of utxos we want to output

//for the CLI transaction creation/execution, just put it all in JSON form
