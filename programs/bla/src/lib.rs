use anchor_lang::prelude::*;
use ark_bn254::Fr;
use num_bigint::BigUint;
use gnark_bn254_verifier::{verify, ProvingSystem};

declare_id!("2Z9iGJD2L5WUycxNzjvE7E1RNR5ExHsrpRggopBQZFrb");

#[program]
pub mod bla {
    use super::*;

    pub fn prove(ctx: Context<Initialize>, proof: Vec<u8>, vk: Vec<u8>, public_inputs: Vec<u8>) -> Result<()> {
        let committed_values_digest = Fr::from(BigUint::from_bytes_be(&public_inputs));
        
        // Move the verification to a separate function
        verify(&proof, &vk, &[committed_values_digest], ProvingSystem::Plonk);
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// // Separate function for verification
// #[inline(never)]
// fn verify_proof(proof: &[u8], vk: &[u8], committed_values_digest: Fr) -> Result<()> {
//     // Use Box to allocate large data structures on the heap
//     let boxed_proof = Box::new(proof.to_vec());
//     let boxed_vk = Box::new(vk.to_vec());
//     let boxed_inputs = Box::new([committed_values_digest]);
    
//     verify(&boxed_proof, &boxed_vk, &boxed_inputs, ProvingSystem::Plonk);

//     Ok(())
// }