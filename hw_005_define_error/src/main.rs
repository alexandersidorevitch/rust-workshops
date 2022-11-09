use std::ops::{Add, Div};

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct WrappedU8(u8);

// write here something
impl Add for WrappedU8 {
    type Output = Result<WrappedU8>;

    fn add(self, other: Self) -> Self::Output {
        let (answer, is_overflow) = self.0.overflowing_add(other.0);
        if is_overflow {
            Err(anyhow!("Ты че-то попутал"))
        } else {
            Ok(Self { 0: answer })
        }
    }
}

impl Div for WrappedU8 {
    type Output = Result<WrappedU8>;

    fn div(self, other: Self) -> Self::Output {
        match self.0.checked_div_euclid(other.0) {
            Some(value) => Ok(Self {0: value}),
            None => Err(anyhow!("Бесконечность не предел"))
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    println!("Start");

    // Step 1
    let x = WrappedU8(1) + WrappedU8(1);
    let x = x?;

    println!("1 + 1 = {}", x.0);

    // Step 2
    let x = WrappedU8(255) + WrappedU8(1);
    let x = x.unwrap_err();
    println!("255 + 1 = {}", x);

    // Step 3
    let x = WrappedU8(9) / WrappedU8(3);
    let x = x?;
    println!("9 / 3 = {}", x.0);

    // // Step 4
    let x = WrappedU8(255) / WrappedU8(0);
    let x = x.unwrap_err();
    println!("255 / 0 = {}", x);

    println!("Finish");
    Ok(())
}
