use std::marker::PhantomData;
use halo2_proofs::circuit::{AssignedCell, Chip, Layouter, SimpleFloorPlanner};
use halo2_proofs::dev::MockProver;
use halo2_proofs::pasta::Fp;
use halo2_proofs::plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector};
// x^3 + x + 5 = 35

// ANCHOR: instructions
// Instructions are the boundary between high-level gadgets and the low-level circuit operations.
trait Ops {
    type Num;

    fn load_private(&self, layouter: impl Layouter<Fp>, x: Option<Fp>) -> Result<Self::Num, Error>;
    fn load_constant(&self, layouter: impl Layouter<Fp>, x: Fp) -> Result<Self::Num, Error>;

    fn mul(&self, layouter: impl Layouter<Fp>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;
    fn add(&self, layouter: impl Layouter<Fp>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;
    // fn expose_public(&self, layouter: impl Layouter<Fp>, num: Self::Num, row: usize) -> Result<(), Error>;
}

impl Ops for MyChip {
    type Num = AssignedCell<Fp, Fp>;

    fn load_private(&self, mut layouter: impl Layouter<Fp>, v: Option<Fp>) -> Result<Self::Num, Error> {
        let config = self.config();
        layouter.assign_region(|| "load private", |mut region | {
            region.assign_advuce(|| "private value", config.advice[0], 0 || v.ok_or(Error:syn) )
        });
    }

    fn load_constant(&self, layouter: impl Layouter<Fp>, x: Fp) -> Result<Self::Num, Error> {
        todo!()
    }

    fn mul(&self, layouter: impl Layouter<Fp>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        todo!()
    }

    fn add(&self, layouter: impl Layouter<Fp>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        todo!()
    }
}

struct MyChip{
    config: MyConfig,
    // _marker: PhantomData<Fp>,
}

impl Chip<Fp> for MyChip {
    type Config = MyConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config;
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

impl MyChip {
    fn new(config: MyConfig) -> Self {
        MyChip {
            config,
        }
    }
}





#[derive(Clone,Debug)]
struct MyConfig {
    advice: [Column<Advice>; 2],

    instance: Column<Instance>,

    s_mul: Selector,
}

#[derive(Default)]
struct MyCircuit {
    constant: Fp,
    x: Option<Fp>,
}

impl Circuit<Fp> for MyCircuit {
    type Config = MyConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        todo!()
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<Fp>) -> Result<(), Error> {
        let chip = MyChip::new(config);

        // let x = chip.load_private();
        todo!()
    }
}


fn main() {
    let x = Fp::from(3);
    let constant = Fp::from(5);
    let res = Fp::from(35);

    let circuit = MyCircuit {
        constant,
        x: Some(x),
    };

    let public_inputs = vec![res];

    let prover = MockProver::run(4, &circuit, vec![public_inputs]).unwrap();

    assert_eq!(prover.verify(), Ok(()));

    println!("Hello, world!");
}
