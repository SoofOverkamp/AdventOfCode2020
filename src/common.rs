use std::{fs, str};
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;

pub type AOCResult = Result<(Option<String>, Option<String>), Box<dyn Error>>;

pub trait AOCProgram {
    fn run(&self, input: &Vec<String>) -> AOCResult;
}

impl<F: Fn(&Vec<String>) -> AOCResult> AOCProgram for F {
    fn run(&self, input: &Vec<String>) -> AOCResult {
        self(input)
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    return lines.collect();
}

pub fn run<F: AOCProgram>(day: &str, prog: &F) {
    let inputs = files_to_vec(day, "./inputs");

    if let Err(e) = inputs {
        eprintln!("Error occurred while reading input files {:?}", e);
        return;
    }
    let inputs = inputs.unwrap();

    for (path, input) in inputs {
        let result = prog.run(&input);
        match result {
            Err(e) => println!("{}: Failed on input {:?} with error {:?}", path, &input, e),
            Ok(result) => match result {
                (Some(o1), Some(o2)) => println!("{}:\n\t1:{}\n\t2:{}",path, o1, o2),
                (Some(o1), None) => println!("{}:\n\t1:{}",path, o1),
                (None, Some(o2)) => println!("{}:\n\t2:{}",path, o2),
                (None, None) => println!("{}: No output",path)
            }
        }
    }
}


pub fn run_test<F: AOCProgram>(day: &str, prog: &F) -> bool {
    let expected_re: regex::Regex = regex::Regex::new(r"([^\$]+)?(?:\$([^\$]+))?").unwrap();

    let inputs = files_to_vec(day, "./test_inputs");

    if let Err(e) = inputs {
        eprintln!("Error occurred while reading test input files {:?}", e);
        return false;
    }
    let inputs = inputs.unwrap();

    if inputs.len() == 0 {
        println!("Tests failed: No valid input files given");
        return false;
    }

    let mut succeed = true;

    for (path, mut input) in inputs.clone() {
        let expected = input.pop();
        if expected.is_none() {
            println!("{}: empty", path);
            succeed = false;
            continue;
        }
        let expected = expected.unwrap();

        let exp_caps = expected_re.captures(&expected);
        let exp1 = exp_caps.as_ref().and_then(|cap| cap.get(1)).map(|m| m.as_str());
        let exp2 = exp_caps.as_ref().and_then(|cap| cap.get(2)).map(|m| m.as_str());

        let result = test::<F>(prog, &input, (exp1, exp2), &path);
        succeed = succeed && result.0.unwrap_or(true) && result.1.unwrap_or(true);
    }

    return succeed;
}

fn test<F: AOCProgram>(prog: &F, input: &Vec<String>, expected: (Option<&str>, Option<&str>), path: &str) -> (Option<bool>, Option<bool>) {

    let result = prog.run(input);
    if result.is_err() {
        println!("{}: Program failed on input {:?} with error {:?}", path, &input[..10], result.unwrap_err());
        return (Some(false), Some(false));
    }
    let result = result.unwrap();

    let succeeded = (
        result.0
            .as_ref()
            .ok_or("Program did not produce output")
            .and_then(|r| expected.0
                .ok_or("No expected output")
                .map(|e| e == r.as_str())
            ),
        result.1
            .as_ref()
            .ok_or("Program did not produce output")
            .and_then(|r| expected.1
                .ok_or("No expected output")
                .map(|e| e == r.as_str())
            ),
    );
    if succeeded.0.is_err() {
        println!("{}: Skipped testing result 1: {}", path, succeeded.0.unwrap_err())
    } else {
        if succeeded.0.unwrap() {
            println!("{}: Result 1 succeeded", path)
        } else {
            eprintln!("{}: Result 1 failed: Wrong output for input {:?}.\n\tExpected {}\n\tReceived {}", path, &input[..10], expected.0.unwrap(), result.0.unwrap());
        }
    }
    if succeeded.1.is_err() {
        println!("{}: Skipped testing result 2: {}", path, succeeded.1.unwrap_err())
    } else {
        if succeeded.1.unwrap() {
            println!("{}: Result 2 succeeded", path)
        } else {
            eprintln!("{}: Result 2 failed: Wrong output for input {:?}.\n\tExpected {}\n\tReceived {}", path, &input[..10], expected.1.unwrap(), result.1.unwrap());
        }
    }

    return (succeeded.0.ok(), succeeded.1.ok())
}

fn files_to_vec<'a>(day: &str, input_path: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn Error>> {
    let dir = fs::read_dir(input_path)?;

    let file_name_re = regex::Regex::new(&format!(r"{}.*(?:.txt)?", day))?;

    let results = dir.map::<io::Result<_>,_>(|f| {
        let f = f?;

        if !f.metadata()?.is_file() {
            return Ok(None);
        }

        let path = f.path();

        let file_name = path.file_name().and_then(|s| s.to_str());

        if !file_name.map(|s| file_name_re.is_match(s)).unwrap_or(false) {
            return Ok(None);
        }
        let file_name = String::from(file_name.unwrap());

        return Ok(Some((file_name, read_lines(f.path())?)))
    }).fold(vec![], |mut acc, r| {
        match r {
            Ok(None) => (),
            Ok(Some(v)) => acc.push(v),
            Err(e) => eprintln!("Error while processing file: {:?}", e),
        }
        return acc;
    });

    return Ok(results);
}

// #[derive(Debug)]
// pub enum FileParseError<F: str::FromStr> {
//     IoError(io::Error),
//     ParseError(F::Err),
// }
//
// pub fn parse_lines<'a, P, F: 'a, I>(filename: P) -> I
//     where P: AsRef<Path>, F: str::FromStr, I: iter::FromIterator<&'a F>, <F as str::FromStr>::Err: Debug  {
//     //
//     // let lines = read_lines::<P>(filename);
//     // let result: Result<_, FileParseError<F>> = lines.map(|l| {
//     //     match l {
//     //         io::Result::Ok(s) => s.trim().parse().map_err(|e| ParseError(e)),
//     //         io::Result::Err(e) => Err(IoError(e))
//     //     }
//     // }).collect();
//     let result: &[F; 1] = &["5".parse().expect("")];
//
//     let result: slice::Iter<'a, F> = result.into_iter();
//     let result = I::from_iter(result);
//     return result;
// }
