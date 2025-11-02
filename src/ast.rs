#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Action(Action),
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Qubit(QubitDecl),
    List(ListDecl),
    Instruction(InstructionDecl),
}

#[derive(Debug, Clone)]
pub struct QubitDecl {
    pub name: String,
    pub number: u32,
}

#[derive(Debug, Clone)]
pub struct ListDecl {
    pub name: String,
    pub items: Vec<ListItem>,
}

#[derive(Debug, Clone)]
pub struct InstructionDecl {
    pub name: String,
    pub pipeline: Vec<GateElement>,
}

#[derive(Debug, Clone)]
pub enum GateElement {
    Gate {
        name: String,
        args: Vec<ArgType>,
    },
    NamedPipe {
        name: String,
        reverse: bool,
    },
}

#[derive(Debug, Clone)]
pub enum ListItem {
    Name(String),
    Number(u32),
}

#[derive(Debug, Clone)]
pub enum ArgType {
    Name(String),
    Number(u32),
}

#[derive(Debug, Clone)]
pub struct Action {
    pub target: ActionTarget,
    pub index: Option<u32>,
    pub pipeline: Vec<GateElement>,
}

#[derive(Debug, Clone)]
pub enum ActionTarget {
    Name(String),
    Number(u32),
    List(Vec<ListItem>),
    All,
}
