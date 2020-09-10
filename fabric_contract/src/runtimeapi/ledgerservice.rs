/*
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::runtimeapi::wapc::runtime_host_call;
use crate::{contract::LedgerError, ledgerapi::*};
use fabric_ledger_protos::{common_messages, ledger_messages};
use protobuf::{parse_from_bytes, Message};
// Added this to make the code less cumbersome
use Expression::*;
/// This module contains the APIs that the peer is logically
/// exposing. i.e. the fabric-protos-ledger
///
///
pub struct LedgerService {}

impl LedgerService {
    pub fn create_state(
        key: String,
        collection: String,
        data: Vec<u8>,
    ) -> Result<State, LedgerError> {
        // Create the protobuf request for 'create_state'
        // and populate
        let mut state = ledger_messages::State::new();
        state.set_key(key);
        state.set_value(data);
        let mut csr = ledger_messages::CreateStateRequest::new();

        csr.set_context(LedgerService::get_context()?);
        csr.set_state(state);

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        csr.set_collection(c);
        // need to get the ownership back again
        let state = csr.get_state();

        // create the buffer to send
        let buffer = csr.write_to_bytes().unwrap();

        // make the host call
        // note the response here is empty, so no requirement to handle it
        match runtime_host_call(
            "LedgerService".to_string(),
            "CreateState".to_string(),
            buffer,
        ) {
            Ok(_) => Ok(State::from((state,collection))),
            Err(e) => return Err(LedgerError::from(format!("Unable to create State"))),
        }
    }

    pub fn read_state(key: &String, collection: &String) -> Result<State, LedgerError> {
        // create the protobuf message and pass to waPC
        let mut rsr = ledger_messages::ReadStateRequest::new();

        rsr.set_context(LedgerService::get_context()?);
        rsr.set_state_key(key.clone());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        rsr.set_collection(c);

        let buffer = rsr.write_to_bytes().unwrap();

        // need to handle the response to the request
        let response_buffer: Vec<u8> =
            match runtime_host_call("LedgerService".to_string(), "ReadState".to_string(), buffer) {
                Ok(buffer) => buffer,
                Err(e) => {
                    return Err(LedgerError::from(format!("Unable to read State")));
                }
            };

        let response =
            parse_from_bytes::<ledger_messages::ReadStateResponse>(&response_buffer).unwrap();
        let state = response.get_state();
        Ok(State::from((state,collection.clone())))
    }

    pub fn update_state(
        key: String,
        collection: String,
        data: Vec<u8>,
    ) -> Result<State, LedgerError> {
        // Create the protobuf request for 'create_state'
        // and populate
        let mut state = ledger_messages::State::new();
        state.set_key(key);
        state.set_value(data);
        let mut usr = ledger_messages::UpdateStateRequest::new();

        usr.set_context(LedgerService::get_context()?);
        usr.set_state(state);

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        usr.set_collection(c);

        // need to get the ownership back again
        let state = usr.get_state();

        // create the buffer to send
        let buffer = usr.write_to_bytes().unwrap();

        // make the host call
        // note the response here is empty, so no requirement to handle it
        match runtime_host_call(
            "LedgerService".to_string(),
            "UpdateState".to_string(),
            buffer,
        ) {
            Ok(_) => Ok(State::from((state,collection))),
            Err(r) => return Err(LedgerError::from(format!("Unable to delete State"))),
        }
    }

    pub fn delete_state(key: &String, collection: String) -> Result<(), LedgerError> {
        let mut dsr = ledger_messages::DeleteStateRequest::new();
        dsr.set_context(LedgerService::get_context()?);
        dsr.set_state_key(key.clone());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        dsr.set_collection(c);

        let buffer = dsr.write_to_bytes().unwrap();
        // make the host call
        // note the response here is empty, so no requirement to handle it
        match runtime_host_call(
            "LedgerService".to_string(),
            "DeleteState".to_string(),
            buffer,
        ) {
            Ok(_) => Ok(()),
            Err(e) => return Err(LedgerError::from(format!("Unable to delete State"))),
        }
    }

    pub fn exists_state(key: &str, collection: String) -> Result<bool, LedgerError> {
        // create the protobuf message and pass to waPC
        let mut esr = ledger_messages::ExistsStateRequest::new();
        esr.set_context(LedgerService::get_context()?);
        esr.set_state_key(key.to_string());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        esr.set_collection(c);

        let buffer = esr.write_to_bytes().unwrap();

        let response_buffer: Vec<u8> = match runtime_host_call(
            "LedgerService".to_string(),
            "ExistsState".to_string(),
            buffer,
        ) {
            Ok(buffer) => buffer,
            Err(e) => return Err(LedgerError::from(format!("Unable to exists_state"))),
        };
        let response =
            parse_from_bytes::<ledger_messages::ExistsStateResponse>(&response_buffer).unwrap();

        Ok(response.get_exists())
    }

    pub fn get_hash(key: &String, collection: &String) -> Result<Vec<u8>, LedgerError> {
        let mut ghr = ledger_messages::GetHashRequest::new();
        ghr.set_context(LedgerService::get_context()?);
        ghr.set_state_key(key.to_string());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        ghr.set_collection(c);

        let buffer = ghr.write_to_bytes().unwrap();

        let response_buffer: Vec<u8> =
            match runtime_host_call("LedgerService".to_string(), "GetHash".to_string(), buffer) {
                Ok(buffer) => buffer,
                Err(e) => return Err(LedgerError::from(format!("Unable to get_hash"))),
            };
        let response =
            parse_from_bytes::<ledger_messages::GetHashResponse>(&response_buffer).unwrap();

        Ok(response.get_hash().to_vec())
    }

    pub fn set_endorsement_policy(
        key: &String,
        collection: &String,
        sbe: StateBasedEndorsement,
    ) -> Result<(), LedgerError> {
        let mut epr = ledger_messages::SetEndorsementPolicyRequest::new();
        epr.set_context(LedgerService::get_context()?);
        epr.set_state_key(key.to_string());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        epr.set_collection(c);


        let mut ep = ledger_messages::EndorsementPolicy::new();
        let mut r = ledger_messages::EndorsementRule::new();
        LedgerService::match_expr(&sbe.root, &mut r);
        ep.set_rule(r);
        epr.set_policy(ep);


        let buffer = epr.write_to_bytes().unwrap();

        let response_buffer: Vec<u8> =
            match runtime_host_call("LedgerService".to_string(), "GetHash".to_string(), buffer) {
                Ok(buffer) => buffer,
                Err(e) => return Err(LedgerError::from(format!("Unable to get_hash"))),
            };
        let response =
            parse_from_bytes::<ledger_messages::SetEndorsementPolicyResponse>(&response_buffer)
                .unwrap();

        Ok(())
    }

    pub fn get_endorsement_policy(
        key: &String,
        collection: &String,
    ) -> Result<StateBasedEndorsement, LedgerError> {
        let mut ghr = ledger_messages::GetEndorsementPolicyRequest::new();
        ghr.set_context(LedgerService::get_context()?);
        ghr.set_state_key(key.to_string());

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.clone());
        ghr.set_collection(c);

        let buffer = ghr.write_to_bytes().unwrap();

        let response_buffer: Vec<u8> =
            match runtime_host_call("LedgerService".to_string(), "GetHash".to_string(), buffer) {
                Ok(buffer) => buffer,
                Err(e) => return Err(LedgerError::from(format!("Unable to get_hash"))),
            };
        let response =
            parse_from_bytes::<ledger_messages::GetEndorsementPolicyResponse>(&response_buffer)
                .unwrap();

        let ep = response.get_policy();
        let e = LedgerService::read_policy(&ep.get_rule());
        let sbe = StateBasedEndorsement::build(e);
        Ok(sbe)
    }

    pub fn get_range_states(
        start_key: &str,
        end_key: &str,
        collection: &str,
    ) -> Result<Vec<State>, LedgerError> {
        // create the protobuf message and pass to waPC
        let mut gsr = ledger_messages::GetStatesRequest::new();
        let mut key_range = ledger_messages::KeyRangeQuery::new();

        gsr.set_context(LedgerService::get_context()?);
        key_range.set_start_key(start_key.to_string());
        key_range.set_end_key(end_key.to_string());

        gsr.set_by_key_range(key_range);

        let mut c = ledger_messages::Collection::new();
        c.set_name(collection.to_string());
        gsr.set_collection(c);

        let buffer = gsr.write_to_bytes().unwrap();

        // need to handle the response to the request
        let response_buffer: Vec<u8> =
            match runtime_host_call("LedgerService".to_string(), "GetStates".to_string(), buffer) {
                Ok(buffer) => buffer,
                Err(e) => return Err(LedgerError::from(format!("Unable to read State"))),
            };

        let response =
            parse_from_bytes::<ledger_messages::GetStatesResponse>(&response_buffer).unwrap();

        let from_ledger_states = response.get_states();
        let state_vec: Vec<_> = from_ledger_states.iter().map(|s| State::from(s)).collect();

        Ok(state_vec)
    }

    // Gets the thread-local transaction context and creates the protobuf from it.
    fn get_context() -> Result<common_messages::TransactionContext, LedgerError> {
        let ctx = crate::runtimeapi::wapc::get_context();

        let mut tx_context = common_messages::TransactionContext::new();
        tx_context.set_transaction_id(ctx.get_id());
        tx_context.set_channel_id(ctx.get_channelid());
        Ok(tx_context)
    }

    /// Function to read the protobuf format and return the Rust Enum
    fn read_policy(r: &ledger_messages::EndorsementRule) -> Expression {
        let min = r.get_min_endorsements();
        let rules = r.get_rules();
        let principals = r.get_principals();

        let mut vec_rules = Vec::new();
        let rules_iter = rules.into_iter();
        for r in rules_iter {
            vec_rules.push(LedgerService::read_policy(&r));
        }

        let principals_iter = principals.into_iter();

        let mut vec_principals = Vec::new();
        for p in principals_iter {
            match p.get_role() {
                ledger_messages::EndorsementPrincipal_Role::MEMBER => {
                    vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::MEMBER));
                }
                ledger_messages::EndorsementPrincipal_Role::ADMIN => {
                    vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
                }
                ledger_messages::EndorsementPrincipal_Role::CLIENT => {
                    vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
                }
                ledger_messages::EndorsementPrincipal_Role::PEER => {
                    vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
                }
            }
        }

        let num_elements = (vec_principals.len() + vec_rules.len()) as i32;

        if vec_principals.len() == 1 && vec_rules.len() == 0 {
            return vec_principals[0].clone();
        } else if min == 1 {
            let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
            return Expression::OR(concatenated);
        } else if min == num_elements {
            let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
            return Expression::AND(concatenated);
        } else {
            let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
            if min == 0 {
                return concatenated[0].clone();
            } else {
                return Expression::OUTOF(concatenated, min as usize);
            };
        };
    }

    /// Fn to read the Rust enum and produce the protobuf
    fn match_expr(expr: &Expression, rule: &mut ledger_messages::EndorsementRule) {
        match expr {
            AND(e) => {
                let mut r = ledger_messages::EndorsementRule::new();
                let repeated_rules: Vec<ledger_messages::EndorsementRule> = Vec::new();
                let e_iter = e.iter();
                for subexpre in e_iter {
                    LedgerService::match_expr(subexpre, &mut r);
                }
                let min_endoresemtns: usize = r.get_principals().len() + r.get_rules().len();
                r.set_min_endorsements(min_endoresemtns as i32);

                rule.mut_rules().push(r);
            }
            OR(e) => {
                let mut r = ledger_messages::EndorsementRule::new();
                r.set_min_endorsements(1); // OR so it is set to 1
                let e_iter = e.iter();
                for subexpre in e_iter {
                    LedgerService::match_expr(subexpre, &mut r);
                }
                rule.mut_rules().push(r);
            }
            OUTOF(e, i) => {
                let mut r = ledger_messages::EndorsementRule::new();
                let e_iter = e.iter();
                for subexpre in e_iter {
                    LedgerService::match_expr(subexpre, &mut r);
                }
                r.set_min_endorsements(*i as i32);
                rule.mut_rules().push(r);
            }
            Principal(name, role) => {
                let mut p = ledger_messages::EndorsementPrincipal::new();
                p.set_msp_id(name.clone());
                match role {
                    ROLE::MEMBER => p.set_role(ledger_messages::EndorsementPrincipal_Role::MEMBER),
                    ROLE::PEER => {
                        p.set_role(ledger_messages::EndorsementPrincipal_Role::PEER);
                    }
                    ROLE::ADMIN => p.set_role(ledger_messages::EndorsementPrincipal_Role::ADMIN),
                    ROLE::CLIENT => p.set_role(ledger_messages::EndorsementPrincipal_Role::CLIENT),
                };
                rule.mut_principals().push(p);
            }
        }
    }
}
