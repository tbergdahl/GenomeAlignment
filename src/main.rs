mod cell;  // to construct table
mod parse; // to parse file
mod alignment;

use alignment::{AlignmentType, run_alignment};
use parse::{get_config, extract_sequences};

fn main() -> ()
{
    let params = get_config("params.config");

    println!("{:?}", params);
    match extract_sequences("input.fasta")
    {
        Ok(sequences) =>
        {
            let aligned = run_alignment(&sequences[0], &sequences[1], AlignmentType::Global, params);
            let s1 = aligned.0.chars().rev().collect::<Vec<char>>();
            
            let s2 =  aligned.1.chars().rev().collect::<Vec<char>>();
    

            println!("Score: {:?}", aligned.2);

            let mut middle_format_line: Vec<char> = Vec::new();
            if aligned.0.len() != aligned.1.len()
            {
                panic!("Something went horribly wrong");
            }
            else 
            {
                for i in 0..aligned.0.len()
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
        Err(e) =>
        {
            println!("{:?}", e);
        }
    }
}