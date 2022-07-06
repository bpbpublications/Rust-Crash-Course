use neural_networks::neural_networks::NeuralNetwork;
use neural_networks::{get_accuracy, train};

fn main() {
    let mut nn = NeuralNetwork::new(784, vec![5, 5], 2, 0.1, "sigmoid");
    let num_epochs = 5;
    train(&mut nn, "ZeroOne_train.csv", num_epochs);
    let acc = get_accuracy(&nn, "ZeroOne_test.csv") * 100.0;
    println!("Accuracy: {}%", acc);
}
