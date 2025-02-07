mod cell;  // to construct table
mod parse; // to parse file
mod alignment;

use alignment::{AlignmentType, run_alignment, Result};
use parse::{get_config, extract_sequences};



fn main() -> ()
{
    let params = get_config("params.config");

    println!("{:?}", params);
    match extract_sequences("input.fasta")
    {
        Ok(sequences) =>
        {
            println!("S1: {:?}", sequences[0].clone());
            println!("S2: {:?}", sequences[1].clone());

            println!("Local: \n\n");
            let mut stats = run_alignment(sequences[0].clone(), sequences[1].clone(), AlignmentType::Local, &params);
            print_stats(&stats);

            println!("\nGlobal: \n");
            stats = run_alignment(sequences[0].clone(), sequences[1].clone(), AlignmentType::Global, &params);
            print_stats(&stats);
        }
        Err(e) =>
        {
            println!("{:?}", e);
        }
    }
}

fn print_stats(stats: &Result)
{
    let s1 = stats.s1.chars().rev().collect::<Vec<char>>();
            
    let s2 =  stats.s2.chars().rev().collect::<Vec<char>>();


    println!("Score: {:?}", stats.score);
    println!("# of Matches: {:?}", stats.match_count);
    println!("# of Mismatches: {:?}", stats.mismatch_count);
    println!("# of Gaps: {:?}", stats.gap_count);

    let mut middle_format_line: Vec<char> = Vec::new();
    if stats.s1.len() != stats.s2.len()
    {
        panic!("Something went horribly wrong");
    }
    else 
    {
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

        println!("{:?}", s1.iter().collect::<String>());
        println!("{:?}", middle_format_line.iter().collect::<String>());
        println!("{:?}", s2.iter().collect::<String>());

    }
}