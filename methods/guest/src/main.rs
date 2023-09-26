#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]
// std support is experimental
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here
    const MODULUS: u32 = (1 << 31) - 1;
    let a: Vec<u32> = env::read();
    let s: Vec<u32> = env::read();
    let e: i32 = env::read();

    let start = env::get_cycle_count();
    assert!(s.iter().all(|&x| x < 3), "wrong bound");
    assert!(-5 <= e && e <= 5);

    let y = a
        .iter()
        .zip(s.iter())
        .fold(0, |acc, (&x, &y)| acc + (x * y) % MODULUS);

    let y = (y + e as u32) % MODULUS;

    let end = env::get_cycle_count();

    println!("cycles: {}", end - start);
    //env::commit(&a);
    env::commit(&y);
    //let product = a.checked_mul(b).expect("Integer overflow");
    //env::commit(&product);
}
