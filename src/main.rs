mod cell;  // to construct table
mod parse; // to parse file

use parse::parse::{get_config, Params, extract_sequences};

fn main() -> ()
{
    let params = get_config("params.config");

    println!("{:?}", params);

    match extract_sequences("input.fasta")
    {
        Ok(sequences) =>
        {
            for s in sequences.iter()
            {
                println!("Line: {}", s);
            }
        }
        Err(e) =>
        {
            println!("{:?}", e);
        }
    }
}