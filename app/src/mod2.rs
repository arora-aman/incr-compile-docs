use sdk::non_generic;

pub fn concat_names(names: &Vec<&str>) -> String {
    let mut result = String::new();

    for name in names {
        result = non_generic::strcat(&result, name);
    }

    result
}

pub fn total_length(names: &Vec<&str>) -> usize {
    non_generic::total_strlen(names)
}

