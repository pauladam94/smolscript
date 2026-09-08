pub enum Instruction {}

pub type Label = usize;

pub enum JumpInstruction {
    Return,
}

pub struct Block {
    instructions: Vec<Instruction>,
    exit: JumpInstruction,
}

pub struct SSA {
    blocks: Vec<Block>,
}
