use color_eyre::Result;
use wasmer::{Instance, Value};

use super::{Circom, CircomBase};

#[cfg(feature = "circom-2")]
use super::Circom2;

#[derive(Clone, Debug)]
pub struct Wasm(Instance);

impl Circom for Wasm {
    fn get_fr_len(&self) -> Result<u32> {
        self.invoke_func_u32("getFrLen", &[])
    }

    fn get_ptr_raw_prime(&self) -> Result<u32> {
        self.invoke_func_u32("getPRawPrime", &[])
    }
}

#[cfg(feature = "circom-2")]
impl Circom2 for Wasm {
    fn get_field_num_len32(&self) -> Result<u32> {
        self.invoke_func_u32("getFieldNumLen32", &[])
    }

    fn get_raw_prime(&self) -> Result<()> {
        self.invoke_func("getRawPrime", &[])
    }

    fn read_shared_rw_memory(&self, i: u32) -> Result<u32> {
        self.invoke_func_u32("readSharedRWMemory", &[i])
    }

    fn write_shared_rw_memory(&self, i: u32, v: u32) -> Result<()> {
        self.invoke_func("writeSharedRWMemory", &[i, v])
    }

    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32) -> Result<()> {
        self.invoke_func("setInputSignal", &[hmsb, hlsb, pos])
    }

    fn get_witness(&self, i: u32) -> Result<()> {
        self.invoke_func("getWitness", &[i])
    }

    fn get_witness_size(&self) -> Result<u32> {
        self.invoke_func_u32("getWitnessSize", &[])
    }
}

impl CircomBase for Wasm {
    fn init(&self, sanity_check: bool) -> Result<()> {
        self.invoke_func("init", &[sanity_check as u32])
    }

    fn get_ptr_witness_buffer(&self) -> Result<u32> {
        self.invoke_func_u32("getWitnessBuffer", &[])
    }

    fn get_ptr_witness(&self, w: u32) -> Result<u32> {
        self.invoke_func_u32("getPWitness", &[w])
    }

    fn get_n_vars(&self) -> Result<u32> {
        self.invoke_func_u32("getNVars", &[])
    }

    fn get_signal_offset32(
        &self,
        p_sig_offset: u32,
        component: u32,
        hash_msb: u32,
        hash_lsb: u32,
    ) -> Result<()> {
        self.invoke_func(
            "getSignalOffset32",
            &[p_sig_offset, component, hash_msb, hash_lsb],
        )
    }

    fn set_signal(&self, c_idx: u32, component: u32, signal: u32, p_val: u32) -> Result<()> {
        self.invoke_func("setSignal", &[c_idx, component, signal, p_val])
    }

    fn get_version(&self) -> Result<u32> {
        self.invoke_func_u32("getVersion", &[])
    }

    fn invoke_func_u32(&self, name: &str, args: &[u32]) -> Result<u32> {
        let result = self.0.exports.get_function(name)?.call(
            &args
                .iter()
                .map(|x| Value::from(x.to_owned()))
                .collect::<Vec<Value>>(),
        )?;
        Ok(result[0].unwrap_i32() as u32)
    }

    fn invoke_func(&self, name: &str, args: &[u32]) -> Result<()> {
        self.0.exports.get_function(name)?.call(
            &args
                .iter()
                .map(|x| Value::from(x.to_owned()))
                .collect::<Vec<Value>>(),
        )?;
        Ok(())
    }
}

impl Wasm {
    pub fn new(instance: Instance) -> Self {
        Self(instance)
    }
}
