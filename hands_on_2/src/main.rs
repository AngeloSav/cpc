use hands_on_2::{IsThereSegmentTree, MinMaxSegmentTree};
use std::{fmt::Debug, fs, str::FromStr};

fn parse_line<T: FromStr<Err: Debug>>(line: &str) -> Vec<T> {
    line.split_whitespace()
        .map(|x| x.parse::<T>().unwrap())
        .collect::<Vec<_>>()
}

fn solve1(in_file: String) -> String {
    // println!("opening file {}", in_file);
    let input_content = fs::read_to_string(in_file).expect("Cannot read input file");

    let input_lines = input_content.lines().collect::<Vec<_>>();

    let _n: usize = parse_line(input_lines[0])[0];
    let _m: usize = parse_line(input_lines[0])[1];

    let input_v = parse_line(input_lines[1]);
    let mut stree = MinMaxSegmentTree::new(input_v);

    let mut my_out = String::new();

    for l in &input_lines[2..] {
        let v: Vec<i64> = parse_line(l);
        let l = v[1] as usize - 1;
        let r = v[2] as usize - 1;

        if v[0] == 1 {
            //query
            let res = stree.query(l, r);
            // dbg!(res);
            my_out = format!("{}{}\n", my_out, res);
        } else {
            //update
            stree.update_range(l, r, v[3]);
        }
    }

    my_out
}

fn solve2(in_file: String) -> String {
    let input_content = fs::read_to_string(in_file).expect("Cannot read input file");

    let input_lines: Vec<_> = input_content.lines().collect();
    let n: usize = parse_line(input_lines[0])[0];
    let _m: usize = parse_line(input_lines[0])[1];

    let mut arr = vec![0; n + 1];

    for l in &input_lines[1..n + 1] {
        let v = parse_line::<usize>(l);
        arr[v[0]] += 1;
        arr[v[1] + 1] += -1;
    }

    //prefix sums
    let p = arr
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc as usize)
        })
        .collect::<Vec<_>>();
    // println!("{:?}", p);

    let mut my_out = String::new();

    //queries
    let stree = IsThereSegmentTree::new(p);
    for l in &input_lines[n + 1..] {
        let v: Vec<usize> = parse_line(l);
        let l = v[0];
        let r = v[1];
        let k = v[2];
        // println!("q: [{}, {}] | k = {}", l, r, k);
        let res = if stree.query(l, r, k) { 1 } else { 0 };
        my_out = format!("{}{}\n", my_out, res);
    }

    my_out
}

fn main() {
    println!("Exercise 1 -----------------------");
    let n_tests = 11;
    let tests_folder = "/home/anglo/uni/cpc/hands_on_2/testcases/ex1/";

    for i in 0..n_tests {
        print!("test n {}\t", i);
        let input_path = format!("{}input{}.txt", tests_folder, i);
        let output_path = format!("{}output{}.txt", tests_folder, i);

        //parse input and solve
        let my_out = solve1(input_path);

        // test with output
        let test_out = fs::read_to_string(output_path).expect("Cannot output input file");
        // dbg!(&my_out);
        // dbg!(&test_out);

        assert!(test_out == my_out, "test failed!");
        println!(" OK!");
    }

    println!("Exercise 2 -----------------------");
    let n_tests = 8;
    let tests_folder = "/home/anglo/uni/cpc/hands_on_2/testcases/ex2/";

    for i in 0..n_tests {
        print!("test n {}\t", i);
        let input_path = format!("{}input{}.txt", tests_folder, i);
        let output_path = format!("{}output{}.txt", tests_folder, i);

        //parse input and solve
        let my_out = solve2(input_path);

        // test with output
        let test_out = fs::read_to_string(output_path).expect("Cannot output input file");
        // dbg!(&my_out);
        // dbg!(&test_out);

        assert!(test_out == my_out, "test failed!");
        println!(" OK!");
    }
}
