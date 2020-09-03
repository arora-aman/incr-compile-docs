use sdk::non_generic;

pub fn rust_repository_link() -> String {
    non_generic::strcat("rust-lang/", "rust")
}

pub fn rust_repository_strlen() -> usize {
    non_generic::total_strlen(vec!["rust-lang/", "rust"])
}

