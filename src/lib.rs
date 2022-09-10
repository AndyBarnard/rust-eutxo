// // use crate::transaction;

// //the whole module tree should be in lib.rs, so we create a good API

// use std::collections::HashMap;

// // use crate::initblockchain;

// // struct ScriptContext {
// //     txInfo: TxInfo,
// //     purpose: ScriptPurpose,
// // }

// enum ScriptPurpose {
//     Minting(CurrencySymbol),
//     Spending(TxOutRef),
// }

// //newtype Value = Value { getValue :: Map.Map CurrencySymbol (Map.Map TokenName Integer) }
// //TODO: is a struct the best way to structure the Value type?
// struct Value {
//     value: HashMap<CurrencySymbol, HashMap<TokenName, u32>>,
// }

// type TxHash = String;

// type CurrencySymbol = String;

// type TokenName = String;

// struct Validator {}


pub fn init() -> bool {
    true
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
