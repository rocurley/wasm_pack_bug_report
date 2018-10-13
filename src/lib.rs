extern crate cfg_if;
extern crate wasm_bindgen;
extern crate osqp;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use osqp::{Settings, Problem};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Define problem data
    let P = &[[4.0, 1.0],
              [1.0, 2.0]];
    let q = &[1.0, 1.0];
    let A = &[[1.0, 1.0],
              [1.0, 0.0],
              [0.0, 1.0]];
    let l = &[1.0, 0.0, 0.0];
    let u = &[1.0, 0.7, 0.7];

    // Change the default alpha and disable verbose output
    let settings = Settings::default()
        .alpha(1.0)
        .verbose(false);

    // Create an OSQP problem
    let mut prob = Problem::new(P, q, A, l, u, &settings);

    // Solve problem
    let result = prob.solve();

    // Print the solution
    let solution = format!("{:?}", result.x().expect("failed to solve problem"));
    alert(&solution);
}
