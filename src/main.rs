mod cell;  // to construct table
mod parse; // to parse file
mod alignment;

use std::env;
use std::io::Write;
use parse::Params;

use alignment::{AlignmentType, run_alignment, Result};
use parse::{get_config, extract_sequences};



fn main() -> ()
{
    let args: Vec<String> = env::args().collect();

    // default param config
    let mut params = Params {
        match_bonus: 1,
        mismatch_penalty: -2,
        h: -5,
        g: -2,
    };

    let align_int = args[2].parse::<i32>().unwrap();
    let alignment_type = match align_int
    {
        0 => AlignmentType::Global,
        1 => AlignmentType::Local,
        _ => panic!("Invalid Alignment Type!"),
    };

    // optional params config
    if args.len() > 3
    {
        params = get_config(&args[3]);
    }
    match extract_sequences(&args[1])
    {
        Ok(sequences) =>
        {
            let stats = run_alignment(sequences.0[0].clone(), sequences.0[1].clone(), alignment_type, &params);
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