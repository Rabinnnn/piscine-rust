pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.chars().count();
    let n = target.chars().count();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}