mod principal;
mod tx_id;
pub use principal::*;
pub use tx_id::*;

pub fn is_support_interface(did: String, interface_sig: String) -> bool {
    // remove whitespace from interface_sig
    let interface_sig = interface_sig.replace(" ", "");
    // remove whitespace from did
    let did = did.replace(" ", "");
    // check if the interface_sig is contained  by did
    did.contains(&interface_sig)
}

#[cfg(test)]
#[test]
//test is_support_interface
fn test_is_support_interface() {
    let did = r#"type CallData = record { method : text; args : vec nat8 };
    type Fee = record { rate : nat; minimum : nat };
    type Metadata = record {
      fee : Fee;
      decimals : nat8;
      name : text;
      totalSupply : nat;
      symbol : text;
    };
    type Result = variant { Ok : TransactionResponse; Err : text };
    type Result_1 = variant { Ok : vec TxRecord; Err : text };
    type Result_2 = variant { Ok : bool; Err : text };
    type TokenHolder = variant { None; Account : text; Principal : principal };
    type TokenInfo = record {
      owner : principal;
      allowanceSize : nat;
      cycles : nat64;
      txCount : nat;
      holders : nat;
      storages : vec principal;
      feeTo : TokenHolder;
    };
    type TransactionResponse = record { tx_id : text; error : opt vec text };
    type TxRecord = variant {
      Approve : record {
        nat;
        principal;
        TokenHolder;
        TokenHolder;
        nat;
        nat;
        nat64;
      };
      Transfer : record {
        nat;
        principal;
        TokenHolder;
        TokenHolder;
        nat;
        nat;
        nat64;
      };
    };
    type TxRecordResult = variant {
      Ok : TxRecord;
      Err : text;
      Forward : principal;
    };
    service : {
      allowance : (text, text) -> (nat) query;
      allowancesOf : (text) -> (vec record { TokenHolder; nat }) query;
      approve : (opt vec nat8, text, nat, opt CallData) -> (Result);
      balanceOf : (text) -> (nat) query;
      decimals : () -> (nat8) query;
      desc : () -> (vec record { text; text }) query;
      fee : () -> (Fee) query;
      lastTransactions : (nat64) -> (Result_1) query;
      logo : () -> (vec nat8) query;
      meta : () -> (Metadata) query;
      name : () -> (text) query;
      owner : () -> (principal);
      setDesc : (vec record { text; text }) -> (Result_2);
      setFee : (Fee) -> (Result_2);
      setFeeTo : (text) -> (Result_2);
      setLogo : (vec nat8) -> (Result_2);
      setOwner : (principal) -> (Result_2);
      symbol : () -> (text) query;
      tokenInfo : () -> (TokenInfo) query;
      totalSupply : () -> (nat) query;
      transactionById : (text) -> (TxRecordResult) query;
      transactionByIndex : (nat) -> (TxRecordResult) query;
      transfer : (opt vec nat8, text, nat, opt CallData) -> (Result);
      transferFrom : (opt vec nat8, text, text, nat) -> (Result);
    }"#;
    let interface_sig = "balanceOf : (text) -> ( nat )   query;";
    assert_eq!(
        is_support_interface(did.to_string(), interface_sig.to_string()),
        true
    );
}
