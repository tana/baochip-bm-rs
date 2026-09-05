#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfrcr_trm: SfrcrTrm,
    sfrsr_trm: SfrsrTrm,
    sfrar_trm: SfrarTrm,
}
impl RegisterBlock {
    #[doc = "0x00 - See `rbist_wrp.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L174>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfrcr_trm(&self) -> &SfrcrTrm {
        &self.sfrcr_trm
    }
    #[doc = "0x04 - See `rbist_wrp.sv#L175 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L175>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfrsr_trm(&self) -> &SfrsrTrm {
        &self.sfrsr_trm
    }
    #[doc = "0x08 - See `rbist_wrp.sv#L176 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L176>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfrar_trm(&self) -> &SfrarTrm {
        &self.sfrar_trm
    }
}
#[doc = "SFRCR_TRM (rw) register accessor: See `rbist_wrp.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L174>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrcr_trm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrcr_trm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrcr_trm`] module"]
#[doc(alias = "SFRCR_TRM")]
pub type SfrcrTrm = crate::Reg<sfrcr_trm::SfrcrTrmSpec>;
#[doc = "See `rbist_wrp.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L174>`__ (line numbers are approximate)"]
pub mod sfrcr_trm;
#[doc = "SFRSR_TRM (rw) register accessor: See `rbist_wrp.sv#L175 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L175>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrsr_trm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrsr_trm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrsr_trm`] module"]
#[doc(alias = "SFRSR_TRM")]
pub type SfrsrTrm = crate::Reg<sfrsr_trm::SfrsrTrmSpec>;
#[doc = "See `rbist_wrp.sv#L175 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L175>`__ (line numbers are approximate)"]
pub mod sfrsr_trm;
#[doc = "SFRAR_TRM (rw) register accessor: See `rbist_wrp.sv#L176 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L176>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrar_trm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrar_trm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrar_trm`] module"]
#[doc(alias = "SFRAR_TRM")]
pub type SfrarTrm = crate::Reg<sfrar_trm::SfrarTrmSpec>;
#[doc = "See `rbist_wrp.sv#L176 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L176>`__ (line numbers are approximate)"]
pub mod sfrar_trm;
