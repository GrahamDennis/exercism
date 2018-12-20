fn col_iter<'a>(input: &'a [Vec<u64>], col_idx: usize) -> impl Iterator<Item = u64> + 'a {
    (0..input.len()).map(move |row_idx| input[row_idx][col_idx])
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    let num_columns = input.get(0).map_or(0, |row| row.len());

    let row_maxes: Vec<u64> = input.iter().map(|row| row.iter().cloned().max().unwrap_or(u64::max_value())).collect();
    let col_mins: Vec<u64> = (0..num_columns).map(|col_idx| col_iter(input, col_idx).min().unwrap_or(u64::min_value())).collect();

    for row_idx in 0..input.len() {
        for col_idx in 0..num_columns {
            let value = input[row_idx][col_idx];
            if value == row_maxes[row_idx] && value == col_mins[col_idx] {
                result.push((row_idx, col_idx));
            }
        }
    }

    result
}
