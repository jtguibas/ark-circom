use color_eyre::{eyre::eyre, Result};
use wasmi::{ModuleRef, NopExternals, RuntimeValue};

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
    fn get_u32(&self, name: &str, args: &[RuntimeValue]) -> Result<u32>;
    fn func(&self, name: &str, args: &[RuntimeValue]) -> Result<Option<RuntimeValue>>;
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
        self.get_u32("getFrLen", &[])
    }

    fn get_ptr_raw_prime(&self) -> Result<u32> {
        self.get_u32("getPRawPrime", &[])
    }
}

#[cfg(feature = "circom-2")]
impl Circom2 for Wasm {
    fn get_field_num_len32(&self) -> Result<u32> {
        self.get_u32("getFieldNumLen32", &[])
    }

    fn get_raw_prime(&self) -> Result<()> {
        self.func("getRawPrime", &[])?;
        Ok(())
    }

    fn read_shared_rw_memory(&self, i: u32) -> Result<u32> {
        self.get_u32("readSharedRWMemory", &[i.into()])
    }

    fn write_shared_rw_memory(&self, i: u32, v: u32) -> Result<()> {
        self.func("writeSharedRWMemory", &[i.into(), v.into()])?;
        Ok(())
    }

    fn set_input_signal(&self, hmsb: u32, hlsb: u32, pos: u32) -> Result<()> {
        self.func("setInputSignal", &[hmsb.into(), hlsb.into(), pos.into()])?;
        Ok(())
    }

    fn get_witness(&self, i: u32) -> Result<()> {
        self.func("getWitness", &[i.into()])?;
        Ok(())
    }

    fn get_witness_size(&self) -> Result<u32> {
        self.get_u32("getWitnessSize", &[])
    }
}

impl CircomBase for Wasm {
    fn init(&self, sanity_check: bool) -> Result<()> {
        self.func("init", &[RuntimeValue::I32(sanity_check as i32)])?;
        Ok(())
    }

    fn get_ptr_witness_buffer(&self) -> Result<u32> {
        self.get_u32("getWitnessBuffer", &[])
    }

    fn get_ptr_witness(&self, w: u32) -> Result<u32> {
        self.get_u32("getPWitness", &[w.into()])
    }

    fn get_n_vars(&self) -> Result<u32> {
        self.get_u32("getNVars", &[])
    }

    fn get_signal_offset32(
        &self,
        p_sig_offset: u32,
        component: u32,
        hash_msb: u32,
        hash_lsb: u32,
    ) -> Result<()> {
        self.func(
            "getSignalOffset32",
            &[
                p_sig_offset.into(),
                component.into(),
                hash_msb.into(),
                hash_lsb.into(),
            ],
        )?;

        Ok(())
    }

    fn set_signal(&self, c_idx: u32, component: u32, signal: u32, p_val: u32) -> Result<()> {
        self.func(
            "setSignal",
            &[c_idx.into(), component.into(), signal.into(), p_val.into()],
        )?;
        Ok(())
    }

    // Default to version 1 if it isn't explicitly defined
    fn get_version(&self) -> Result<u32> {
        self.get_u32("getVersion", &[])
    }

    fn get_u32(&self, name: &str, args: &[RuntimeValue]) -> Result<u32> {
        self.func(name, args)?
            .ok_or_else(|| eyre!("returned null"))?
            .try_into::<u32>()
            .ok_or_else(|| eyre!("parsing as u32"))
    }

    fn func(&self, name: &str, args: &[RuntimeValue]) -> Result<Option<RuntimeValue>> {
        Ok(self.0.invoke_export(name, args, &mut NopExternals)?)
    }
}

impl Wasm {
    pub fn new(instance: ModuleRef) -> Self {
        Self(instance)
    }
}
