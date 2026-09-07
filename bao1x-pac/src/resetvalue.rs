#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pc: Pc,
}
impl RegisterBlock {
    #[doc = "0x00 - Latched value for PC on reset"]
    #[inline(always)]
    pub const fn pc(&self) -> &Pc {
        &self.pc
    }
}
#[doc = "PC (rw) register accessor: Latched value for PC on reset\n\nYou can [`read`](crate::Reg::read) this register and get [`pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`] module"]
#[doc(alias = "PC")]
pub type Pc = crate::Reg<pc::PcSpec>;
#[doc = "Latched value for PC on reset"]
pub mod pc;
