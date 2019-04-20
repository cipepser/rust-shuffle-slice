# Rustでsliceをシャッフルする

[O'Reilly Japan \- プログラミングRust](https://www.oreilly.co.jp/books/9784873118550/)を読んでいたら
[rand::Rng \- Rust](https://doc.rust-lang.org/1.1.0/rand/trait.Rng.html)でsliceのシャッフルができるとあったので試したところ`rand::Rng::shuffle`がdeprecatedになっていました。
`SliceRandom::shuffle`を使った例を残します。

`Cargo.toml`は以下のとおりです。

```toml
[dependencies]
rand = "0.6.5"
```

実装例

```rust
use rand::seq::SliceRandom;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);
    println!("{:?}", v); // [3, 4, 7, 6, 2, 0, 5, 9, 1, 8] 
}
```

## deprecatedな実装

```rust
use rand::Rng;

fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng();
    rng.shuffle(&mut v);
    println!("{:?}", v);
}
```

```sh
❯ cargo run
warning: use of deprecated item 'rand::Rng::shuffle': use SliceRandom::shuffle instead
 --> src/main.rs:7:9
  |
7 |     rng.shuffle(&mut v);
  |         ^^^^^^^
  |
  = note: #[warn(deprecated)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/rust-shuffle-slice`
[1, 2, 4, 8, 3, 5, 7, 0, 9, 6]
```

## References
- [O'Reilly Japan \- プログラミングRust](https://www.oreilly.co.jp/books/9784873118550/)
- [rand::Rng \- Rust](https://doc.rust-lang.org/1.1.0/rand/trait.Rng.html)