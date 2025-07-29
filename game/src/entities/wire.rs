pub struct Wire {
    gate_in_id: u32,
    gate_out_id: u32,
    value: u64,
}

impl Wire {
    pub fn new(gate_in_id: u32, gate_out_id: u32) -> Self {
        Self {
            gate_in_id,
            gate_out_id,
            value: 0,
        }
    }
}
