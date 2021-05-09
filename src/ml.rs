use ndarray::prelude::*;
use polars::prelude::*;
use rand::prelude::*;

fn random_shuffle(matrix: &mut Array2<f64>) {
    let mut rng = thread_rng();

    for i in 1..matrix.nrows() {
        let mut it = matrix.outer_iter_mut();
        let j = rng.gen_range(0..i);

        ndarray::Zip::from(it.nth(j).unwrap())
            .and(it.nth(i - (j + 1)).unwrap())
            .apply(std::mem::swap);
    }
}

pub fn split(df: &DataFrame, train_ratio: f64) -> (Array2<f64>, Array2<f64>) {
    let mut matrix: Array2<f64> = df.to_ndarray::<Float64Type>().unwrap();
    random_shuffle(&mut matrix);

    let split_point = (train_ratio * matrix.nrows() as f64) as usize;

    let (train_set, test_set) = (
        matrix.slice(s![..split_point, ..]),
        matrix.slice(s![split_point.., ..]),
    );

    (train_set.to_owned(), test_set.to_owned())
}

fn sigmoid(z: Array2<f64>) -> Array2<f64> {
    z.mapv(|x| 1. / (1. + (-x).exp()))
}

fn forward_backward(
    weights: &Array2<f64>,
    bias: &f64,
    x_train: &Array2<f64>,
    y_train: &Array2<f64>,
) -> (f64, Array2<f64>, f64) {
    // forward
    let y_head: Array2<f64> = sigmoid(weights.t().dot(x_train).mapv(|x| x + bias));
    let loss = (y_train * &(y_head.mapv(|z| z.ln()))
        + &((y_train.mapv(|z| 1. - z)) * &y_head.mapv(|z| (1. - z).ln())))
        .mapv(|z| -1. * z);
    let cost = loss.sum() / x_train.ncols() as f64;

    // backward
    let d_weights = (x_train.dot(&(&y_head - y_train).t())).mapv(|z| z / x_train.ncols() as f64);
    let d_bias = (&y_head - y_train).sum() / x_train.ncols() as f64;

    (cost, d_weights, d_bias)
}

fn update(
    weights: Array2<f64>,
    bias: f64,
    x_train: Array2<f64>,
    y_train: Array2<f64>,
    learning_rate: f64,
    iterations: usize,
) -> (Vec<f64>, Array2<f64>, f64) {
    let mut costs: Vec<f64> = Vec::new();
    let mut weights = weights;
    let mut bias = bias;

    for _ in 0..iterations {
        let (cost, d_weight, d_bias) = forward_backward(&weights, &bias, &x_train, &y_train);
        weights -= &d_weight.mapv(|x| learning_rate * x);
        bias -= learning_rate * d_bias;

        costs.push(cost);
    }

    (costs, weights, bias)
}

fn predict(weights: &Array2<f64>, bias: &f64, x_test: &Array2<f64>) -> Array2<f64> {
    sigmoid((&weights.t().dot(x_test)).mapv(|z| z + bias)).mapv(|z| if z <= 0.5 { 0. } else { 1. })
}

pub fn train(
    train_set: &Array2<f64>,
    learning_rate: f64,
    iterations: usize,
) -> (Vec<f64>, Array2<f64>, f64) {
    let x_train: Array2<f64> = train_set.slice(s![.., ..-1]).t().to_owned();
    let y_train: Array2<f64> = train_set.slice(s![.., -1..]).t().to_owned();

    let weights = Array2::from_elem([x_train.nrows(), 1], 0.01);
    let bias = 0.0;

    update(weights, bias, x_train, y_train, learning_rate, iterations)
}

pub fn accuracy(y_test: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    100. - (y_pred - y_test).mapv(|z| z.abs() * 100.).mean().unwrap()
}
