mod cell;  // to construct table
mod parse; // to parse file
mod alignment;

use std::io::Write;

use alignment::{AlignmentType, run_alignment, Result};
use parse::{get_config, extract_sequences};



fn main() -> ()
{
    let params = get_config("params.config");
    match extract_sequences("input.fasta")
    {
        Ok(sequences) =>
        {

            let mut stats = Result::default();
            println!("{}: {}", sequences.1[0], sequences.0[0]);
            println!("{}: {}", sequences.1[1], sequences.0[1]);
            match params.align_type
            {
                0 =>
                {
                    println!("Performing Global Alignment...\n");
                    stats = run_alignment(sequences.0[0].clone(), sequences.0[1].clone(), AlignmentType::Global, &params);
                }
                1 =>
                {
                    println!("Performing Local Alignment...\n");
                    stats = run_alignment(sequences.0[1].clone(), sequences.0[1].clone(), AlignmentType::Local, &params);
                }
                _ =>
                {
                    panic!("Invalid Alignment Type Parameter.");
                }
            }
            print_stats(stats, &params);
        }
        Err(e) =>
        {
            println!("{:?}", e);
        }
    }
}

fn print_stats(stats: Result, params: &parse::Params)
{
    let s1: Vec<char> = stats.s1.chars().collect();
    let s2: Vec<char> = stats.s2.chars().collect();
    let mut file = std::fs::File::create("output.txt").expect("Error opening output file.");

    let mut middle_format_line: Vec<char> = Vec::new();
    if stats.s1.len() != stats.s2.len()
    {
        panic!("Something went horribly wrong");
    }
    else 
    {
        let _ = file.write(format!("Score: {:?}\n", stats.score).as_bytes());
        let _ = file.write(format!("{}\n", params).as_bytes());
        let _ = file.write(format!("# of Matches: {:?}\n", stats.match_count).as_bytes());
        let _ = file.write(format!("# of Mismatches: {:?}\n", stats.mismatch_count).as_bytes());
        let _ = file.write(format!("# of Gaps: {:?}\n", stats.gap_count).as_bytes());


        for i in 0..stats.s1.len()
        {
            if s1[i] == s2[i]
            {
                middle_format_line.push('|');
            }
            else 
            {
                middle_format_line.push(' ');
            }
        }
        let num_chunks = s1.len() / 60; // Since all are the same size, we use one for reference
        let _ = file.write_all(b"\nAlignment:\n");
        for i in 0..num_chunks {
            let start = i * 60;
            let end = start + 60;
    
            let _ = file.write_all(&stats.s1[start..end].as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(&middle_format_line.iter().collect::<String>()[start..end].as_bytes());
            let _ = file.write_all(b"\n");
            let _ = file.write_all(&stats.s2[start..end].as_bytes());
            let _ = file.write_all(b"\n\n");
        }
    }
}