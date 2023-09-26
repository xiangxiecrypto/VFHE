// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use risc0_zkvm::{
    default_prover,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use vfhe_methods::{VFHE_ELF, VFHE_ID};

fn main() {
    let a = vec![1u32; 1024];
    let s = vec![1u32; 1024];
    let e: i32 = -4;
    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&a).unwrap())
        .add_input(&to_vec(&s).unwrap())
        .add_input(&to_vec(&e).unwrap())
        .build()
        .unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Obtain the default prover.
    let start = std::time::Instant::now();
    let prover = default_prover();
    let duration = start.elapsed();
    println!("default time: {} milliseconds", duration.as_millis());

    // Produce a receipt by proving the specified ELF binary.
    let start = std::time::Instant::now();
    let receipt = prover.prove_elf(env, VFHE_ELF).unwrap();
    let duration = start.elapsed();
    println!("Prove time: {} milliseconds", duration.as_millis());
    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    let start = std::time::Instant::now();
    receipt.verify(VFHE_ID).unwrap();
    let duration = start.elapsed();
    println!("Verify time: {} milliseconds", duration.as_millis());

    //let _aprime: Vec<u32> = from_slice(&receipt.journal).unwrap();
    let c: u32 = from_slice(&receipt.journal).unwrap();

    //println!("aprime: {:?}", aprime);
    println!("I know the factors of {}", c);
}
