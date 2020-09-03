mod mod1;
mod mod2;

fn main() {
    let link = mod1::rust_repository_link();
    let link_length = mod1::rust_repository_strlen();

    assert_eq!(link.len(), link_length);

    println!("Rust repo is on github at {}", link);


    let names = vec!["Alice", "Bob", "Carol", "Dave"];

    let names_concat = mod2::concat_names(&names);
    let names_total_len = mod2::total_length(&names);

    assert_eq!(names_concat.len(), names_total_len);

    println!("Concatnated names = {}", names_concat);
}
