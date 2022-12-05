// Advent of Code 2022 Number One
// Code by ThePrimeTime

use anyhow::Result;

// Step 1
// fn main() -> Result<()> {
//     let max = include_str!("../input.txt")
//         .split("\n\n")
//         .map(|x| {
//             return x
//                 .lines()
//                 .flat_map(|x| x.parse::<usize>())
//                 .sum::<usize>();
//         })
//         .max();

//     println!("Result: {:?}", max);

//     return Ok(());
// }


// Step 2
fn main() -> Result<()> {
    let mut max = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            return x
                .lines()
                .flat_map(|x| x.parse::<usize>())
                .sum::<usize>();
        })
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));



    println!("Result: {:?}", max
        .iter()
        .take(3)
        .sum::<usize>()
    );

    return Ok(());
}
