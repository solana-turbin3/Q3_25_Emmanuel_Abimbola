#[test]
fn base58_to_wallet() {
    println!("Enter your name");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    //  5kczzVBjNMnzM3TKsR5WXF4B4zjQ9SQSxxS6pvGzsX4q

    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("Decoded wallet: {:?}", wallet);

}

#[test]
fn wallet_to_base58() {
    let wallet = vec![34, 46, 55, 124, 141, 19];
    let base58_encoded = bs58::encode(wallet).into_string();
    println!("Encoded base58: {}", base58_encoded);
}