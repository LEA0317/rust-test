fn main() {
    const INF: usize = 1_000;
    const M: i32 = 10_000;
    let n: i32 = 3;
    let m: i32 = 3;
    let mut a: [i32; 3*INF] = [0 as i32; 3*INF];
    let mut dp = [[0 as i32; INF]; INF];

    for i in 0..n+1 {
    	dp[i as usize][0] = 1;
    }
    for i in 0..3*INF {
    	a[i as usize] = (i % 3) as i32 + 1;
    }

    for i in 0..n {
        for j in 1..m+1 {
       	    if j - 1 - a[i as usize] >= 0 {
       	        dp[(i + 1) as usize][j as usize]
		      = (dp[(i + 1) as usize][(j - 1) as usize] + dp[i as usize][j as usize] - dp[i as usize][(j - 1 - a[i as usize]) as usize] + M) % M;
       	    } else {
       	        dp[(i + 1) as usize][j as usize] = (dp[(i + 1) as usize][(j - 1) as usize] + dp[i as usize][j as usize]) % M;
       	    }
        }
    }
    println!("{}", dp[n as usize][m as usize]); 
}
