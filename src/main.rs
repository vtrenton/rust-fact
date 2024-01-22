use std::{time::Instant, process};


fn main() {
    // 20! seems to be the max for u64
    let input = 20;

    // calculate start time of the interative approach
    let iterstart = Instant::now();
    // perform the standard iterative approach
    println!("The factorial of {} is {}", &input, iterfactorial(&input));
    // gather post execution time
    let itercost = iterstart.elapsed();
    println!("Iterative approach: {}", itercost.subsec_nanos());

    // calculate start time of the functional approach
    let funcstart = Instant::now();
    // perform the functional approach
    let factor: u64 = 1;
    println!("The factorial of {} is {}", &input, funcfactorial(&input, factor));
    // gather post execution time
    let funccost = funcstart.elapsed();
    println!("Functional approach: {}", funccost.subsec_nanos());
}

fn iterfactorial(limit: &u64) -> u64 {
    if *limit == 1 {
        return 1;
    } else {
        let mut factor: u64 = 1;
        for i in (1..=*limit).rev() {
            // use checked_mul to assure we don't overflow
            match factor.checked_mul(i) {
                Some(result) => factor = result,
                None => {
                    // catch integer overflow and exit cleanly
                    println!("A really, really, really big number");
                    process::exit(0);
                }
            }
        }
        return factor;
    }
}

fn funcfactorial(limit: &u64, mut result: u64) -> u64 {
    if *limit > 0 {
        let result = factmultiplier(&limit, &mut result);
        let iter = limit - 1;
        funcfactorial(&iter, result)
    } else {
        return result;
    }
}

fn factmultiplier(iter: &u64, factorial: &mut u64) -> u64 {
    match factorial.checked_mul(*iter) {
        Some(result) => *factorial = result,
        None => {
            // catch integer overflow and exit cleanly
            println!("A really, really, really big number");
            process::exit(0);
        }
    }
    return *factorial;
}
