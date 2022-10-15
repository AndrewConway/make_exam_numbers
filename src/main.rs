// This program is Copyright 2022 Andrew Conway and licensed under the GPL:
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.


use std::fs::File;
use std::io::{BufRead, Write};
use std::num::ParseIntError;
use std::ops::Range;
use std::path::PathBuf;
use std::str::FromStr;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use clap::Parser;

/// Program to produce a set of randomish exam numbers such that no two exam numbers are very similar.
///
/// Similar means that number of characters that need to be different between any pair of exam numbers
/// (the Hamming distance) is at least some specified number, like 3.
#[derive(Parser, Debug)]
#[clap(author, about)]
struct Parameters {
    /// An optional random seed (64 bit unsigned integer). This can be used to make a reproducible list.
    #[clap(long, value_parser)]
    seed: Option<u64>,
    /// The minimum number of characters different any code is allowed to be from
    /// any other code.
    #[clap(value_parser)]
    min_hamming_distance : usize,
    /// The number of digits in the code
    #[clap(value_parser)]
    digits : usize,

    /// Existing numbers that you want to avoid
    ///
    /// This is typically used when you used this program to create some numbers, and then decided you want some more,
    /// and want to avoid the old numbers.
    #[clap(long,value_parser)]
    existing : Vec<PathBuf>,

    /// How many codes you want, possibly with prefixes.
    ///
    /// A simple number means that many codes, with no prefix. The results will be stored in a file called `prefix_.txt`
    ///
    /// A value of the form "AB3:78" means get 78 codes, each with the prefix "AB3", stored in a file called `prefix_AB3.txt`.
    ///
    /// Multiple values would be allowed, so "A:500 B:200" means get 500 codes starting with "A" (stored in `prefix_A.txt`) and 200 starting with "B" (stored in `prefix_B.txt`).
    #[clap(value_parser)]
    prefixes : Vec<WantedPrefix>,
}

#[derive(Clone,Debug)]
struct WantedPrefix {
    prefix : String,
    number : usize,
}

impl FromStr for WantedPrefix {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((prefix,number)) = s.split_once(":") {
            let number : usize = number.parse()?;
            Ok(WantedPrefix{prefix:prefix.to_string(),number})
        } else {
            let number : usize = s.parse()?;
            Ok(WantedPrefix{prefix:"".to_string(),number})
        }
    }
}
/*
fn default_wanted() -> Parameters {
    Parameters{ min_hamming_distance: 3, prefixes: vec!["S0:600".parse().unwrap(),"P0:250".parse().unwrap(),"S1:100".parse().unwrap(),"P1:50".parse().unwrap(),], digits:6 }
}*/


fn main() -> std::io::Result<()> {
    let args : Parameters = Parameters::parse();
    let prng = if let Some(seed) = args.seed { rand_chacha::ChaCha8Rng::seed_from_u64(seed) } else { rand_chacha::ChaCha8Rng::from_entropy() };
    let upper_end_of_range = (10u64).pow(args.digits as u32);
    let mut generator = GenerateCodes {
        prng,
        range: 0..upper_end_of_range,
        num_digits: args.digits,
        used: vec![]
    };
    for path in &args.existing {
        let start_count = generator.used.len();
        let f = File::open(path)?;
        for line in std::io::BufReader::new(f).lines() {
            generator.used.push(line?);
        }
        println!("Read file {} containing {} entries",path.to_string_lossy(),generator.used.len()-start_count);
    }
    for p in &args.prefixes {
        println!("Processing prefix {} trying to find {}.",p.prefix,p.number);
        let mut file = File::create(format!("prefix_{}.txt",p.prefix))?;
        for i in 0..p.number {
            let code = generator.new_code(&p.prefix,args.min_hamming_distance);
            writeln!(file,"{}",code)?;
            println!("Found {} of {}",i+1,p.number)
        }
    }
    println!("All finished!");
    Ok(())
}

struct GenerateCodes {
    prng : ChaCha8Rng,
    range : Range<u64>,
    num_digits : usize,
    used : Vec<String>,
}

impl GenerateCodes {
    fn generate_candidate(&mut self,prefix:&str) -> String {
        let digits = self.prng.gen_range(self.range.clone());
        format!("{}{:02$}",prefix,digits,self.num_digits)
    }
    fn ok(&self,candidate:&str,min_hamming_distance:usize) -> bool {
        let hamming = |s:&String| s.chars().zip(candidate.chars()).filter(|(a,b)|a!=b).count()>=min_hamming_distance;
        self.used.iter().all(hamming)
    }

    fn new_code(&mut self,prefix:&str,min_hamming_distance:usize) -> String {
        let mut candidate = self.generate_candidate(prefix);
        while !self.ok(&candidate,min_hamming_distance) {
            print!(".");
            candidate = self.generate_candidate(prefix);
        }
        self.used.push(candidate.clone());
        candidate
    }
}


