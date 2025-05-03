use ndarray::{Array, Array2};
//use ndarray_rand::rand::distributions::weighted;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::fs::File;
use std::io::{self, BufRead};

fn csv_to_ndarray(path: &str) -> (Vec<Array2<f32>>, Vec<Array2<f32>>) {
    let file: File = File::open(path).expect("Failed to read the file.");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut arrs= Vec::new();
    let mut vals = Vec::new();


    for line in reader.lines() {

        let mut val: f32 = 0.0;

        let mut values: Vec<f32> = Vec::new();

        
        match line {
            Ok(text) => {
                let mut parts = text.split(',').map(|s| s.trim());
                if let Some(first) = parts.next() {
                    if let Ok(id) = first.parse::<f32>() {
                        let list: Vec<f32> = parts
                            .filter_map(|v| v.parse::<f32>().ok())
                            .collect();

                        val = id;
                        values = list;
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {}",e),
        }

        let mut ohe_val: Array2<f32> = Array::zeros((10,1));

        ohe_val[[val as usize, 0]] +=1.0;

        vals.push(ohe_val);

        let arr: Array2<f32> = Array2::from_shape_vec((28,28), values).unwrap();

        let min =  0.0;
        let max = 255.0; 

        let norm = arr.mapv(|x| (x - min) / (max - min));

        arrs.push(norm);
    }
    println!("Set Length: {:?}", vals.len());
    (arrs, vals)
}

struct NeuralNetwork {
    input_size: usize,
    layer1_size: usize,
    layer2_size: usize,
    output_size: usize,
    learning_rate: f32,
    weights_input_to_layer1: Array2<f32>,
    weights_layer1_to_layer2: Array2<f32>,
    weights_layer2_to_output: Array2<f32>,
}

impl NeuralNetwork {
    fn new(input_size: usize, layer1_size: usize, layer2_size:usize, output_size: usize, learning_rate: f32) -> Self {
        // Initialize the weights for the input and hidden layers randomly between 0 and 0.1.

        let weights_input_to_layer1 = Array::random((layer1_size,input_size), Uniform::new(-0.1,0.1)); //-0.28, 0.28
        let weights_layer1_to_layer2 = Array::random((layer2_size,layer1_size), Uniform::new(-0.1,0.1));
        let weights_layer2_to_output = Array::random((output_size,layer2_size), Uniform::new(-0.1,0.1));

        // Return a neural network that has the randomly initialized weights.
        NeuralNetwork {
            input_size,
            layer1_size,
            layer2_size,
            output_size,
            learning_rate,
            weights_input_to_layer1,
            weights_layer1_to_layer2,
            weights_layer2_to_output,
        }
    }

    // Forward propagation.  Returns all of the intermediate and final outputs.
    // Currently commented out to avoid compilations errors
    fn forward(&self, input: &Array2<f32>) -> (Array2<f32>, Array2<f32>, Array2<f32>) 
    {
        let flattened: Array2<f32> = Array::from_shape_vec((input.len(), 1), input.iter().cloned().collect()).unwrap();
        
        let layer1_output = (self.weights_input_to_layer1.dot(&flattened)).mapv(|x| x.max(0.0)); 

        let layer2_output  = (self.weights_layer1_to_layer2.dot(&layer1_output)).mapv(|x| x.max(0.0));

        let final_output = self.weights_layer2_to_output.dot(&layer2_output);
        
        //let max = final_output.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        //println!("Final pre softmax {:?}", final_output);

        let max = final_output.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        //println!("Pre-softmax: {:?}", final_output);

        let exp_vals = final_output.mapv(|x| (x - max).exp());

        let sum = exp_vals.sum();

        let final_output = exp_vals.mapv(|x| x / sum);


        //println!("sum {}", final_output.sum());

        (layer1_output, layer2_output, final_output)
    }

    // Backpropagation pass through the network. No return values but it is supposed to
    // update all weights.  It accepts the input, intermediate outputs and final outputs
    // as parameters as well as the target values.
    fn backward(
        &mut self,
        input: &Array2<f32>,
        layer1_output: &Array2<f32>,
        layer2_output: &Array2<f32>,
        final_output: &Array2<f32>,
        target: &Array2<f32>,
    ) {
        let err1= final_output - target;

        let gradient1 = err1.dot(&layer2_output.t());

        let relu_deriv = layer2_output.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 });

        let err2 = (self.weights_layer2_to_output.t().dot(&err1)) * relu_deriv;

        let gradient2 = err2.dot(&layer1_output.t());

        let relu_deriv = layer1_output.mapv(|x| if x > 0.0 { 1.0 } else { 0.0 });

        let err3 = (self.weights_layer1_to_layer2.t().dot(&err2)) * relu_deriv;

        let input: Array2<f32> = Array::from_shape_vec((input.len(), 1), input.iter().cloned().collect()).unwrap();

        let gradient3 = err3.dot(&input.t());

        self.weights_layer2_to_output -= &(gradient1 * self.learning_rate);

        self.weights_layer1_to_layer2 -= &(gradient2 * self.learning_rate);

        self.weights_input_to_layer1 -= &(gradient3 * self.learning_rate);
    }
}

fn main() {

    println!("Loading MNIST training set");

    let (x,y) = csv_to_ndarray("mnist_train.csv");

    println!("Loaded MNIST training set");

    let mut snn: NeuralNetwork = NeuralNetwork::new(784, 128,128,10, 0.005); //0.005

    // println!("{:?}", &x[1].shape());

    for j in 1..4{ 
        println!("Epoch {}", j);
        for i in 0..x.len(){
            let (x1, x2, x3) = snn.forward(&x[i]);
            let target = &y[i];
            snn.backward(&x[i], &x1, &x2, &x3, &target);
        }
    }

    println!("finished training");

    println!("Loading MNIST testing set");

    let (x2,y2) = csv_to_ndarray("mnist_test.csv");

    println!("Loaded MNIST testing set");

    let mut correct= 0;

    let total = y2.len();

    for k in 0..x2.len(){
        let (_x1, _x2, x3) = snn.forward(&x[k]);
        let target = &y[k];

        let mut est_value: f32 = 0.0;
        let mut idx: i32 = -1;
        for i in 0..x3.len(){
            if x3[[i,0]] > est_value as f32{
                est_value = x3[[i,0]];
                idx = i as i32;
            }
        }

        let mut act_value = 0;
        for i in 0..target.len(){
            if target[[i,0]] > act_value as f32{
                act_value = i;
            }
        }
        
        //println!("{:?},{}", est_value, act_value);
        
        if idx == act_value as i32 {
            correct += 1;
        }
    }

    println!("Accuracy {}%", (correct as f32 / total as f32) * 100.0);

    

}


