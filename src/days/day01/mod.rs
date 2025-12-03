use std::{fs, path::PathBuf};

use color_eyre::eyre::{Result, eyre};

const INPUT_RELATIVE: &str = "src/days/day01/assets/input.txt";

/// Mock implementation for Day 01 that sums lines in `src/days/day01/assets/input.txt`.
pub fn run() -> Result<String> {
    let asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(INPUT_RELATIVE);
    if !asset_path.exists() {
        return Err(eyre!(
            "missing input file at {}",
            asset_path.to_string_lossy()
        ));
    }

    let input = fs::read_to_string(asset_path)?;
    let mut total = 0_i64;
    for (idx, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value: i64 = trimmed
            .parse()
            .map_err(|err| eyre!("failed to parse line {} ('{}'): {}", idx + 1, trimmed, err))?;
        total += value;
    }

    Ok(format!("Day 01 mock answer: {}", total))
}
