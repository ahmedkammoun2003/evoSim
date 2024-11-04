mod animal;
mod food;
mod world;

pub use self::animal::*;
pub use self::food::*;
pub use self::world::*;
use sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let config = sim::Config::default();
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&sim::Config::default())
    }

    pub fn config(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self.sim.config())
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) -> Option<String> {
        self.sim.step(&mut self.rng).map(|stats| stats.to_string())
    }

    pub fn train(&mut self) -> String {
        self.sim.train(&mut self.rng).to_string()
    }
}
