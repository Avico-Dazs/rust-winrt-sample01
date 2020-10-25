winrt::import!(
    dependencies
        os
    types
        windows::system::diagnostics::*
);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
