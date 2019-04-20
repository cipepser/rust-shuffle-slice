use rand::Rng;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng();
    rng.shuffle(&mut v);
    println!("{:?}", v);
}
