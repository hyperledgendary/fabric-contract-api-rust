/*
 * SPDX-License-Identifier: Apache-2.0
 */

pub enum ROLE {
    MEMBER,
    PEER,
    ADMIN,
    CLIENT,
}


pub enum Expression {
    AND(Box<Expression>),
    OR(Box<Expression>),
    ONEOF(Box<Expression>),
    Principal(String,ROLE),
}

pub struct StateBasedEndorsement {

}

impl StateBasedEndorsement {
    pub fn build(expr: &[&Expression]) -> Self {
        StateBasedEndorsement {

        }
    }
}