use halo2_proofs::dev::MockProver;
use ::hammster::hammster::{accumulate_inputs, create_circuit, draw_circuit};

mod hammster;

fn main() {
    let k = 6;
    let a_vec = vec![1,1,0,1,0,1,0,0];
    let b_vec = vec![0,1,0,0,0,1,1,0];
    let pub_input = accumulate_inputs(a_vec.clone(), b_vec.clone());

    let circuit = create_circuit(a_vec, b_vec);

    draw_circuit(k, &circuit);

    let prover = MockProver::run(k, &circuit, vec![pub_input]).unwrap();
    let res = prover.verify();
    match res {
        Ok(()) => println!("MockProver OK"),
        Err(e) => println!("err {:#?}", e),
    }
}
