

pub mod parse
{
    use confy::ConfyError;
    use serde::{Serialize, Deserialize};// for config file

    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct Params {
        pub match_bonus: i32,
        pub mismatch_penalty: i32,
        pub h: i32,
        pub g: i32
    }
    

    pub fn get_config(filepath: &str) -> Params
    {
        let config: Result<Params, ConfyError> = confy::load_path(filepath);

        match config 
        {
            Ok(params) => // success
            {
                params
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
}