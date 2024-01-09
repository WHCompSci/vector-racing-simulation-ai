use std::{f64::consts::FRAC_PI_2, fmt::Error, io::Empty};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct NeuralNet {
    layers_widths: Vec<usize>,
    weights_widths: Vec<usize>,
    layers: Vec<Vec<f64>>,
    weights: Vec<Vec<f64>>,
}

impl NeuralNet {
    fn new(layers_widths: Vec<usize>) -> NeuralNet {
        let layers: Vec<Vec<f64>> = layers_widths
            .iter()
            .map(|width| vec![0.0; *width])
            .collect();
        let mut weights_widths: Vec<usize> = vec![];
        for i in 0..layers_widths.len() - 1 {
            weights_widths.push(layers_widths[i] * layers_widths[i + 1])
        }
        let weights: Vec<Vec<f64>> = weights_widths
            .iter()
            .map(|width| vec![0.0; *width])
            .collect();
        NeuralNet {
            layers_widths: layers_widths,
            weights_widths,
            layers,
            weights,
        }
    }
    fn run(&mut self, layer_inputs: Vec<f64>) -> Result<&Vec<f64>, Error> {
        match layer_inputs {
            inputs if inputs.len() != self.layers_widths[0] => Err(Error),
            inputs => {
                self.layers[0] = inputs; // set the input layer
                for i in 1..self.layers.len() {
                    // for every layer after the input layer, we want to set all the nodes in the next node layer
                    // based on the weights and the nodes from the previous layer
                    for j in 0..self.layers_widths[i] {
                        for k in 0..self.layers_widths[i - 1] {
                            //add the last (node * weight)
                            let prev_node = self.layers[i - 1][k];
                            let weight = self.weights[i][j * self.layers_widths[i - 1] + k];
                            self.layers[i][j] += prev_node * weight;
                        }
                        //do activation function
                        self.layers[i][j] = relu(self.layers[i][j])
                    }
                }
                Ok(self.layers.last().unwrap())
            }
        }
    }
}

#[inline(always)]
fn relu(x: f64) -> f64 {
    x.max(0.0)
}
const RACER_FOV_RADIUS: f64 = 4.0;
const RACER_FOV_ANGLE: f64 = FRAC_PI_2;
#[derive(Debug, Clone)]
struct Racer {
    brain: NeuralNet,
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,
    score: Option<f64>,
}

impl Racer {
    fn new() -> Self {
        let num_input_nodes = ( 0.5 * RACER_FOV_RADIUS * RACER_FOV_RADIUS * FRAC_PI_2) as usize;
        Self {
            brain: NeuralNet::new(vec![num_input_nodes, 15_usize, 15_usize, 9_usize]),
            pos_x: 0,
            pos_y: 0,
            vel_x: 0,
            vel_y: 0,
            score: None,
        }
    }
}
// fn reward_function() {
//     todo!()
// }
#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty {
        is_start: bool,
        is_finish: bool,
    },
    Wall,

}

struct Map {
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(height: usize, width: usize) -> Map { Map { height, width, tiles } }
}

fn train_networks(racers_per_map: usize, racers_per_generation: usize, num_generations: usize) {
    assert!(racers_per_generation % racers_per_map == 0 && racers_per_map < racers_per_generation, "The racers_per_generation should be divisible by racers_per_map");
    let num_maps_per_generation = racers_per_generation / racers_per_map;
    for generation in 0..num_generations {
        let racers = vec![Racer::new()];
        for map_idx in 0..num_maps_per_generation {
            let curr_racers = &racers[map_idx*racers_per_map..(map_idx + 1)*racers_per_map];
            let curr_map = Map {
                height: 100,
                width: 100,
            };
        }
        
    }
}
