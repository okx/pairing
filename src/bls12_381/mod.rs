use crate::arithmetic::BaseExt;
use ff::PrimeField;

pub use bls12_381::Scalar as Fr;
pub use bls12_381::Fp as Fq;
pub use bls12_381::G1Projective as G1;
pub use bls12_381::G2Projective as G2;
pub use bls12_381::{multi_miller_loop, pairing, Bls12, G1Affine, G2Affine, G2Prepared, Gt, MillerLoopResult};

impl BaseExt for Fr {
    const MODULUS: &'static str =
        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";

    fn from_bytes_wide(bytes: &[u8; 64]) -> Self {
        Fr::from_bytes_wide(bytes)
    }

    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let compressed = self.to_repr();
        writer.write_all(&compressed[..])
    }

    fn read<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut compressed = [0u8; 32];
        reader.read_exact(&mut compressed[..])?;
        Option::from(Self::from_repr(compressed)).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "invalid point encoding in proof")
        })
    }
}

impl BaseExt for Fq {
    const MODULUS: &'static str = "0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab";

    fn from_bytes_wide(_bytes: &[u8; 64]) -> Self {
        unimplemented!()
    }

    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut compressed = self.to_bytes();
        compressed.reverse();
        writer.write_all(&compressed[..])
    }

    fn read<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut compressed = [0u8; 48];
        reader.read_exact(&mut compressed[..])?;
        compressed.reverse();
        Option::from(Fq::from_bytes(&compressed)).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "invalid point encoding in proof")
        })
    }
}
