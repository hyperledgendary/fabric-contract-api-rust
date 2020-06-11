/*
 * SPDX-License-Identifier: Apache-2.0
 */

/// Represents the identity that is requesting this transaction
/// 
/// 
///  
pub struct ClientIdentity {

}

impl ClientIdentity {

    fn new() -> ClientIdentity {
        ClientIdentity{

        }
    }

    /// The String id associated with this identiy
    pub fn get_id(&self) -> String {
        todo!("get_id");
    }

    /// The MSP Identifier of this identity
    /// 
    pub fn get_mspid(&self) -> String {
        todo!("get_mspid")
    }

    /// Returns the value of the client's attribute named `attrName`.
    /// 
    pub fn get_attribute(&self, attrName: String) -> Option<String> {
        todo!("get_attributed")
    }

    // GetX509Certificate returns the X509 certificate associated with the client,
    // or nil if it was not identified by an X509 certificate.
    // GetX509Certificate() (*x509.Certificate, error)

}