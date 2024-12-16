use std::{fmt::Debug, fs, str::FromStr, vec};

pub fn parse_line<T: FromStr<Err: Debug>>(line: &str) -> Vec<T> {
    line.split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

fn solve1(input: String) -> i64 {
    let input_lines = input.trim().lines().collect::<Vec<_>>();

    let n: usize = parse_line(input_lines[0])[0];
    let d: usize = parse_line(input_lines[0])[1];

    let mut m = Vec::new();

    // println!("lines: {:?}", input_lines);

    //fill each line with psum
    for l in &input_lines[1..] {
        let v = parse_line::<_>(l);
        assert!(v.len() == d);

        //prefix sums
        let p = v
            .iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        m.push(p);
    }

    // println!("m: {:?}", m);

    let mut dp = vec![vec![0; d]; n];

    for i in 0..n {
        for j in 0..d {
            if i == 0 {
                dp[i][j] = m[i][j];
            } else if j == 0 {
                dp[i][j] = m[i][j].max(dp[i - 1][j]);
            } else {
                let mut max = m[i][j].max(dp[i - 1][j]);
                for k in 0..j {
                    max = max.max(m[i][j - 1 - k] + dp[i - 1][k]);
                }
                dp[i][j] = max;
            }
        }
    }

    // println!("dp: {:?}", dp);
    dp[n - 1][d - 1]
}

fn solve2(input: String) -> usize {
    let input_lines = input.trim().lines().collect::<Vec<_>>();

    let n: usize = parse_line(input_lines[0])[0];

    // vector of (beauty, difficulty)
    let mut v = Vec::with_capacity(n);

    //read input
    for l in &input_lines[1..] {
        let parsed = parse_line::<i64>(l);
        v.push((parsed[0], parsed[1]));
    }

    // sort by beauty, ties are resolved by comparing difficulty
    v.sort();

    // Custom Longest Increasing Subsequence ---
    let mut dom_points = Vec::with_capacity(v.len());

    // println!("{:?}", v);
    dom_points.push(v[0]);

    for &el in v.iter().skip(1) {
        let last = dom_points[dom_points.len() - 1];
        if last.1 < el.1 && last.0 != el.0 {
            dom_points.push(el);
        } else if last.0 != el.0 {
            //binary search
            let x = dom_points.partition_point(|y| y.0 < el.0 && y.1 < el.1);

            // same semantics as this
            // let x = match dom_points.binary_search_by(|y| {
            //     if el.0 > y.0 && el.1 > y.1 {
            //         Ordering::Less
            //     } else {
            //         Ordering::Greater
            //     }
            // }) {
            //     Ok(x) => x,
            //     Err(x) => x,
            // };

            dom_points[x] = el;
        }
    }

    // println!("{:?}", dom_points);

    dom_points.len()
}

fn main() {
    println!("Exercise 1 -----------------------");
    let n_tests = 4;
    let tests_folder = "/home/anglo/uni/cpc/hands_on_3/testcases/ex1/";

    for i in 0..=n_tests {
        print!("test n {}\t", i);
        let input_path = format!("{}input{}.txt", tests_folder, i);
        let output_path = format!("{}output{}.txt", tests_folder, i);
        let input = fs::read_to_string(input_path).expect("Cannot read input file");

        let my_out = solve1(input);
        let test_out = fs::read_to_string(output_path).expect("Cannot read input file");

        // print!("{}\t", my_out);
        // print!("{}\t", test_out.trim().parse::<i64>().unwrap());

        if test_out.trim().parse::<i64>().unwrap() == my_out {
            println!(" OK!");
        } else {
            println!(" WRONG!");
        }
    }

    println!("Exercise 2 -----------------------");
    let n_tests = 10;
    let tests_folder = "/home/anglo/uni/cpc/hands_on_3/testcases/ex2/";

    for i in 0..=n_tests {
        print!("test n {}\t", i);
        let input_path = format!("{}input{}.txt", tests_folder, i);
        let output_path = format!("{}output{}.txt", tests_folder, i);
        let input = fs::read_to_string(input_path).expect("Cannot read input file");

        let my_out = solve2(input);
        let test_out = fs::read_to_string(output_path).expect("Cannot read input file");

        // print!("{}\t", my_out);
        // print!("{}\t", test_out.trim().parse::<usize>().unwrap());

        if test_out.trim().parse::<usize>().unwrap() == my_out {
            println!(" OK!");
        } else {
            println!(" WRONG!");
        }
    }
}

#[test]
fn custom_test_1() {
    let input = "
    2 3
    3 2 1
    3 1 1
    ";

    println!("{:?}", solve1(input.to_string()))
}

#[test]
fn custom_test_2() {
    // let i = 9;
    // let tests_folder = "/home/anglo/uni/cpc/hands_on_3/testcases/ex2/";
    // let input_path = format!("{}input{}.txt", tests_folder, i);
    // let input = fs::read_to_string(input_path).expect("Cannot read input file");

    let input = "
    5
    0 3    
    99 1   
    11 20
    1 2
    10 5
    ";

    println!("{:?}", solve2(input.to_string()))
}
