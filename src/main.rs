mod cell;  // to construct table
mod parse; // to parse file

use confy::ConfyError;
use parse::parse::{get_config, Params};

fn main() -> ()
{
    let _ = confy::store_path("params.config", Params
    {
        match_bonus: 0,
        mismatch_penalty: 0,
        g: 69,
        h: 32
    }).expect("error");


    let params = get_config("params.config");

    println!("{:?}", params);
}