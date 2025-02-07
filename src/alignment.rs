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
   let mut table = init_table(s1.len() + 1, s2.len() + 1, &params, false);

    calc_table(&mut table, s1, s2, params, false);

    backtrace(&table, &params, &s1, &s2, s1.len(), s2.len(), false)
}

fn local_align(s1: &String, s2: &String, params: &Params) -> Result
{
    let mut table = init_table(s1.len() + 1, s2.len() + 1, &params, true); 
    calc_table(&mut table, s1, s2, params, true);
 
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
    backtrace(&table, &params, s1, s2, i_max, j_max, true)
}

fn backtrace(table: &Vec<Vec<DPCell>>,params: &Params,  seq1: &String, seq2: &String, max_i: usize, max_j: usize, is_local: bool) -> Result
{
    let s1: Vec<char> = seq1.chars().collect();
    let s2: Vec<char> = seq2.chars().collect();

    let mut i = max_i;
    let mut j = max_j;

    let mut stats = Result::default();
    let mut max_cell_val = max
    (
        max(table[i][j].ins_score, table[i][j].del_score),
        table[i][j].sub_score
    );
    let mut score = max_cell_val;

    while i > 0 && j > 0
    {
        if is_local && max_cell_val == 0 {
            break; // Stop backtracking in local alignment when reaching zero
        }
        if max_cell_val == table[i][j].ins_score 
        {
            stats.s2.push(s2[j - 1]);
            stats.s1.push('-');
            stats.gap_count += 1;
            j -= 1;
            max_cell_val = compute_i_max(max_cell_val, params, &table[i][j]);
        } 
        else if max_cell_val == table[i][j].del_score 
        {
            stats.s1.push(s1[i - 1]);
            stats.s2.push('-');
            stats.gap_count += 1;
            i -= 1;
            max_cell_val = compute_d_max(max_cell_val, params, &table[i][j]);
        } 
        else 
        {
            stats.s1.push(s1[i - 1]);
            stats.s2.push(s2[j - 1]);
            i -= 1;
            j -= 1;
            max_cell_val = compute_s_max(max_cell_val, params, &table[i][j], s1[i], s2[j], &mut stats);
        }
    }

    if !is_local 
    {

        // handle the case where there are gaps needed in the very front
    if j == 0
    {
        score += params.h + ((i as i32) * params.g);
        while i > 0
        {   
            stats.s1 += s1[i - 1].to_string().as_str();
            stats.s2 += "-";
            stats.gap_count += 1;
            i -= 1;
        }
    }
    if i == 0
    {  
        score += params.h + ((j as i32) * params.g);
        while j > 0
        {
            stats.s2 += s2[j - 1].to_string().as_str();
            stats.s1 += "-";
            stats.gap_count += 1;
            j -= 1;
        }
    }
    }

    stats.score = score;
    stats.s1 = stats.s1.chars().rev().collect();
    stats.s2 = stats.s2.chars().rev().collect();
    stats
}


fn calc_table(table: &mut Vec<Vec<DPCell>>, s1: &str, s2: &str, params: &Params, is_local: bool)
{
    let s1_vec: Vec<char> = s1.chars().collect();
    let s2_vec: Vec<char> = s2.chars().collect();

    for i in 1..=s1_vec.len() 
    {
        for j in 1..=s2_vec.len() 
        {
            let del = max(
                max(table[i - 1][j].del_score + params.g, table[i - 1][j].ins_score + params.h + params.g),
                table[i - 1][j].sub_score + params.h + params.g
            );
            
            let ins = max(
                max(table[i][j - 1].sub_score + params.h + params.g, table[i][j - 1].del_score + params.h + params.g), 
                table[i][j - 1].ins_score + params.g
            );

            let sub = max(
                max(table[i - 1][j - 1].del_score, table[i - 1][j - 1].ins_score), 
                table[i - 1][j - 1].sub_score
            ) + cost_substitute(s1_vec[i - 1], s2_vec[j - 1], params, &mut Result::default());

            table[i][j].del_score = if is_local { max(0, del) } else { del };
            table[i][j].ins_score = if is_local { max(0, ins) } else { ins };
            table[i][j].sub_score = if is_local { max(0, sub) } else { sub };
        }
    }
}


fn compute_i_max(max_cell_val: i32, params: &Params, next_cell: &DPCell) -> i32
{
    if (next_cell.ins_score + params.g) == max_cell_val
    {
        next_cell.ins_score
    }
    else if (next_cell.del_score + params.g + params.h) == max_cell_val
    {
        next_cell.del_score
    }
    else 
    {
        next_cell.sub_score
    }    
}

fn compute_d_max(max_cell_val: i32, params: &Params, next_cell: &DPCell) -> i32
{
    if (next_cell.del_score + params.g) == max_cell_val
    {
        next_cell.del_score
    }
    else if (next_cell.ins_score + params.g + params.h) == max_cell_val
    {
        next_cell.ins_score
    }
    else 
    {
        next_cell.sub_score
    } 
}

fn compute_s_max(max_cell_val: i32, params: &Params, next_cell: &DPCell, c1: char, c2: char, stats: &mut Result) -> i32
{
    if (next_cell.sub_score + cost_substitute(c1, c2, params, stats)) == max_cell_val
    {
        next_cell.sub_score
    }
    else if (next_cell.ins_score + cost_substitute(c1, c2, params, stats)) == max_cell_val
    {
        next_cell.ins_score
    }
    else 
    {
        next_cell.del_score
    } 
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




fn init_table(nrows: usize, ncols: usize, params: &Params, is_local: bool) -> Vec<Vec<DPCell>>
{
    let mut table = vec![vec![DPCell::default(); ncols]; nrows];

    if !is_local 
    {
        for i in 0..nrows 
        {
            table[i][0].del_score = params.h + ((i as i32) * params.g);
        }
        for j in 0..ncols 
        {
            table[0][j].ins_score = params.h + ((j as i32) * params.g);
        }
    }

    table
}


