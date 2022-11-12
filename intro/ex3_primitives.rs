// The principal data types in rust are called primitives
// Variables can be or not type anotated

fn main() {
    // Variables can be type annotated.
    let _logical: bool = true;

    // And there are two types of anoation traditional and suffix, respectively
    let _unigned_int1: u8 = 12;
    let _unigned_int2 = 345u128;

    let _signed_int1: i8 = -100;
    let _signed_int2: i128 = -3245632;

    // Types also can be inferred
    let _inferred_type = -420;

    // Of course, variables always are constant, unless u use mut
    let mut _modifiable_type = 32i16;
    _modifiable_type = 420;


    println!("[*] Modification: {0:?}", _modifiable_type);
}