use crate::cell::DPCell;
use crate::parse::Params;
use std::cmp::max;


pub enum AlignmentType
{
    Local,
    Global
}

#[derive(Default)]
pub struct Result
{
    pub s1: String,
    pub s2: String,
    pub match_count: u32,
    pub gap_count: u32,
    pub mismatch_count: u32,
    pub score: i32
}

pub fn run_alignment(s1: String, s2: String, align_type: AlignmentType, params: &Params) -> Result
{
    match align_type
    {
        AlignmentType::Global =>
        {
            global_align(&s1, &s2, &params)
        }
        AlignmentType::Local =>
        {
            local_align(&s1, &s2, &params)
        }
    }
    
}

fn global_align(s1: &String, s2: &String, params: &Params) -> Result
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
                + cost_substitute(s1_vec[i - 1], s2_vec[j - 1], &params, &mut Result{..Default::default()});
        }
    }

    for row in &table
    {
        println!("{:?}", row);
    }

    backtrace_and_construct_sequences(&table, &params, &s1, &s2, s1.len(), s2.len())
}

fn backtrace_and_construct_sequences(table: &Vec<Vec<DPCell>>,params: &Params,  seq1: &String, seq2: &String, max_i: usize, max_j: usize) -> Result
{

    // create indexable strings to build up aligned strings
    let s1 = seq1.chars().collect::<Vec<char>>();
    let s2 = seq2.chars().collect::<Vec<char>>();

    let mut i = max_i;
    let mut j = max_j;

    let mut stats = Result::default();

    // find max cell at T(i, j)
    let mut max_cell_val = max(max(table[i][j].ins_score, table[i][j].del_score), table[i][j].sub_score);
    let score = max_cell_val;
    while i > 0 && j > 0
    {
        if max_cell_val == table[i][j].ins_score // if our max value came from computing from I direction
        {
            stats.s2 += s2[j - 1].to_string().as_str();
            stats.s1 += "-";
            stats.gap_count += 1;

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
            stats.s1 += s1[i - 1].to_string().as_str();
            i -= 1;
            stats.s2 += "-";
            stats.gap_count += 1;
            

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
            stats.s1 += s1[i - 1].to_string().as_str();
            stats.s2 += s2[j - 1].to_string().as_str();
            i -= 1;
            j -= 1;

            // calculate max value for the cell we will be visiting next to be checked next iteration
            let next_cell = table[i][j].clone();
            if (next_cell.sub_score + cost_substitute(s1[i], s2[j], params, &mut stats)) == max_cell_val
            {
                max_cell_val = next_cell.sub_score;
            }
            else if (next_cell.ins_score + cost_substitute(s1[i], s2[j], params, &mut stats)) == max_cell_val
            {
                max_cell_val = next_cell.ins_score;
            }
            else 
            {
                max_cell_val = next_cell.del_score;
            } 
        }
    }

    // handle the case where there are gaps needed in the very front
    while i > 0
    {
        i -= 1;
        stats.s1 += s1[i - 1].to_string().as_str();
        stats.s2 += "-";
    }
    while j > 0
    {
        stats.s2 += s2[j - 1].to_string().as_str();
        stats.s1 += "-";
        j -= 1;
    }

    stats.score = score;
    stats
}


fn backtrace_and_construct_sequences2(table: &Vec<Vec<DPCell>>,params: &Params,  seq1: &String, seq2: &String, max_i: usize, max_j: usize) -> Result
{

    // create indexable strings to build up aligned strings
    let s1 = seq1.chars().collect::<Vec<char>>();
    let s2 = seq2.chars().collect::<Vec<char>>();

    let mut i = max_i;
    let mut j = max_j;

    let mut stats = Result::default();

    // find max cell at T(i, j)
    let mut max_cell_val = max(max(table[i][j].ins_score, table[i][j].del_score), table[i][j].sub_score);
    let score = max_cell_val;
    while i > 0 && j > 0 && max_cell_val != 0
    {
        if max_cell_val == table[i][j].ins_score // if our max value came from computing from I direction
        {
            stats.s2 += s2[j - 1].to_string().as_str();
            stats.s1 += "-";
            stats.gap_count += 1;

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
            stats.s1 += s1[i - 1].to_string().as_str();
            stats.s2 += "-";
            stats.gap_count += 1;
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
            stats.s1 += s1[i - 1].to_string().as_str();
            stats.s2 += s2[j - 1].to_string().as_str();
            i -= 1;
            j -= 1;

            // calculate max value for the cell we will be visiting next to be checked next iteration
            let next_cell = table[i][j].clone();
            if (next_cell.sub_score + cost_substitute(s1[i], s2[j], params, &mut stats)) == max_cell_val
            {
                max_cell_val = next_cell.sub_score;
            }
            else if (next_cell.ins_score + cost_substitute(s1[i], s2[j], params, &mut stats)) == max_cell_val
            {
                max_cell_val = next_cell.ins_score;
            }
            else 
            {
                max_cell_val = next_cell.del_score;
            } 
        }
    }
    stats.score = score;
    stats
}


fn cost_substitute(c1: char, c2: char, params: &Params, stats: &mut Result) -> i32
{
    if c1 == c2
    {
        stats.match_count += 1;
        params.match_bonus
    }
    else 
    {
        stats.mismatch_count += 1;
        params.mismatch_penalty
    }
}


fn local_align(s1: &String, s2: &String, params: &Params) -> Result
{
    let mut table = init_table2(s1.len() + 1, s2.len() + 1, &params);
    let s1_vec: Vec<char> = s1.chars().collect(); // strings can't be indexed, but vectors can
    let s2_vec: Vec<char> = s2.chars().collect();

    for i in 1..s1_vec.len() + 1
    {
        for j in 1..s2_vec.len() + 1
        {
            table[i][j].del_score = max(0, max
            (
                max(table[i - 1][j].del_score + params.g, table[i - 1][j].ins_score + params.h + params.g),
                table[i - 1][j].sub_score + params.h + params.g
            ));
            
            table[i][j].ins_score = max(0, max
            (
                max(table[i][j - 1].sub_score + params.h + params.g, table[i][j - 1].del_score + params.h + params.g), 
                table[i][j - 1].ins_score + params.g
            ));
            
            table[i][j].sub_score = max(0, max
            (
                max(table[i - 1][j - 1].del_score, table[i - 1][j - 1].ins_score), 
                table[i - 1][j - 1].sub_score
            ) 
                + cost_substitute(s1_vec[i - 1], s2_vec[j - 1], &params, &mut Result{..Default::default()}));
        }
    }

    // find max value in the table and save position
    let mut cur_max = 0;
    let mut i_max = 0;
    let mut j_max = 0;
    for i in 1..s1.len() + 1
    {
        for j in 1..s2.len() + 1
        {
            let max = max(table[i][j].del_score, max(table[i][j].ins_score, table[i][j].sub_score));
            if  max > cur_max
            {
                cur_max = max;
                i_max = i;
                j_max = j;
            }
        }
    }
    // backtrace starting from that position
    backtrace_and_construct_sequences2(&table, &params, s1, s2, i_max, j_max)
}

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


fn init_table2(nrows: usize, ncols: usize, params: &Params) -> Vec<Vec<DPCell>>
{
   let mut table =  vec![vec![DPCell::default(); ncols]; nrows];
   table
}