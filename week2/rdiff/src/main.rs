use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{ self, BufRead }; // For read_file_lines()
use std::process;
use std::cmp::max;
pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
//#[allow(unused)] // TODO: delete this line when you implement this function
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    //unimplemented!();
    //let file = File::open(filename).unwrap();
    let path = File::open(filename)?;
    let mut lines = Vec::new();
    for line in io::BufReader::new(path).lines() {
        let line_str = line?;
        lines.push(line_str);
    }
    Ok(lines)
    // Be sure to delete the #[allow(unused)] line above
}

//#[allow(unused)] // TODO: delete this line when you implement this function
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
    let m = seq1.len();
    let n = seq2.len();
    let mut lcs_table = Grid::new(m + 1, n + 1);
    for i in 0..m + 1 {
        lcs_table.set(i, 0, 0).unwrap();
    }
    for j in 0..n + 1 {
        lcs_table.set(0, j, 0).unwrap();
    }

    for i in 0..m {
        for j in 0..n {
            if seq1[i] == seq2[j] {
                let val = lcs_table.get(i, j).unwrap() + 1;
                let _ = lcs_table.set(i + 1, j + 1, val);
            } else {
                let val1 = lcs_table.get(i + 1, j).unwrap();
                let val2 = lcs_table.get(i, j + 1).unwrap();
                lcs_table.set(i + 1, j + 1, max(val1, val2)).unwrap();
            }
        }
    }
    lcs_table
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(
    lcs_table: &Grid,
    lines1: &Vec<String>,
    lines2: &Vec<String>,
    mut i: usize,
    mut j: usize
) {
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above

    // if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
    //     print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
    //     println!("  {}", lines1[i - 1]);
    // } else if
    //     // j > 0 and (i = 0 or C[i,j-1] ≥ C[i-1,j])
    //     j > 0 &&
    //     (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
    // {
    //     print_diff(lcs_table, lines1, lines2, i, j - 1);
    //     println!("> {}", lines2[j - 1]);
    // } else if
    //     i > 0 &&
    //     (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
    // {
    //     print_diff(lcs_table, lines1, lines2, i - 1, j);
    //     println!("< {}", lines1[i - 1]);
    // } else {
    //     println!(" ");
    // }

    let mut ops = Vec::new();
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
            ops.push(format!("  {}", lines1[i - 1]));
            i -= 1;
            j -= 1;
        } else if
            j > 0 &&
            (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
        {
            ops.push(format!("> {}", lines2[j - 1]));
            j -= 1;
        } else if
            i > 0 &&
            (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
        {
            ops.push(format!("> {}", lines1[i - 1]));
            i -= 1;
        }
    }

    for line in ops.iter() {
        println!("{}", line);
    }
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let lines1_result = read_file_lines(filename1).expect("Failed to read first file");
    let lines2_result = read_file_lines(filename2).expect("Failed to read second file");
    let lcs_table = lcs(&lines1_result, &lines2_result);
    let m = lines1_result.len();
    let n = lines2_result.len();
    print_diff(&lcs_table, &lines1_result, &lines2_result, m, n);
    //unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd"
                .chars()
                .map(|c| c.to_string())
                .collect(),
            &"adb"
                .chars()
                .map(|c| c.to_string())
                .collect()
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
