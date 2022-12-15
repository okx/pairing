use crate::arithmetic::BaseExt;
use crate::arithmetic::Coordinates;
use crate::arithmetic::CurveAffine;
use crate::arithmetic::CurveExt;
use crate::arithmetic::FieldExt;
use crate::arithmetic::Group;
use ff::PrimeField;

pub use bls12_381::Fp as Fq;
pub use bls12_381::Fp2;
pub use bls12_381::G1Projective as G1;
pub use bls12_381::G2Projective as G2;
pub use bls12_381::Scalar as Fr;
pub use bls12_381::{
    multi_miller_loop, pairing, Bls12, G1Affine, G2Affine, G2Prepared, Gt, MillerLoopResult,
};
use subtle::Choice;
use subtle::CtOption;

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

impl Group for Fr {
    type Scalar = Fr;

    fn group_zero() -> Self {
        Fr::zero()
    }

    fn group_add(&mut self, rhs: &Self) {
        *self = &*self + rhs
    }

    fn group_sub(&mut self, rhs: &Self) {
        *self = &*self - rhs
    }

    fn group_scale(&mut self, by: &Self::Scalar) {
        *self = &*self * by
    }
}

impl FieldExt for Fr {
    //0x39f6d3a994cebea4199cec0404d0ec02a9ded2017fff2dff7fffffff80000001
    const TWO_INV: Self = unimplemented!();
    /*
    Fr::from_bytes([
        [0x7fffffff80000001,
        0xa9ded2017fff2dff,
        0x199cec0404d0ec02,
        0x39f6d3a994cebea4,
        ].
    ]); */

    //0x0538a6f66e19c653ed4f2f74a35d01686f67d4a2b566f8330fb4d6e13cf19a78
    const ROOT_OF_UNITY_INV: Self = unimplemented!();
    /*Fr([
        0xfb4d6e13cf19a78,
        0x6f67d4a2b566f833,
        0xed4f2f74a35d0168,
        0x538a6f66e19c653
    ]);
    */

    const DELTA: Self = unimplemented!();

    const ZETA: Self = unimplemented!();

    fn from_u128(_v: u128) -> Self {
        todo!()
    }

    fn get_lower_128(&self) -> u128 {
        todo!()
    }
}

impl Group for G1 {
    type Scalar = Fr;

    fn group_zero() -> Self {
        G1::identity()
    }

    fn group_add(&mut self, rhs: &Self) {
        *self = &*self + rhs
    }

    fn group_sub(&mut self, rhs: &Self) {
        *self = &*self - rhs
    }

    fn group_scale(&mut self, by: &Self::Scalar) {
        *self = &*self * by
    }
}

impl CurveExt for G1 {
    type ScalarExt = Fr;

    type Base = Fq;

    type AffineExt = G1Affine;

    const CURVE_ID: &'static str = "bls12_381_g1";

    fn jacobian_coordinates(&self) -> (Self::Base, Self::Base, Self::Base) {
        todo!()
    }

    fn is_on_curve(&self) -> Choice {
        todo!()
    }

    fn b() -> Self::Base {
        todo!()
    }

    fn new_jacobian(_x: Self::Base, _y: Self::Base, _z: Self::Base) -> CtOption<Self> {
        todo!()
    }
}

impl CurveAffine for G1Affine {
    type ScalarExt = Fr;

    type Base = Fq;

    type CurveExt = G1;

    fn is_on_curve(&self) -> Choice {
        self.is_on_curve()
    }

    fn coordinates(&self) -> CtOption<Coordinates<Self>> {
        CtOption::new(
            Coordinates {
                x: self.x,
                y: self.y,
            },
            1u8.into(),
        )
    }

    fn from_xy(_x: Self::Base, _y: Self::Base) -> CtOption<Self> {
        unimplemented!()
    }

    fn b() -> Self::Base {
        let two = Fq::one() + Fq::one();
        two + two
    }

    fn get_endomorphism_base(_base: &Self) -> Self {
        todo!()
    }

    fn get_endomorphism_scalars(_k: &Self::ScalarExt) -> (u128, u128) {
        todo!()
    }

    fn batch_add<const COMPLETE: bool, const LOAD_POINTS: bool>(
        _points: &mut [Self],
        _output_indices: &[u32],
        _num_points: usize,
        _offset: usize,
        _bases: &[Self],
        _base_positions: &[u32],
    ) {
        todo!()
    }
}
