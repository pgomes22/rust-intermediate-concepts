fn main() {
    let square = |x| x * x;
    println!("5 square is {}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        // .map(|t| (t.0 + 1, t.1))
        .map(|(x, y)| (x + 1, y))
        .for_each(|t| println!("{:?}", t));
}
