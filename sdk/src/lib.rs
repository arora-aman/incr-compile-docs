pub mod generic;
pub mod non_generic;

#[cfg(test)]
mod tests {
    use crate::non_generic;

    #[test]
    fn concat_test() {
        let a = "Str";
        let b = "Concat";

        assert_eq!("StrConcat", non_generic::strcat(a, b));
    }

    #[test]
    fn strlen_test() {
        let a = "Str";
        let b = "Concat";

        assert_eq!(9, non_generic::total_strlen(&vec![a, b]));
    }
}
