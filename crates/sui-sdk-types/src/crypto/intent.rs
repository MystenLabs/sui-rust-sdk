/// A Signing Intent
///
/// An intent is a compact struct serves as the domain separator for a message that a signature
/// commits to. It consists of three parts:
///     1. [enum IntentScope] (what the type of the message is)
///     2. [enum IntentVersion]
///     3. [enum AppId] (what application that the signature refers to).
///
/// The serialization of an Intent is a 3-byte array where each field is represented by a byte and
/// it is prepended onto a message before it is signed in Sui.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Intent {
    pub scope: IntentScope,
    pub version: IntentVersion,
    pub app_id: IntentAppId,
}

impl Intent {
    pub fn new(scope: IntentScope, version: IntentVersion, app_id: IntentAppId) -> Self {
        Self {
            scope,
            version,
            app_id,
        }
    }

    pub fn to_bytes(self) -> [u8; 3] {
        [self.scope as u8, self.version as u8, self.app_id as u8]
    }

    pub fn scope(self) -> IntentScope {
        self.scope
    }

    pub fn version(self) -> IntentVersion {
        self.version
    }

    pub fn app_id(self) -> IntentAppId {
        self.app_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum IntentScope {
    TransactionData = 0,         // Used for a user signature on a transaction data.
    TransactionEffects = 1,      // Used for an authority signature on transaction effects.
    CheckpointSummary = 2,       // Used for an authority signature on a checkpoint summary.
    PersonalMessage = 3,         // Used for a user signature on a personal message.
    SenderSignedTransaction = 4, // Used for an authority signature on a user signed transaction.
    ProofOfPossession = 5, // Used as a signature representing an authority's proof of possession of its authority protocol key.
    HeaderDigest = 6,      // Used for narwhal authority signature on header digest.
    BridgeEventUnused = 7, // for bridge purposes but it's currently not included in messages.
    ConsensusBlock = 8,    // Used for consensus authority signature on block's digest
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum IntentVersion {
    V0 = 0,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum IntentAppId {
    Sui = 0,
    Narwhal = 1,
    Consensus = 2,
}
