# make_exam_numbers

This is a short program to make a list of effectively random numbers to assign
codes to candidates for exams to reduce the effect of biases upon markers.

In order to reduce the chances of confusion between numbers, the program makes
sure that no two numbers are too similar. The *Hamming distance*
between two codes is the number of differing digits at a particular place
in the number. For instance `1234567` and `1204507` have a Hamming distance of 2
because there are 2 digit substitutions needed to convert one to the other.
You specify the minimum Hamming distance allowable between any two codes (3 is pretty good).

You may also specify prefixes to be in front of the numbers. This might be
if there are different categories of the examination (e.g. individuals or groups,
senior or junior paper, etc.). These prefixes count towards the Hamming distance.

# How to compile the program.

This program is written in [Rust](https://www.rust-lang.org/). Install Rust (latest stable version
recommended), then run, in this directory,
```bash
cargo build --release
```

This will create a binary program `make_exam_numbers` in the `target/release` directory.

# How to run the program.

The program takes two compulsary arguments - the minimum Hamming distance, and the number of
desired digits - then a list of (optional) prefixes and associated desired numbers of codes. 

To run without prefixes, and just get 100 codes with 5 digits and a minimum Hamming distance of 3,
run
```bash
./target/release/make_exam_numbers 3 5 100
```
This will create a 100 line file `prefix_.txt` containing a list of the codes.

As a more complex example, the following command:
```bash
3 6 S0:1000 P0:400 S1:500 P1:200
```
will make codes with 6 digits (not counting the prefixes), with 1000 codes starting with `S0`
in the file `prefix_S0.txt`, 400 starting with `P0` in the file `prefix_P0.txt`, 500 starting
with `S1` and 200 starting with `P1`.

A more detailed explanation is printed by
```bash
./target/release/make_exam_numbers --help
```

# Impossible or unsolvable requests

It is possible to request something impossible. For instance, if you request 
5 digit codes with a minimum Hamming distance of 6, it is impossible to get more than
one code. Even with a minimum Hamming distance of 1, with 5 digit codes it is
impossible to get more then 100000 codes.

Furthermore, the program will not do a perfect job of maximizing the total possible
codes - it just keeps making new codes until it finds one that works. If you are getting
close to the maximimum possible number of codes, the program will slow down. This
can be observed by the text printed on the screen - each period is a failed attempt.

It is trivial to find 100 codes with 5 digits and a minimum Hamming distance of 3, will
take a second or so to find 530, and starts really struggling to get each one over 550.
If you find the program can't find enough, try adding another digit.

## Copyright

This program is Copyright 2022 Andrew Conway and licensed under the GPL:

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 





