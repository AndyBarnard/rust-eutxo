use crate::utxo;

//addr currently contains its actual addr, e.g. 0xals;dkfja... and all the utxos sitting
//at that addr

//a utxo contains the pkh...in the utxo it's either pkh or a validator that locks
//the utxo.

//or is it possible there's an address that has nothing?

// struct Address {
//     addr: String,       //0x12345
//     utxos: Option<Vec<Utxo>>
// }

//TODO: how to model it? we have a bunch of addresses and each one has Option<Vec<Utxo>>?
//or each utxo has the address as a field on it? #2 seems better I think