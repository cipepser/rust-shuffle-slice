use rand::seq::SliceRandom;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    println!("{:?}", v);
}
