// Generated by Molecule 0.7.2

use super::super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct CodeCellLockArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for CodeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for CodeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for CodeCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_arg", self.lock_arg())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for CodeCellLockArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        CodeCellLockArgs::new_unchecked(v.into())
    }
}
impl CodeCellLockArgs {
    pub const FIELD_COUNT: usize = 1;
    pub const FIELD_SIZES: [usize; 1] = [20];
    pub const TOTAL_SIZE: usize = 20;

    pub fn lock_arg(&self) -> PubKeyHash {
        PubKeyHash::new_unchecked(self.0.slice(0..20))
    }

    pub fn as_reader<'r>(&'r self) -> CodeCellLockArgsReader<'r> {
        CodeCellLockArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for CodeCellLockArgs {
    type Builder = CodeCellLockArgsBuilder;

    const NAME: &'static str = "CodeCellLockArgs";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CodeCellLockArgs(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CodeCellLockArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CodeCellLockArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder().lock_arg(self.lock_arg())
    }
}
#[derive(Clone, Copy)]
pub struct CodeCellLockArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for CodeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for CodeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for CodeCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "lock_arg", self.lock_arg())?;
        write!(f, " }}")
    }
}
impl<'r> CodeCellLockArgsReader<'r> {
    pub const FIELD_COUNT: usize = 1;
    pub const FIELD_SIZES: [usize; 1] = [20];
    pub const TOTAL_SIZE: usize = 20;

    pub fn lock_arg(&self) -> PubKeyHashReader<'r> {
        PubKeyHashReader::new_unchecked(&self.as_slice()[0..20])
    }
}
impl<'r> molecule::prelude::Reader<'r> for CodeCellLockArgsReader<'r> {
    type Entity = CodeCellLockArgs;

    const NAME: &'static str = "CodeCellLockArgsReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        CodeCellLockArgsReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct CodeCellLockArgsBuilder {
    pub(crate) lock_arg: PubKeyHash,
}
impl CodeCellLockArgsBuilder {
    pub const FIELD_COUNT: usize = 1;
    pub const FIELD_SIZES: [usize; 1] = [20];
    pub const TOTAL_SIZE: usize = 20;

    pub fn lock_arg(mut self, v: PubKeyHash) -> Self {
        self.lock_arg = v;
        self
    }
}
impl molecule::prelude::Builder for CodeCellLockArgsBuilder {
    type Entity = CodeCellLockArgs;

    const NAME: &'static str = "CodeCellLockArgsBuilder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(self.lock_arg.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        CodeCellLockArgs::new_unchecked(inner.into())
    }
}