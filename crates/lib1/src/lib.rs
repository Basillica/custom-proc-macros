pub fn lib1_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = lib1_add(2, 2);
        assert_eq!(result, 4);
    }
}