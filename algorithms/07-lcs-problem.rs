use std::cmp::max;



fn main() {

    let s1= "OHELOD";
    let s2 = "HELLOWORLD";

    let s1_len = s1.len();
    let s2_len = s2.len();

    let mut dp = vec![vec![0;s2_len+1];s1_len+1];


    for i in 1..=s1_len {
        for j in 1..=s2_len {
            if s1.as_bytes()[i-1] == s2.as_bytes()[j-1] {
                dp[i][j] =  dp[i-1][j-1] + 1;
            }else {
                dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
            }
        }
    }

    // print the dp table
    for val in &dp {
        println!("{:?}",val);
    }

    let mut lcs = vec![];


    let mut i = s1_len;
    let mut j = s2_len;

    while i != 0 && j != 0 {
        if dp[i][j] == dp[i-1][j] {
            i -= 1;
        }else if dp[i][j] == dp[i][j-1] {
            j -= 1;
        }else {
            lcs.push(s1.as_bytes()[i-1] as char);
            i -= 1;
            j -= 1;  
        }
    }

    lcs.reverse();

    println!("{:?}",lcs);
}