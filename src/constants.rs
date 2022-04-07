pub const PSKLEN: usize = 32;
pub const CIPHERKEYLEN: usize = 32;
/// The length of the message tag.
///
/// The encrypted size of a message is the plaintext's size plus `TAGLEN`.
pub const TAGLEN: usize = 16;

pub const MAXHASHLEN: usize = 64;
pub const MAXBLOCKLEN: usize = 128;
pub const MAXDHLEN: usize = 56;

/// Maximum size of an encrypted Noise message, including its tag.
///
/// A message's plaintext size cannot exceed `MAXMSGLEN - TAGLEN` bytes.
pub const MAXMSGLEN: usize = 65535;

#[cfg(feature = "hfs")]
pub const MAXKEMPUBLEN: usize = 4096;
#[cfg(feature = "hfs")]
pub const MAXKEMCTLEN: usize = 4096;
#[cfg(feature = "hfs")]
pub const MAXKEMSSLEN: usize = 32;
