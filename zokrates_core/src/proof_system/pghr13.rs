extern crate libc;

use self::libc::{c_char, c_int, uint8_t};
use flat_absy::flat_variable::FlatVariable;
use proof_system::utils::{prepare_generate_proof, prepare_setup};
use proof_system::ProofSystem;

use field::Field;

pub struct PGHR13 {}

impl PGHR13 {
    pub fn new() -> Self {
        PGHR13 {}
    }
}

extern "C" {
    fn _pghr13_setup(
        A: *const uint8_t,
        B: *const uint8_t,
        C: *const uint8_t,
        A_len: c_int,
        B_len: c_int,
        C_len: c_int,
        constraints: c_int,
        variables: c_int,
        inputs: c_int,
        pk_path: *const c_char,
        vk_path: *const c_char,
    ) -> bool;

    fn _pghr13_generate_proof(
        pk_path: *const c_char,
        proof_path: *const c_char,
        public_inputs: *const uint8_t,
        public_inputs_length: c_int,
        private_inputs: *const uint8_t,
        private_inputs_length: c_int,
    ) -> bool;
}

impl ProofSystem for PGHR13 {
    fn setup<T: Field>(
        &self,
        variables: Vec<FlatVariable>,
        a: Vec<Vec<(usize, T)>>,
        b: Vec<Vec<(usize, T)>>,
        c: Vec<Vec<(usize, T)>>,
        num_inputs: usize,
        pk_path: &str,
        vk_path: &str,
    ) -> bool {
        let (
            a_arr,
            b_arr,
            c_arr,
            a_vec,
            b_vec,
            c_vec,
            num_constraints,
            num_variables,
            num_inputs,
            pk_path_cstring,
            vk_path_cstring,
        ) = prepare_setup(variables, a, b, c, num_inputs, pk_path, vk_path);

        unsafe {
            _pghr13_setup(
                a_arr.as_ptr(),
                b_arr.as_ptr(),
                c_arr.as_ptr(),
                a_vec.len() as i32,
                b_vec.len() as i32,
                c_vec.len() as i32,
                num_constraints as i32,
                num_variables as i32,
                num_inputs as i32,
                pk_path_cstring.as_ptr(),
                vk_path_cstring.as_ptr(),
            )
        }
    }

    fn generate_proof<T: Field>(
        &self,
        pk_path: &str,
        proof_path: &str,
        public_inputs: Vec<T>,
        private_inputs: Vec<T>,
    ) -> bool {
        let (
            pk_path_cstring,
            proof_path_cstring,
            public_inputs_arr,
            public_inputs_length,
            private_inputs_arr,
            private_inputs_length,
        ) = prepare_generate_proof(pk_path, proof_path, public_inputs, private_inputs);

        unsafe {
            _pghr13_generate_proof(
                pk_path_cstring.as_ptr(),
                proof_path_cstring.as_ptr(),
                public_inputs_arr[0].as_ptr(),
                public_inputs_length as i32,
                private_inputs_arr[0].as_ptr(),
                private_inputs_length as i32,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_pghr13 {
        use super::*;
        use field::FieldPrime;

        // #[test]
        // fn setup() {
        // 	PGHR13::setup::<FieldPrime>(vec![], vec![], vec![], vec![], 1, "", "");
        // }

        // #[test]
        // fn generate_proof() {
        // 	PGHR13::generate_proof::<FieldPrime>("", "", vec![], vec![]);
        // }
    }
}
