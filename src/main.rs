use ndarray::{Array, ArrayView, Axis};

// create an empty array and push rows to it

// assert_eq!(
//     a,
//     array![[1., 1., 1., 1.],
//            [0., 0., 0., 0.],
//            [1., 1., 1., 1.]]);
fn main() {

    let mut a = Array::zeros((0, 6));
    let ones  = ArrayView::from(&[2.; 6]);
    let zeros = ArrayView::from(&[0.; 6]);
    a.push(Axis(0), ones).unwrap();
    a.push(Axis(0), zeros).unwrap();
    a.push(Axis(0), ones).unwrap();

    println!("{}", ones);
}