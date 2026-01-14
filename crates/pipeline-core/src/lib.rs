pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}



pub mod pipelines;
pub mod states;
pub mod utils;

// Re-export for convenience
pub use pipelines::{parse_bulk_params, parse_sc_params};
pub use states::{AppParams, AppSCParams, Genome, Workflow};
