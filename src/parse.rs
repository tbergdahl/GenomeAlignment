
//! Module to parse input files for PA1.
//! 
//! Parses configurations of match, mismatch, gap, and consecutive gap penalties.
//! Parses genome strands contained in the [.fasta] format.
//! 
//! [.fasta]: https://www.ncbi.nlm.nih.gov/genbank/fastaformat/
pub mod parse
{
    use core::result;
    use std::fs::File;
    use std::io::{self, BufRead, Error, ErrorKind};
    use std::path::Path;
    use confy::ConfyError;
    use serde::{Serialize, Deserialize};// for config file

    /// A configuration struct to specify params used in the Global and Local Alignment Algorithms.
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Params {
        // bonus for a match 
        pub match_bonus: i32, 

        // penalty for a mismatch
        pub mismatch_penalty: i32,

        // consecutive gap penalty - applied when there are more than 1 gaps in sequence
        pub h: i32, 
        
        // initial gap penalty - applied for every gap, regardless of its relation to others.
        pub g: i32, 
    }
    
    /// Extracts information from the file with the inputted name into a Param struct instance.
    pub fn get_config(filepath: &str) -> Params
    {
        let config: Result<Params, ConfyError> = confy::load_path(filepath);

        match config 
        {
            // success
            Ok(params) => 
            {
                params // return successfully created struct
            }
            Err(e) => // failure
            {
                println!("Error: {:?}", e);
                Params // return default bad params
                {
                    match_bonus: 0,
                    mismatch_penalty: 0,
                    h: 0, 
                    g: 0
                }
            }
        }
    }

    /*
        Function to extract DNA sequences in the .fasta file format.

        Return: Collection of each sequence, as a String.
    */
    pub fn extract_sequences<P>(filename: P) -> Result<Vec<String>, std::io::Error> where P: AsRef<Path>,
    {
        if let Ok(lines) = split_lines(filename)
        {
            let mut sequences: Vec<String> = Vec::new();
            for line in lines
            {
                match line
                {
                    Ok(s) =>
                    {
                        sequences.push(s);
                    }
                    Err(e) =>
                    {
                        return Err(e);
                    }
                };
            }
            
            Ok(flatten_into_sequences(&mut sequences))
        }
        else
        {
            Err(Error::new(ErrorKind::Other, "split_lines() failure."))
        }
    }


    /*
        Takes lines from a file and flattens them into DNA sequences, ignoring non-sequence lines.
    */
    fn flatten_into_sequences(lines: &mut Vec<String>) -> Vec<String>
    {
        let mut sequences: Vec<String> = Vec::new();
        let mut sequence: String = String::new();
        for line in lines.iter()
        {
            if (*line).contains(">") && !sequence.trim().is_empty()
            {
                sequences.push(sequence.clone()); // push is a move op, so move a clone of it
                sequence.clear();
            }
            else if !(*line).contains(">") && !line.trim().is_empty()
            {
                sequence.push_str(line.trim());
            }
        }
        sequences.push(sequence);
        sequences
    }

    fn split_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        // wrap with BufReader to reduce system calls, since we are reading the same file.
        Ok(io::BufReader::new(file).lines())
    }
}