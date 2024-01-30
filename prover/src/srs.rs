use std::fs::File;
use zstd::stream::Decoder;

use crate::types::*;
use crate::error::*;

pub use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

pub fn load_srs(file: &str) -> Result<SRS, ProofError> {
    let f = File::open(file)?;
    let mut dec = Decoder::new(&f)?;
    let srs = SRS::deserialize_compressed(&mut dec)?;
    Ok(srs)
}

pub mod test_srs {
    use super::*;
    use zstd::stream::Encoder;
    use ark_std::test_rng;
    /// This function should only be used for testing, as it is insescure:
    /// the SRS should be generated by a trusted setup ceremony.
    pub fn gen_test_srs(num_vars: usize) -> Result<SRS, ProofError> {
        let mut rng = test_rng();
        PC::setup(num_vars, b"test_srs", &mut rng).map_err(|_| ProofError::SRSSamplingError)
    }

    pub fn save_srs(srs: SRS, file: &str) -> Result<(), ProofError> {
        let f = File::create(file)?;
        let mut enc = Encoder::new(&f, 0)?;
        srs.serialize_compressed(&mut enc)?;
        enc.finish()?;
        f.sync_all()?;
        Ok(())
    }

    pub fn gen_test_srs_to_file(poly_length: usize, file: &str) -> Result<(), ProofError> {
        let srs = gen_test_srs(poly_length)?;
        save_srs(srs, file)
    }
}
