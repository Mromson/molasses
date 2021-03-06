use crate::crypto::sig::{SigPublicKey, SignatureScheme};

// TODO: Decide whether we check the size on the lower end while (de)serializing

/// A `Roster`, as it appears in a `GroupState`, is a list of optional `Credential`s
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Roster(pub(crate) Vec<Option<Credential>>);

impl Roster {
    /// Truncates the roster to the last non-blank entry. If the roster is all blank, this will
    /// clear the roster.
    pub(crate) fn truncate_to_last_nonblank(&mut self) {
        // Truncate the roster to the last non-None credential
        let mut last_nonempty_roster_entry = None;
        for (i, entry) in self.0.iter().rev().enumerate() {
            if entry.is_some() {
                last_nonempty_roster_entry = Some(i);
            }
        }
        match last_nonempty_roster_entry {
            // If there are no nonempty entries in the roster, clear it
            None => self.0.clear(),
            Some(i) => {
                // This can't fail, because i is an index
                let num_elements_to_retain = i + 1;
                self.0.truncate(num_elements_to_retain)
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

// opaque cert_data<1..2^24-1>;
/// A bunch of bytes representing an X.509 certificate. This currently doesn't do anything.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "X509CertData__bound_u24")]
pub(crate) struct X509CertData(Vec<u8>);

// opaque identity<0..2^16-1>;
/// A bytestring that should uniquely identify the user in the Group
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "Identity__bound_u16")]
pub(crate) struct Identity(pub(crate) Vec<u8>);

/// Defines a simple user credential
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct BasicCredential {
    /// This is a user ID
    pub(crate) identity: Identity,

    /// The member's preferred signature scheme
    pub(crate) signature_scheme: &'static dyn SignatureScheme,

    /// The member's public key under said signature scheme
    pub(crate) public_key: SigPublicKey,
}

/// A user credential, as defined in section 5.6 of the MLS spec
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "Credential__enum_u8")]
pub(crate) enum Credential {
    Basic(BasicCredential),
    X509(X509CertData),
}

impl Credential {
    pub(crate) fn get_public_key(&self) -> &SigPublicKey {
        match self {
            Credential::Basic(ref basic) => &basic.public_key,
            Credential::X509(_) => unimplemented!(),
        }
    }

    pub(crate) fn get_signature_scheme(&self) -> &'static dyn SignatureScheme {
        match self {
            Credential::Basic(ref basic) => basic.signature_scheme,
            Credential::X509(_) => unimplemented!(),
        }
    }

    pub(crate) fn get_identity(&self) -> &Identity {
        match self {
            Credential::Basic(ref basic) => &basic.identity,
            Credential::X509(_) => unimplemented!(),
        }
    }
}
