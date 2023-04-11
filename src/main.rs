use ::hammster::hammster::{accumulate_inputs, create_circuit, draw_circuit, generate_setup_params, generate_keys, generate_proof, verify, run_mock_prover};

fn main() {
    let k = 6;
    let a_vec = vec![1, 1, 0, 1, 0, 1, 0, 0];
    let b_vec = vec![0, 1, 0, 0, 0, 1, 1, 0];
    let pub_input = accumulate_inputs(a_vec.clone(), b_vec.clone());

    let circuit = create_circuit(a_vec, b_vec);

    // Debug items
    draw_circuit(k, &circuit);
    run_mock_prover(k, &circuit, &pub_input);

    // Run the real prover and verifier
    let params = generate_setup_params(k);
    let (pk, vk) = generate_keys(&params, &circuit);
    let proof = generate_proof(&params, &pk, circuit, &pub_input);
    let verify = verify(&params, &vk, &pub_input, proof);
    println!("Verify result: {:?}", verify);

}