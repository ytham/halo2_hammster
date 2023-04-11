use std::time::Instant;

use ::hammster::hammster::{accumulate_inputs, create_circuit, draw_circuit};
use halo2_proofs::{
    dev::MockProver,
    pasta::EqAffine,
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use rand_core::OsRng;

mod hammster;

fn main() {
    let k = 6;
    let a_vec = vec![1, 1, 0, 1, 0, 1, 0, 0];
    let b_vec = vec![0, 1, 0, 0, 0, 1, 1, 0];
    let pub_input = accumulate_inputs(a_vec.clone(), b_vec.clone());

    let circuit = create_circuit(a_vec, b_vec);

    draw_circuit(k, &circuit);

    let prover = MockProver::run(k, &circuit, vec![pub_input.clone()]).unwrap();
    let res = prover.verify();
    match res {
        Ok(()) => println!("MockProver OK"),
        Err(e) => println!("err {:#?}", e),
    }

    // we generate a universal trusted setup of our own for testing
    let params = Params::<EqAffine>::new(k);

    // just to emphasize that for vk, pk we don't need to know the value of `x`
    let dummy_circuit = create_circuit(vec![0; 8], vec![0; 8]);
    let vk = keygen_vk(&params, &dummy_circuit).expect("vk should not fail");
    let pk = keygen_pk(&params, vk, &dummy_circuit).expect("pk should not fail");

    let start = Instant::now();
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[&pub_input]],
        OsRng,
        &mut transcript,
    )
    .expect("prover should not fail");
    let proof = transcript.finalize();
    println!("Proof time: {:?}", start.elapsed());

    let start = Instant::now();
    // verify the proof to make sure everything is ok
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let strategy = SingleVerifier::new(&params);
    verify_proof(
        &params,
        pk.get_vk(),
        strategy,
        &[&[&pub_input]],
        &mut transcript,
    )
    .unwrap();
    println!("Verify time: {:?}", start.elapsed());
}
