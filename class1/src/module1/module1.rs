pub mod module_a_bz {
    pub fn print_a_to_bz() {
        for c in ('Z'..='a').rev() {
            print!("{} ", c);
        }
    }
}
