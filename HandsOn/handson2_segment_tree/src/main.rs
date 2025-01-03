use handson2_segment_tree::FreqSTree;
use handson2_segment_tree::MaxSTree;
use std::fs::File;
use std::io::{self, BufRead};

/// This program checks the correctness of the implemented solutions
/// for the second HandsOn from the "Competitive Programming and Contests"
/// 2024/25 course held at the University of Pisa.
/// This code tests the solutions to the two assigned problems, `Min and Max` and `IsThere`,
/// using the test sets provided.
/// The program assumes that the tests are stored in the folders "Testset_handson2_p1"
/// and "Testset_handson2_p2" at the root of the cargo project.
fn main() {
    let tests = vec![11, 8]; //TODO put first element to 11
    for i in 1..=2 {
        println!("{}", "Testing Problem #$".replace("$", &i.to_string()));
        for j in 0..tests[i - 1] {
            let files = get_files(i, j);
            let res = match i {
                1 => min_max(files.0, files.1),
                _ => is_there(files.0, files.1),
            };
            print_test_result(res, j);
        }
    }
}

/// Prints the message "Test i - Passed" or "Test i - Failed"
/// based on the outcome of the `i`-th test.
fn print_test_result(b: bool, i: usize) {
    let base_success = "Test $ - Passed";
    let base_fail = "Test $ - Failed";
    if b {
        let msg = base_success.replace("$", &i.to_string());
        println!("{msg}");
    } else {
        let msg = base_fail.replace("$", &i.to_string());
        println!("{msg}");
    }
}

/// Returns the test pair (input,output) for the
/// `j`-th test file of the `i`-th problem
fn get_files(i: usize, j: usize) -> (File, File) {
    let base_in = "./Testset_handson2_p*/input$.txt";
    let base_out = "./Testset_handson2_p*/output$.txt";
    let input = base_in
        .replace("*", &i.to_string())
        .replace("$", &j.to_string());
    let output = base_out
        .replace("*", &i.to_string())
        .replace("$", &j.to_string());
    let input = File::open(&input).unwrap();
    let output = File::open(&output).unwrap();
    (input, output)
}

/// Executes the `Is There` problem for a given pair
/// of test files `(input, output)`.
/// Returns `true` if the generated output matches the
/// expected `output`, `false` otherwise.
fn is_there(input: File, output: File) -> bool {
    // Creating the iterator for scanning the file
    let input = io::BufReader::new(input);
    let mut input = input.lines();

    // Determining n and m
    let n_m = input.next().unwrap().unwrap();
    let n_m = get_inputs(Some(2), n_m).unwrap();

    let (n, m) = (n_m[0], n_m[1]);

    // Constructing s
    let mut s: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let l_r = input.next().unwrap().unwrap();
        let l_r = get_inputs(Some(2), l_r);
        if let Some(vec) = l_r {
            s.push((vec[0], vec[1]));
        } else {
            eprintln!("Error while parsing segments");
            return false;
        }
    }

    // Creating the tree
    let tree = FreqSTree::new(s);
    let tree = tree.unwrap();

    // Store results of IsThere queries in `res`
    let mut res: Vec<usize> = Vec::new();

    // Execute m queries from file
    for _ in 0..m {
        let q = input.next().unwrap().unwrap();
        let q = get_inputs(Some(3), q);

        if let Some(vec) = q {
            let val = tree.is_there((vec[0], vec[1]), vec[2]);
            res.push(val);
        }
    }
    check_outputs(output, res)
}

/// Executes the `Min and Max` problem for a given pair
/// of test files `(input, output)`.
/// Returns `true` if the generated output matches the
/// expected `output`, `false` otherwise.
fn min_max(input: File, output: File) -> bool {
    // Creating the iterator for scanning the file
    let input = io::BufReader::new(input);
    let mut input = input.lines();

    // Determining n and m
    let n_m = input.next().unwrap().unwrap();
    let n_m = get_inputs(Some(2), n_m).unwrap();

    let (n, m) = (n_m[0], n_m[1]);

    // Determing a
    let a = input.next().unwrap().unwrap();
    let a = get_inputs(Some(n), a).unwrap();

    // Construct the tree
    let mut tree = MaxSTree::new(&a).unwrap();

    // Store results of max queries in `res`
    let mut res: Vec<usize> = Vec::new();

    // Execute m queries from file
    for _ in 0..m {
        let q = input.next().unwrap().unwrap();
        let q = get_inputs(None, q);

        if let Some(vec) = q {
            match vec[0] {
                0 => tree.update((vec[1], vec[2]), vec[3]),
                _ => {
                    if let Some(val) = tree.max((vec[1], vec[2])) {
                        res.push(val);
                    }
                }
            }
        }
    }

    check_outputs(output, res)
}

/// Given a set of outputs `res` checks if the results
/// match the ones of the associated `output` test file.
fn check_outputs(output: File, res: Vec<usize>) -> bool {
    // Checking that the results are equal to the ones in output
    let output = io::BufReader::new(output);
    let mut output = output.lines();
    for item in res {
        let to_check = output.next().unwrap().unwrap();
        let to_check: usize = to_check.parse().unwrap();
        if item != to_check {
            return false;
        }
    }
    true
}

/// Gets an input line from standard input
/// and parse it into a vector of `n` positive ints.
/// If n = `None` it means we are getting a query for
/// the `Min and Max` problem.
/// In the case after getting the input it will read
/// the first element in the parsed vector and
/// if vec[0] = 0 -> `update` query then vec.len() == 4
/// if vec[0] = 1 -> `max` query then vec.len() == 3 .
/// For any kind of error while parsing returns `None`
fn get_inputs(n: Option<usize>, str: String) -> Option<Vec<usize>> {
    let inputs: Vec<&str> = str.trim().split_whitespace().collect();

    // Detemining n
    let n = if let Some(val) = n {
        Some(val)
    } else {
        let val: usize = inputs[0].parse().unwrap();
        match val {
            0 => Some(4),
            1 => Some(3),
            _ => None,
        }
    };

    // Convert inputs into a vector of uints
    if Some(inputs.len()) == n {
        let ret = inputs.iter().map(|s| s.parse::<usize>()).collect();
        match ret {
            Ok(ret) => Some(ret),
            Err(_) => {
                eprintln!("Error while parsing the input");
                None
            }
        }
    } else {
        eprintln!("Gotten a string with more elems than requested");
        None
    }
}
