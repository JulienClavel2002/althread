use std::fmt::{self};

use pest::iterators::Pairs;

use crate::{
    ast::{
        display::{AstDisplay, Prefix},
        node::{InstructionBuilder, Node, NodeBuilder},
        statement::expression::Expression,
        token::{
            binary_assignment_operator::BinaryAssignmentOperator, identifier::Identifier,
            literal::Literal,
        },
    }, compiler::CompilerState, vm::instruction::{GlobalAssignmentControl, Instruction, InstructionType}, error::{AlthreadError, AlthreadResult, ErrorType}, parser::Rule
};

#[derive(Debug)]
pub struct BinaryAssignment {
    pub identifier: Node<Identifier>,
    pub operator: Node<BinaryAssignmentOperator>,
    pub value: Node<Expression>,
}

impl NodeBuilder for BinaryAssignment {
    fn build(mut pairs: Pairs<Rule>) -> AlthreadResult<Self> {
        let identifier = Node::build(pairs.next().unwrap())?;
        let operator = Node::build(pairs.next().unwrap())?;
        let value = Node::build(pairs.next().unwrap())?;

        Ok(Self {
            identifier,
            operator,
            value,
        })
    }
}



impl InstructionBuilder for BinaryAssignment {
    fn compile(&self, state: &mut CompilerState) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        instructions.append(&mut self.value.compile(state));

        instructions.push(Instruction {
            span: 0,
            control: InstructionType::GlobalAssignment(GlobalAssignmentControl{
                identifier: self.identifier.value.value.clone(),
                operator: self.operator.value.clone(),
            })
        });


        instructions
    }
}




impl AstDisplay for BinaryAssignment {
    fn ast_fmt(&self, f: &mut fmt::Formatter, prefix: &Prefix) -> fmt::Result {
        writeln!(f, "{}binary_assign", prefix)?;

        let prefix = prefix.add_branch();
        writeln!(f, "{}ident: {}", &prefix, self.identifier)?;
        writeln!(f, "{}op: {}", &prefix, self.operator)?;

        let prefix = prefix.switch();
        writeln!(f, "{}value:", &prefix)?;
        let prefix = prefix.add_leaf();
        self.value.ast_fmt(f, &prefix)?;
        Ok(())
    }
}
