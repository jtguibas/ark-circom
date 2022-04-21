use color_eyre::Result;
use wasmi::{
    ExternVal, FuncRef, ImportsBuilder, Module, ModuleInstance, NopExternals, RuntimeValue, ModuleRef,
};

#[derive(Clone, Debug)]
pub struct Wasm(ModuleRef);

pub trait CircomBase {
    fn init(&self, sanity_check: bool) -> Result<()>;
    fn get_ptr_witness_buffer(&self) -> Result<u32>;
    fn get_ptr_witness(&self, w: u32) -> Result<u32>;
    fn get_n_vars(&self) -> Result<u32>;
    fn get_signal_offset32(
        &self,
        p_sig_offset: u32,
        component: u32,
        hash_msb: u32,
        hash_lsb: u32,
    ) -> Result<()>;
    fn set_signal(&self, c_idx: u32, component: u32, signal: u32, p_val: u32) -> Result<()>;
    fn get_u32(&self, name: &str) -> Result<u32>;
    // Only exists natively in Circom2, hardcoded for Circom
    fn get_version(&self) -> Result<u32>;
}

pub trait Circom {
    fn get_fr_len(&self) -> Result<u32>;
    fn get_ptr_raw_prime(&self) -> Result<u32>;
}

pub trait Circom2 {
    fn get_field_num_len32(&self) -> Result<u32>;
    fn get_raw_prime(&self) -> Result<()>;
    fn read_shared_rw_memory(&self, i: u32) -> Result<u32>;
    fn write_shared_rw_memory(&self, i: u32, v: u32) -> Result<()>;
    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32) -> Result<()>;
    fn get_witness(&self, i: u32) -> Result<()>;
    fn get_witness_size(&self) -> Result<u32>;
}

impl Circom for Wasm {
    fn get_fr_len(&self) -> Result<u32> {
        self.get_u32("getFrLen")
    }

    fn get_ptr_raw_prime(&self) -> Result<u32> {
        self.get_u32("getPRawPrime")
    }
}

#[cfg(feature = "circom-2")]
impl Circom2 for Wasm {
    fn get_field_num_len32(&self) -> Result<u32> {
        self.get_u32("getFieldNumLen32")
    }

    fn get_raw_prime(&self) -> Result<()> {
        let _ = self.0.invoke_export("getRawPrime", &[], &mut NopExternals);
        Ok(())
    }

    fn read_shared_rw_memory(&self, i: u32) -> Result<u32> {
        let result = self.0.invoke_export("readSharedRWMemory", &[i.into()], &mut NopExternals)?;
        match result.unwrap() {
            RuntimeValue::I32(val) => Ok(val as u32),
            _ => Ok(0), //TODO
        }
    }

    fn write_shared_rw_memory(&self, i: u32, v: u32) -> Result<()> {
        let _ = self.0.invoke_export(
            "writeSharedRWMemory",
            &[i.into(), v.into()],
            &mut NopExternals,
        );
        Ok(())
    }

    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32) -> Result<()> {
        let _ = self.0.invoke_export(
            "setInputSignal",
            &[hmsb.into(), hlsb.into(), pos.into()],
            &mut NopExternals,
        );
        Ok(())
    }

    fn get_witness(&self, i: u32) -> Result<()> {
        self.0.invoke_export(
            "getWitness",
            &[i.into()],
            &mut NopExternals,
        );
        Ok(())
    }

    fn get_witness_size(&self) -> Result<u32> {
        self.get_u32("getWitnessSize")
    }
}

impl CircomBase for Wasm {
    fn init(&self, sanity_check: bool) -> Result<()> {
        let _ = self.0.invoke_export(
            "init",
            &[RuntimeValue::I32(sanity_check as i32)],
            &mut NopExternals,
        );
        Ok(())
    }

    fn get_ptr_witness_buffer(&self) -> Result<u32> {
        self.get_u32("getWitnessBuffer")
    }

    fn get_ptr_witness(&self, w: u32) -> Result<u32> {
        let result = self.0.invoke_export("getPWitness", &[w.into()], &mut NopExternals)?;
        match result.unwrap() {
            RuntimeValue::I32(val) => Ok(val as u32),
            _ => Ok(0), //TODO
        }
    }

    fn get_n_vars(&self) -> Result<u32> {
        self.get_u32("getNVars")
    }

    fn get_signal_offset32(
        &self,
        p_sig_offset: u32,
        component: u32,
        hash_msb: u32,
        hash_lsb: u32,
    ) -> Result<()> {
        let _ = self.0.invoke_export(
            "getSignalOffset32",
            &[
                p_sig_offset.into(),
                component.into(),
                hash_msb.into(),
                hash_lsb.into(),
            ],
            &mut NopExternals,
        );

        Ok(())
    }

    fn set_signal(&self, c_idx: u32, component: u32, signal: u32, p_val: u32) -> Result<()> {
        let _ = self.0.invoke_export(
            "setSignal",
            &[c_idx.into(), component.into(), signal.into(), p_val.into()],
            &mut NopExternals,
        );
        Ok(())
    }

    // Default to version 1 if it isn't explicitly defined
    fn get_version(&self) -> Result<u32> {
        self.get_u32("getVersion")
    }

    fn get_u32(&self, name: &str) -> Result<u32> {
        let result = self.0.invoke_export(name, &[], &mut NopExternals)?;
        match result.unwrap() {
            RuntimeValue::I32(val) => Ok(val as u32),
            _ => Ok(0), //TODO
        }
    }
}

impl Wasm {
    pub fn new(instance: ModuleRef) -> Self {
        Self(instance)
    }
}
