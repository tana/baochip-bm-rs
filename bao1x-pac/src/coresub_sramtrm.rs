#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_cache: SfrCache,
    sfr_itcm: SfrItcm,
    sfr_dtcm: SfrDtcm,
    sfr_sram0: SfrSram0,
    sfr_sram1: SfrSram1,
    sfr_vexram: SfrVexram,
    _reserved6: [u8; 0x08],
    sfr_sramerr: SfrSramerr,
    _reserved7: [u8; 0x0c],
    sfr_ramsec: SfrRamsec,
}
impl RegisterBlock {
    #[doc = "0x00 - See `coresub_sramtrm.sv#L54 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L54>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cache(&self) -> &SfrCache {
        &self.sfr_cache
    }
    #[doc = "0x04 - See `coresub_sramtrm.sv#L55 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L55>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_itcm(&self) -> &SfrItcm {
        &self.sfr_itcm
    }
    #[doc = "0x08 - See `coresub_sramtrm.sv#L56 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L56>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_dtcm(&self) -> &SfrDtcm {
        &self.sfr_dtcm
    }
    #[doc = "0x0c - See `coresub_sramtrm.sv#L57 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L57>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sram0(&self) -> &SfrSram0 {
        &self.sfr_sram0
    }
    #[doc = "0x10 - See `coresub_sramtrm.sv#L58 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L58>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sram1(&self) -> &SfrSram1 {
        &self.sfr_sram1
    }
    #[doc = "0x14 - See `coresub_sramtrm.sv#L59 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L59>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vexram(&self) -> &SfrVexram {
        &self.sfr_vexram
    }
    #[doc = "0x20 - See `coresub_sramtrm.sv#L60 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L60>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sramerr(&self) -> &SfrSramerr {
        &self.sfr_sramerr
    }
    #[doc = "0x30 - See `coresub_sramtrm.sv#L61 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L61>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ramsec(&self) -> &SfrRamsec {
        &self.sfr_ramsec
    }
}
#[doc = "SFR_CACHE (rw) register accessor: See `coresub_sramtrm.sv#L54 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L54>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cache::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cache::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cache`] module"]
#[doc(alias = "SFR_CACHE")]
pub type SfrCache = crate::Reg<sfr_cache::SfrCacheSpec>;
#[doc = "See `coresub_sramtrm.sv#L54 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L54>`__ (line numbers are approximate)"]
pub mod sfr_cache;
#[doc = "SFR_ITCM (rw) register accessor: See `coresub_sramtrm.sv#L55 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L55>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_itcm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_itcm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_itcm`] module"]
#[doc(alias = "SFR_ITCM")]
pub type SfrItcm = crate::Reg<sfr_itcm::SfrItcmSpec>;
#[doc = "See `coresub_sramtrm.sv#L55 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L55>`__ (line numbers are approximate)"]
pub mod sfr_itcm;
#[doc = "SFR_DTCM (rw) register accessor: See `coresub_sramtrm.sv#L56 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L56>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dtcm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dtcm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_dtcm`] module"]
#[doc(alias = "SFR_DTCM")]
pub type SfrDtcm = crate::Reg<sfr_dtcm::SfrDtcmSpec>;
#[doc = "See `coresub_sramtrm.sv#L56 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L56>`__ (line numbers are approximate)"]
pub mod sfr_dtcm;
#[doc = "SFR_SRAM0 (rw) register accessor: See `coresub_sramtrm.sv#L57 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L57>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sram0`] module"]
#[doc(alias = "SFR_SRAM0")]
pub type SfrSram0 = crate::Reg<sfr_sram0::SfrSram0Spec>;
#[doc = "See `coresub_sramtrm.sv#L57 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L57>`__ (line numbers are approximate)"]
pub mod sfr_sram0;
#[doc = "SFR_SRAM1 (rw) register accessor: See `coresub_sramtrm.sv#L58 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L58>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sram1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sram1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sram1`] module"]
#[doc(alias = "SFR_SRAM1")]
pub type SfrSram1 = crate::Reg<sfr_sram1::SfrSram1Spec>;
#[doc = "See `coresub_sramtrm.sv#L58 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L58>`__ (line numbers are approximate)"]
pub mod sfr_sram1;
#[doc = "SFR_VEXRAM (rw) register accessor: See `coresub_sramtrm.sv#L59 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L59>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vexram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vexram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vexram`] module"]
#[doc(alias = "SFR_VEXRAM")]
pub type SfrVexram = crate::Reg<sfr_vexram::SfrVexramSpec>;
#[doc = "See `coresub_sramtrm.sv#L59 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L59>`__ (line numbers are approximate)"]
pub mod sfr_vexram;
#[doc = "SFR_SRAMERR (rw) register accessor: See `coresub_sramtrm.sv#L60 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L60>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sramerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sramerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sramerr`] module"]
#[doc(alias = "SFR_SRAMERR")]
pub type SfrSramerr = crate::Reg<sfr_sramerr::SfrSramerrSpec>;
#[doc = "See `coresub_sramtrm.sv#L60 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L60>`__ (line numbers are approximate)"]
pub mod sfr_sramerr;
#[doc = "SFR_RAMSEC (rw) register accessor: See `coresub_sramtrm.sv#L61 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L61>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ramsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ramsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ramsec`] module"]
#[doc(alias = "SFR_RAMSEC")]
pub type SfrRamsec = crate::Reg<sfr_ramsec::SfrRamsecSpec>;
#[doc = "See `coresub_sramtrm.sv#L61 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L61>`__ (line numbers are approximate)"]
pub mod sfr_ramsec;
