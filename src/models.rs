use std::cmp::min;



/// Naive Levensthein distance predictor.
/// this is incredibly slow, but stands as a contrast to
/// implementation using an SGD Classifier. 
pub fn levensthein_distance(a: &str, b: &str) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..a.len() + 1 {
        for j in 0..b.len() + 1 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                dp[i][j] = min(dp[i - 1][j - 1] + (if a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 1).unwrap() { 0 } else { 1 }), min(dp[i - 1][j] + 1, dp[i][j - 1] + 1));
            }
        }
    }
    return dp[a.len()][b.len()];
}