
#[derive(Clone)]
pub struct DPCell
{
    pub sub_score: i32, // Substitution score
    pub del_score: i32, // Deletion score
    pub ins_score: i32  // Insertion score
}

impl std::fmt::Debug for DPCell
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
        .field("D", &self.del_score)
        .field("S", &self.sub_score)
        .field("I", &self.ins_score)
        .finish()
        
    }
}

impl std::default::Default for DPCell
{
    fn default() -> Self
    {
        DPCell{ sub_score: 0, del_score: 0, ins_score: 0 }
    }
}
