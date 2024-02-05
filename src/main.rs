use std::time::Instant;


fn main() {
    // 20! seems to be the max for u64
    let input = 20;

    if input < 1 {
        println!("Please enter an input greater than one");
    }

    // calculate start time of the interative approach
    let iterstart = Instant::now();
    
    // perform the standard iterative approach
    let itfact = iterfactorial(&input)
        .expect("A really, really big number");
    println!("The factorial of {} is {}", &input, itfact);
    
    // gather post execution time
    let itercost = iterstart.elapsed();
    println!("Iterative approach: {} nanoseconds", itercost.subsec_nanos());

    // calculate start time of the functional approach
    let funcstart = Instant::now();
    
    // perform the functional approach
    let factor: u64 = 1;
    let funfact = funcfactorial(&input, factor)
        .expect("Something went very very wrong!");
    println!("The factorial of {} is {}", &input, funfact);
    
    // gather post execution time
    let funccost = funcstart.elapsed();
    println!("Functional approach: {} nanoseconds", funccost.subsec_nanos());
}

fn iterfactorial(limit: &u64) -> Result<u64, String> {
    let mut factor: u64 = 1;
    for i in (1..=*limit).rev() {
        // use checked_mul to assure we don't overflow
        factor = factor
            .checked_mul(i)
            .ok_or_else(|| "Integer Overflow".to_string())?;
    }
    Ok(factor)
}

fn funcfactorial(limit: &u64, mut result: u64) -> Result<u64, String> {
    if *limit > 0 {
        let result = factmultiplier(&limit, &mut result);
        let iter = limit - 1;
        funcfactorial(&iter, result?)
    } else {
        Ok(result)
    }
}

fn factmultiplier(iter: &u64, factorial: &mut u64) -> Result<u64, String> {
    // the check here is redundant since we already checked with iterfactorial
    // but since this is a speed test we want to make the two approaches
    // as close as possible
    Ok(factorial
        .checked_mul(*iter)
        .ok_or_else(|| "Integer Overflow".to_string())?)
}
