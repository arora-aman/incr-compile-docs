use sdk::generic;

pub fn print_int_details(x: i32) -> String {
    let s1 = generic::stringify_and_cat("The integers is ", x);
    let s2 = generic::stringify_and_cat(" and requires ", generic::stringify_length(x));
    
    let s = generic::stringify_and_cat(s1, s2);

    generic::stringify_and_cat(s, "chars")
}

