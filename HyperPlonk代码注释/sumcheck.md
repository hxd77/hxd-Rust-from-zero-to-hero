# sumcheck

```rust
use crate::poly_iop::{
    errors::PolyIOPErrors,
    structs::{IOPProof, IOPProverState, IOPVerifierState},
    PolyIOP,
};
use arithmetic::{VPAuxInfo, VirtualPolynomial};
use ark_ff::PrimeField;
use ark_poly::DenseMultilinearExtension;
use ark_std::{end_timer, start_timer};
use std::{fmt::Debug, sync::Arc};
use transcript::IOPTranscript;

mod prover;
mod verifier;

```

