pub type Ship = Vec<Vec<char>>;
#[derive(Clone, Debug)]
pub struct Command {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}
