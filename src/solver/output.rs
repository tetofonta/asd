#[derive(Debug)]
pub struct Solution{
    pub kind: String,
    pub expanded_states: usize,
    pub opened_states: usize,
    pub path_info: SolutionPath
}

#[derive(Debug)]
pub struct SolutionPath{
    pub path: Vec<(usize, usize)>,
    pub weight: f64,
    pub time: usize,
    pub waits: usize
}