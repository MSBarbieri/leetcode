#![allow(dead_code)]
pub mod utils;
pub use utils::read_file;

use anyhow::Result;

#[test]
fn test_1() -> Result<()> {
    #[path = "1.rs"]
    pub mod current_import;
    pub use current_import::Solution;

    let values = read_file("./src/1.tests.dat")?;

    for i in (0..values.len()).step_by(2) {
        let list = serde_json::from_value(values[i].clone())?;
        let target = serde_json::from_value(values[i + 1].clone())?;

        let result = current_import::Solution::two_sum(list, target);
        println!(
            "values: {:?} ; {:?}\n result: {:?}\n--------------",
            values[i],
            values[i + 1],
            result
        );
    }
    // -----------------------------------

    Ok(())
}

fn main() -> Result<()> {
    Ok(())
}
