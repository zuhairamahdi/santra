
pub struct RegisterConsts {
    pub int_regs_count: i32,
    pub vec_regs_count: i32,
    pub flags_count: i32,
    pub fp_flags_count: i32,
    pub int_and_vec_regs_count: i32,
    pub fp_flags_offset: i32,
    pub total_count: i32,
    pub zero_index: i32,
}

impl RegisterConsts {
    pub fn new() -> RegisterConsts {
        RegisterConsts {
            int_regs_count: 32,
            vec_regs_count: 32,
            flags_count: 32,
            fp_flags_count: 32,
            int_and_vec_regs_count: int_regs_count + vec_regs_count,
            fp_flags_offset: int_regs_count + vec_regs_count + flags_count,
            total_count: int_regs_count + vec_regs_count + flags_count + fp_flags_count,
            zero_index: 31,
        }
    }
}