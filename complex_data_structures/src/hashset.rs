pub mod hashset_data {
    use std::collections::HashSet;
    pub fn hashset_explain() -> () {
        // Fazer uma explicacao sobre como o Into_iter e Collect funcionam nesse caso
        let mut set_data: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    }
}
