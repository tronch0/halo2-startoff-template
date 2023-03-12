use halo2_proofs::dev::MockProver;
use halo2_proofs::pasta::Fp;
use halo2_vitalik::my_circuit::MyCircuit;

fn main() {

    let x = Fp::from(3);
    let constant = Fp::from(5);
    let res = Fp::from(35);

    let circuit = MyCircuit::new(constant, Some(x));

    let public_inputs = vec![res];

    // MOCK PROOVER
    let prover = MockProver::run(4, &circuit, vec![public_inputs]).unwrap();
    assert_eq!(prover.verify(), Ok(()));

    // PROOF/VERIFICATION
    // let params: Params<EqAffine> = Params::new(4);
    // let vk = keygen_vk(&params, &circuit).unwrap();
    // let pk = keygen_pk(&params, vk, &circuit).unwrap();
    //
    // let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    //
    // let proof = create_proof(
    //     &params,
    //     &pk,
    //     &[circuit.clone()],
    //     &[&[&[res]]],
    //     OsRng,
    //     &mut transcript,
    // );


    // PLOTTING SECTION
    // use plotters::prelude::*;
    // let root = BitMapBackend::new("layout.png", (1024, 768)).into_drawing_area();
    // root.fill(&WHITE).unwrap();
    // let root = root
    //     .titled("Example Circuit Layout", ("sans-serif", 60))
    //     .unwrap();

    // halo2_proofs::dev::CircuitLayout::default()
    //     // You can optionally render only a section of the circuit.
    //     .view_width(0..7)
    //     .view_height(0..60)
    //     // You can hide labels, which can be useful with smaller areas.
    //     .show_labels(true)
    //     // Render the circuit onto your area!
    //     // The first argument is the size parameter for the circuit.
    //     .render(5, &circuit, &root)
    //     .unwrap();

    // let dot_string = halo2_proofs::dev::circuit_dot_graph(&circuit);

    // Now you can either handle it in Rust, or just
    // print it out to use with command-line tools.
    // print!("{}", dot_string);
}