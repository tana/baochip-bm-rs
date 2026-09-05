#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_txd: SfrTxd,
    sfr_cr: SfrCr,
    sfr_sr: SfrSr,
    sfr_etuc: SfrEtuc,
}
impl RegisterBlock {
    #[doc = "0x00 - See `duart.sv#L42 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L42>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txd(&self) -> &SfrTxd {
        &self.sfr_txd
    }
    #[doc = "0x04 - See `duart.sv#L43 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L43>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr(&self) -> &SfrCr {
        &self.sfr_cr
    }
    #[doc = "0x08 - See `duart.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L44>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr(&self) -> &SfrSr {
        &self.sfr_sr
    }
    #[doc = "0x0c - See `duart.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L45>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_etuc(&self) -> &SfrEtuc {
        &self.sfr_etuc
    }
}
#[doc = "SFR_TXD (rw) register accessor: See `duart.sv#L42 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L42>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txd`] module"]
#[doc(alias = "SFR_TXD")]
pub type SfrTxd = crate::Reg<sfr_txd::SfrTxdSpec>;
#[doc = "See `duart.sv#L42 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L42>`__ (line numbers are approximate)"]
pub mod sfr_txd;
#[doc = "SFR_CR (rw) register accessor: See `duart.sv#L43 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L43>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr`] module"]
#[doc(alias = "SFR_CR")]
pub type SfrCr = crate::Reg<sfr_cr::SfrCrSpec>;
#[doc = "See `duart.sv#L43 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L43>`__ (line numbers are approximate)"]
pub mod sfr_cr;
#[doc = "SFR_SR (rw) register accessor: See `duart.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L44>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr`] module"]
#[doc(alias = "SFR_SR")]
pub type SfrSr = crate::Reg<sfr_sr::SfrSrSpec>;
#[doc = "See `duart.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L44>`__ (line numbers are approximate)"]
pub mod sfr_sr;
#[doc = "SFR_ETUC (rw) register accessor: See `duart.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L45>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_etuc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_etuc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_etuc`] module"]
#[doc(alias = "SFR_ETUC")]
pub type SfrEtuc = crate::Reg<sfr_etuc::SfrEtucSpec>;
#[doc = "See `duart.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L45>`__ (line numbers are approximate)"]
pub mod sfr_etuc;
