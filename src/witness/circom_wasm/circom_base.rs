use color_eyre::Result;

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
    fn invoke_func(&self, name: &str, args: &[u32]) -> Result<()>;
    fn invoke_func_u32(&self, name: &str, args: &[u32]) -> Result<u32>;
    fn set_signal(&self, c_idx: u32, component: u32, signal: u32, p_val: u32) -> Result<()>;
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

