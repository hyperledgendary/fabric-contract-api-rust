/*
 * SPDX-License-Identifier: Apache-2.0
 */

/// The ROLE definition
#[derive(Debug, Clone)]
pub enum ROLE {
    MEMBER,
    PEER,
    ADMIN,
    CLIENT,
}

/// The Expressions - either AND, OR, OUTOF  or the actual Principal
#[derive(Debug, Clone)]
pub enum Expression {
    AND(Vec<Expression>),
    OR(Vec<Expression>),
    OUTOF(Vec<Expression>, usize),
    Principal(String, ROLE),
}

/// Struct to represent the Overal Endorsement
#[derive(Debug)]
pub struct StateBasedEndorsement {
    pub root: Expression,
}

impl StateBasedEndorsement {
    pub fn build(expr: Expression) -> Self {
        StateBasedEndorsement { root: expr }
    }
}