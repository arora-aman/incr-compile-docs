mod mod1;
// mod mod2;

fn main() {
    let link = mod1::rust_repository_link();
    let link_length = mod1::rust_repository_strlen();

    assert_eq!(link.len(), link_length);

    println!("Rust repo is on github at {}", link);
}
