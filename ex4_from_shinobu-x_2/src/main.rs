fn main() {
    const INF: usize = 1_000;
    const M: i32 = 10_000;
    let n: i32 = 4;
    let m: i32 = 3;
    let mut dp = [[0 as i32; INF]; INF];

    dp[0][0] = 1;
    for i in 1..m+1 {
        for j in 0..n+1 {
       	    if j - i >= 0 {
       	        dp[i as usize][j as usize] = (dp[(i - 1) as usize][j as usize] + dp[i as usize][(j - i) as usize]) % M;
       	    } else {
       	        dp[i as usize][j as usize] = dp[(i - 1) as usize][j as usize];
       	    }
        }
    }
    println!("{}", dp[m as usize][n as usize]); 
}
