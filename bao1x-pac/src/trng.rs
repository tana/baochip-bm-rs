#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_crsrc: SfrCrsrc,
    sfr_crana: SfrCrana,
    sfr_pp: SfrPp,
    sfr_opt: SfrOpt,
    sfr_sr: SfrSr,
    sfr_ar_gen: SfrArGen,
    sfr_fr: SfrFr,
    _reserved7: [u8; 0x04],
    sfr_drpsz: SfrDrpsz,
    sfr_drgen: SfrDrgen,
    sfr_drreseed: SfrDrreseed,
    _reserved10: [u8; 0x04],
    sfr_buf: SfrBuf,
    _reserved11: [u8; 0x0c],
    sfr_chain_rngchainen0: SfrChainRngchainen0,
    sfr_chain_rngchainen1: SfrChainRngchainen1,
    sfr_chain_rngchainen2: SfrChainRngchainen2,
    sfr_chain_rngchainen3: SfrChainRngchainen3,
}
impl RegisterBlock {
    #[doc = "0x00 - See `trng.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L105>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crsrc(&self) -> &SfrCrsrc {
        &self.sfr_crsrc
    }
    #[doc = "0x04 - See `trng.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L106>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crana(&self) -> &SfrCrana {
        &self.sfr_crana
    }
    #[doc = "0x08 - See `trng.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L107>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pp(&self) -> &SfrPp {
        &self.sfr_pp
    }
    #[doc = "0x0c - See `trng.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L108>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt(&self) -> &SfrOpt {
        &self.sfr_opt
    }
    #[doc = "0x10 - See `trng.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L114>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr(&self) -> &SfrSr {
        &self.sfr_sr
    }
    #[doc = "0x14 - See `trng.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L112>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar_gen(&self) -> &SfrArGen {
        &self.sfr_ar_gen
    }
    #[doc = "0x18 - See `trng.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L115>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fr(&self) -> &SfrFr {
        &self.sfr_fr
    }
    #[doc = "0x20 - See `trng.sv#L239 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L239>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_drpsz(&self) -> &SfrDrpsz {
        &self.sfr_drpsz
    }
    #[doc = "0x24 - See `trng.sv#L240 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L240>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_drgen(&self) -> &SfrDrgen {
        &self.sfr_drgen
    }
    #[doc = "0x28 - See `trng.sv#L241 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L241>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_drreseed(&self) -> &SfrDrreseed {
        &self.sfr_drreseed
    }
    #[doc = "0x30 - See `trng.sv#L242 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L242>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_buf(&self) -> &SfrBuf {
        &self.sfr_buf
    }
    #[doc = "0x40 - See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_chain_rngchainen0(&self) -> &SfrChainRngchainen0 {
        &self.sfr_chain_rngchainen0
    }
    #[doc = "0x44 - See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_chain_rngchainen1(&self) -> &SfrChainRngchainen1 {
        &self.sfr_chain_rngchainen1
    }
    #[doc = "0x48 - See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_chain_rngchainen2(&self) -> &SfrChainRngchainen2 {
        &self.sfr_chain_rngchainen2
    }
    #[doc = "0x4c - See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_chain_rngchainen3(&self) -> &SfrChainRngchainen3 {
        &self.sfr_chain_rngchainen3
    }
}
#[doc = "SFR_CRSRC (rw) register accessor: See `trng.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crsrc`] module"]
#[doc(alias = "SFR_CRSRC")]
pub type SfrCrsrc = crate::Reg<sfr_crsrc::SfrCrsrcSpec>;
#[doc = "See `trng.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L105>`__ (line numbers are approximate)"]
pub mod sfr_crsrc;
#[doc = "SFR_CRANA (rw) register accessor: See `trng.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crana`] module"]
#[doc(alias = "SFR_CRANA")]
pub type SfrCrana = crate::Reg<sfr_crana::SfrCranaSpec>;
#[doc = "See `trng.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L106>`__ (line numbers are approximate)"]
pub mod sfr_crana;
#[doc = "SFR_PP (rw) register accessor: See `trng.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pp`] module"]
#[doc(alias = "SFR_PP")]
pub type SfrPp = crate::Reg<sfr_pp::SfrPpSpec>;
#[doc = "See `trng.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L107>`__ (line numbers are approximate)"]
pub mod sfr_pp;
#[doc = "SFR_OPT (rw) register accessor: See `trng.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L108>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt`] module"]
#[doc(alias = "SFR_OPT")]
pub type SfrOpt = crate::Reg<sfr_opt::SfrOptSpec>;
#[doc = "See `trng.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L108>`__ (line numbers are approximate)"]
pub mod sfr_opt;
#[doc = "SFR_SR (rw) register accessor: See `trng.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L114>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr`] module"]
#[doc(alias = "SFR_SR")]
pub type SfrSr = crate::Reg<sfr_sr::SfrSrSpec>;
#[doc = "See `trng.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L114>`__ (line numbers are approximate)"]
pub mod sfr_sr;
#[doc = "SFR_AR_GEN (rw) register accessor: See `trng.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L112>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar_gen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar_gen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar_gen`] module"]
#[doc(alias = "SFR_AR_GEN")]
pub type SfrArGen = crate::Reg<sfr_ar_gen::SfrArGenSpec>;
#[doc = "See `trng.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L112>`__ (line numbers are approximate)"]
pub mod sfr_ar_gen;
#[doc = "SFR_FR (rw) register accessor: See `trng.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L115>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fr`] module"]
#[doc(alias = "SFR_FR")]
pub type SfrFr = crate::Reg<sfr_fr::SfrFrSpec>;
#[doc = "See `trng.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L115>`__ (line numbers are approximate)"]
pub mod sfr_fr;
#[doc = "SFR_DRPSZ (rw) register accessor: See `trng.sv#L239 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L239>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drpsz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drpsz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_drpsz`] module"]
#[doc(alias = "SFR_DRPSZ")]
pub type SfrDrpsz = crate::Reg<sfr_drpsz::SfrDrpszSpec>;
#[doc = "See `trng.sv#L239 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L239>`__ (line numbers are approximate)"]
pub mod sfr_drpsz;
#[doc = "SFR_DRGEN (rw) register accessor: See `trng.sv#L240 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L240>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drgen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drgen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_drgen`] module"]
#[doc(alias = "SFR_DRGEN")]
pub type SfrDrgen = crate::Reg<sfr_drgen::SfrDrgenSpec>;
#[doc = "See `trng.sv#L240 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L240>`__ (line numbers are approximate)"]
pub mod sfr_drgen;
#[doc = "SFR_DRRESEED (rw) register accessor: See `trng.sv#L241 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L241>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_drreseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_drreseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_drreseed`] module"]
#[doc(alias = "SFR_DRRESEED")]
pub type SfrDrreseed = crate::Reg<sfr_drreseed::SfrDrreseedSpec>;
#[doc = "See `trng.sv#L241 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L241>`__ (line numbers are approximate)"]
pub mod sfr_drreseed;
#[doc = "SFR_BUF (rw) register accessor: See `trng.sv#L242 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L242>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_buf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_buf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_buf`] module"]
#[doc(alias = "SFR_BUF")]
pub type SfrBuf = crate::Reg<sfr_buf::SfrBufSpec>;
#[doc = "See `trng.sv#L242 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L242>`__ (line numbers are approximate)"]
pub mod sfr_buf;
#[doc = "SFR_CHAIN_RNGCHAINEN0 (rw) register accessor: See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_chain_rngchainen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_chain_rngchainen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_chain_rngchainen0`] module"]
#[doc(alias = "SFR_CHAIN_RNGCHAINEN0")]
pub type SfrChainRngchainen0 = crate::Reg<sfr_chain_rngchainen0::SfrChainRngchainen0Spec>;
#[doc = "See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_chain_rngchainen0;
#[doc = "SFR_CHAIN_RNGCHAINEN1 (rw) register accessor: See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_chain_rngchainen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_chain_rngchainen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_chain_rngchainen1`] module"]
#[doc(alias = "SFR_CHAIN_RNGCHAINEN1")]
pub type SfrChainRngchainen1 = crate::Reg<sfr_chain_rngchainen1::SfrChainRngchainen1Spec>;
#[doc = "See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_chain_rngchainen1;
#[doc = "SFR_CHAIN_RNGCHAINEN2 (rw) register accessor: See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_chain_rngchainen2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_chain_rngchainen2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_chain_rngchainen2`] module"]
#[doc(alias = "SFR_CHAIN_RNGCHAINEN2")]
pub type SfrChainRngchainen2 = crate::Reg<sfr_chain_rngchainen2::SfrChainRngchainen2Spec>;
#[doc = "See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_chain_rngchainen2;
#[doc = "SFR_CHAIN_RNGCHAINEN3 (rw) register accessor: See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_chain_rngchainen3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_chain_rngchainen3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_chain_rngchainen3`] module"]
#[doc(alias = "SFR_CHAIN_RNGCHAINEN3")]
pub type SfrChainRngchainen3 = crate::Reg<sfr_chain_rngchainen3::SfrChainRngchainen3Spec>;
#[doc = "See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_chain_rngchainen3;
