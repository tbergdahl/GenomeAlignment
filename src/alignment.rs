use crate::cell::DPCell;
use crate::parse::Params;
use std::cmp::max;


pub enum AlignmentType
{
    //Local,
    Global
}

pub fn run_alignment(s1: &String, s2: &String, align_type: AlignmentType, params: Params) -> (String, String, i32)
{
    match align_type
    {
        AlignmentType::Global =>
        {
            global_align(s1, s2, params)
        }
        // AlignmentType::Local =>
        // {
        //     local_align(s1, s2, params)
        // }
    }
    
}

fn global_align(s1: &String, s2: &String, params: Params) -> (String, String, i32)
{
   let mut table = init_table(s1.len() + 1, s2.len() + 1, &params);

   //println!("s1 len: {}, s2 len: {}", s1.len(), s2.len());

    let s1_vec: Vec<char> = s1.chars().collect(); // strings can't be indexed, but vectors can
    let s2_vec: Vec<char> = s2.chars().collect();

    for i in 1..s1_vec.len() + 1
    {
        for j in 1..s2_vec.len() + 1
        {
            table[i][j].del_score = max
            (
                max(table[i - 1][j].del_score + params.g, table[i - 1][j].ins_score + params.h + params.g),
                table[i - 1][j].sub_score + params.h + params.g
            );
            
            table[i][j].ins_score = max
            (
                max(table[i][j - 1].sub_score + params.h + params.g, table[i][j - 1].del_score + params.h + params.g), 
                table[i][j - 1].ins_score + params.g
            );
            
            table[i][j].sub_score = max
            (
                max(table[i - 1][j - 1].del_score, table[i - 1][j - 1].ins_score), 
                table[i - 1][j - 1].sub_score
            ) 
                + cost_substitute(s1_vec[i - 1], s2_vec[j - 1], &params);
        }
    }

    backtrace_and_construct_sequences(&table, &params, &s1_vec, &s2_vec)
}

fn backtrace_and_construct_sequences(table: &Vec<Vec<DPCell>>,params: &Params,  s1: &Vec<char>, s2: &Vec<char>) -> (String, String, i32)
{
    let mut s1_aligned = String::new();
    let mut s2_aligned = String::new(); // build the aligned strings

    let mut i = s1.len();
    let mut j = s2.len();

    // find max cell at T(m, n)
    let mut max_cell_val = max(max(table[i][j].ins_score, table[i][j].del_score), table[i][j].sub_score);
    let score = max_cell_val;
    while i > 0 && j > 0
    {
        if max_cell_val == table[i][j].ins_score // if our max value came from computing from I direction
        {
            s2_aligned += s2[j - 1].to_string().as_str();
            s1_aligned += "-";

            j -= 1;
            // calculate max value for the cell we will be visiting next to be checked next iteration
            let next_cell = table[i][j].clone();
            if (next_cell.ins_score + params.g) == max_cell_val
            {
                max_cell_val = next_cell.ins_score;
            }
            else if (next_cell.del_score + params.g + params.h) == max_cell_val
            {
                max_cell_val = next_cell.del_score;
            }
            else 
            {
                max_cell_val = next_cell.sub_score;
            }    
        }
        else if max_cell_val == table[i][j].del_score
        {
            s1_aligned += s1[i - 1].to_string().as_str();
            s2_aligned += "-";
            i -= 1;

            // calculate max value for the cell we will be visiting next to be checked next iteration
            let next_cell = table[i][j].clone();
            if (next_cell.del_score + params.g) == max_cell_val
            {
                max_cell_val = next_cell.del_score;
            }
            else if (next_cell.ins_score + params.g + params.h) == max_cell_val
            {
                max_cell_val = next_cell.ins_score;
            }
            else 
            {
                max_cell_val = next_cell.sub_score;
            } 
        }
        else 
        {
            s1_aligned += s1[i - 1].to_string().as_str();
            s2_aligned += s2[j - 1].to_string().as_str();
            i -= 1;
            j -= 1;

            // calculate max value for the cell we will be visiting next to be checked next iteration
            let next_cell = table[i][j].clone();
            if (next_cell.sub_score + cost_substitute(s1[i], s2[j], params)) == max_cell_val
            {
                max_cell_val = next_cell.sub_score;
            }
            else if (next_cell.ins_score + cost_substitute(s1[i], s2[j], params)) == max_cell_val
            {
                max_cell_val = next_cell.ins_score;
            }
            else 
            {
                max_cell_val = next_cell.del_score;
            } 
        }
    }

    (s1_aligned, s2_aligned, score)
}


fn cost_substitute(c1: char, c2: char, params: &Params) -> i32
{
    if c1 == c2
    {
        params.match_bonus
    }
    else 
    {
        params.mismatch_penalty
    }
}


// fn local_align(s1: &String, s2: &String, params: Params) -> (String, String)
// {
//     let mut table = init_table(s1.len() + 1, s2.len() + 1, &params);
    
//     ("".to_string(), "".to_string())
// }

fn init_table(nrows: usize, ncols: usize, params: &Params) -> Vec<Vec<DPCell>>
{
   let mut table =  vec![vec![DPCell::default(); ncols]; nrows];

   for i in 0..nrows
   {
        table[i][0].del_score = params.h + ((i as i32) * params.g);
        //println!("{}", table[i][0].del_score);
   }

   for j in 0..ncols
   {
        table[0][j].ins_score = params.h + ((j as i32) * params.g);
        //println!("{}", table[0][j].ins_score);
   }
   //println!("Table Rows: {}, Table Cols: {}", nrows, ncols);
   table[0][0] = DPCell::default();
   table
}


