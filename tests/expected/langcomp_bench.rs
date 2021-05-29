
//! ```cargo
//! [package]
//! edition = "2018"
//! [dependencies]
//!
//! ```

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::double_parens)] // https://github.com/adsharma/py2many/issues/17
#![allow(clippy::map_identity)]
#![allow(clippy::needless_return)]
#![allow(clippy::print_literal)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::redundant_static_lifetimes)] // https://github.com/adsharma/py2many/issues/266
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::useless_vec)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub fn test_python(iterations: i32) {
    let mut iteration: i32 = 0;
    let mut total: f64 = 0.0 as f64;
    let array_length: i32 = 1000;
    let array: Vec<i32> = (0..array_length).map(|i| i).collect::<Vec<_>>();
    println!("{} {}", "iterations", iterations);
    while iteration < iterations {
        let mut innerloop: i32 = 0;
        while innerloop < 100 {
            total += array[((iteration + innerloop) % array_length) as usize] as f64;
            innerloop += 1;
        }
        iteration += 1;
    }
    if total == (15150 as f64) {
        println!("{}", "OK");
    }
    drop(array);
}

pub fn main() -> Result<(), std::io::Error> {
    test_python(3);
    Ok(())
}
