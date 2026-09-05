#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    sfr_suben: SfrSuben,
    sfr_apbs: SfrApbs,
    _reserved2: [u8; 0x04],
    sfr_srbusy: SfrSrbusy,
    sfr_frdone: SfrFrdone,
    sfr_frerr: SfrFrerr,
    sfr_arclr: SfrArclr,
    sfr_tickcyc: SfrTickcyc,
    sfr_tickcnt: SfrTickcnt,
    _reserved8: [u8; 0x08],
    sfr_ffen: SfrFfen,
    sfr_ffclr: SfrFfclr,
    _reserved10: [u8; 0x08],
    sfr_ffcnt_sr_ff0: SfrFfcntSrFf0,
    sfr_ffcnt_sr_ff1: SfrFfcntSrFf1,
    sfr_ffcnt_sr_ff2: SfrFfcntSrFf2,
    sfr_ffcnt_sr_ff3: SfrFfcntSrFf3,
    sfr_ffcnt_sr_ff4: SfrFfcntSrFf4,
    sfr_ffcnt_sr_ff5: SfrFfcntSrFf5,
    _reserved16: [u8; 0x08],
    sfr_fracerr: SfrFracerr,
    _reserved17: [u8; 0x7c],
    sfr_ts_sr_ts0: SfrTsSrTs0,
    sfr_ts_sr_ts1: SfrTsSrTs1,
    sfr_ts_sr_ts2: SfrTsSrTs2,
    sfr_ts_sr_ts3: SfrTsSrTs3,
}
impl RegisterBlock {
    #[doc = "0x04 - See `sce_glbsfra.sv#L76 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L76>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_suben(&self) -> &SfrSuben {
        &self.sfr_suben
    }
    #[doc = "0x08 - See `sce_glbsfra.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L77>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_apbs(&self) -> &SfrApbs {
        &self.sfr_apbs
    }
    #[doc = "0x10 - See `sce_glbsfra.sv#L79 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L79>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srbusy(&self) -> &SfrSrbusy {
        &self.sfr_srbusy
    }
    #[doc = "0x14 - See `sce_glbsfra.sv#L80 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L80>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_frdone(&self) -> &SfrFrdone {
        &self.sfr_frdone
    }
    #[doc = "0x18 - See `sce_glbsfra.sv#L81 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L81>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_frerr(&self) -> &SfrFrerr {
        &self.sfr_frerr
    }
    #[doc = "0x1c - See `sce_glbsfra.sv#L84 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L84>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_arclr(&self) -> &SfrArclr {
        &self.sfr_arclr
    }
    #[doc = "0x20 - See `sce_glbsfra.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L116>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tickcyc(&self) -> &SfrTickcyc {
        &self.sfr_tickcyc
    }
    #[doc = "0x24 - See `sce_glbsfra.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tickcnt(&self) -> &SfrTickcnt {
        &self.sfr_tickcnt
    }
    #[doc = "0x30 - See `sce_glbsfra.sv#L90 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L90>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffen(&self) -> &SfrFfen {
        &self.sfr_ffen
    }
    #[doc = "0x34 - See `sce_glbsfra.sv#L96 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L96>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffclr(&self) -> &SfrFfclr {
        &self.sfr_ffclr
    }
    #[doc = "0x40 - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff0(&self) -> &SfrFfcntSrFf0 {
        &self.sfr_ffcnt_sr_ff0
    }
    #[doc = "0x44 - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff1(&self) -> &SfrFfcntSrFf1 {
        &self.sfr_ffcnt_sr_ff1
    }
    #[doc = "0x48 - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff2(&self) -> &SfrFfcntSrFf2 {
        &self.sfr_ffcnt_sr_ff2
    }
    #[doc = "0x4c - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff3(&self) -> &SfrFfcntSrFf3 {
        &self.sfr_ffcnt_sr_ff3
    }
    #[doc = "0x50 - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff4(&self) -> &SfrFfcntSrFf4 {
        &self.sfr_ffcnt_sr_ff4
    }
    #[doc = "0x54 - See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ffcnt_sr_ff5(&self) -> &SfrFfcntSrFf5 {
        &self.sfr_ffcnt_sr_ff5
    }
    #[doc = "0x60 - See `sce_glbsfra.sv#L86 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L86>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fracerr(&self) -> &SfrFracerr {
        &self.sfr_fracerr
    }
    #[doc = "0xe0 - See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ts_sr_ts0(&self) -> &SfrTsSrTs0 {
        &self.sfr_ts_sr_ts0
    }
    #[doc = "0xe4 - See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ts_sr_ts1(&self) -> &SfrTsSrTs1 {
        &self.sfr_ts_sr_ts1
    }
    #[doc = "0xe8 - See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ts_sr_ts2(&self) -> &SfrTsSrTs2 {
        &self.sfr_ts_sr_ts2
    }
    #[doc = "0xec - See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ts_sr_ts3(&self) -> &SfrTsSrTs3 {
        &self.sfr_ts_sr_ts3
    }
}
#[doc = "SFR_SUBEN (rw) register accessor: See `sce_glbsfra.sv#L76 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L76>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_suben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_suben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_suben`] module"]
#[doc(alias = "SFR_SUBEN")]
pub type SfrSuben = crate::Reg<sfr_suben::SfrSubenSpec>;
#[doc = "See `sce_glbsfra.sv#L76 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L76>`__ (line numbers are approximate)"]
pub mod sfr_suben;
#[doc = "SFR_APBS (rw) register accessor: See `sce_glbsfra.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L77>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_apbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_apbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_apbs`] module"]
#[doc(alias = "SFR_APBS")]
pub type SfrApbs = crate::Reg<sfr_apbs::SfrApbsSpec>;
#[doc = "See `sce_glbsfra.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L77>`__ (line numbers are approximate)"]
pub mod sfr_apbs;
#[doc = "SFR_SRBUSY (rw) register accessor: See `sce_glbsfra.sv#L79 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L79>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srbusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srbusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srbusy`] module"]
#[doc(alias = "SFR_SRBUSY")]
pub type SfrSrbusy = crate::Reg<sfr_srbusy::SfrSrbusySpec>;
#[doc = "See `sce_glbsfra.sv#L79 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L79>`__ (line numbers are approximate)"]
pub mod sfr_srbusy;
#[doc = "SFR_FRDONE (rw) register accessor: See `sce_glbsfra.sv#L80 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L80>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_frdone::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_frdone::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_frdone`] module"]
#[doc(alias = "SFR_FRDONE")]
pub type SfrFrdone = crate::Reg<sfr_frdone::SfrFrdoneSpec>;
#[doc = "See `sce_glbsfra.sv#L80 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L80>`__ (line numbers are approximate)"]
pub mod sfr_frdone;
#[doc = "SFR_FRERR (rw) register accessor: See `sce_glbsfra.sv#L81 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L81>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_frerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_frerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_frerr`] module"]
#[doc(alias = "SFR_FRERR")]
pub type SfrFrerr = crate::Reg<sfr_frerr::SfrFrerrSpec>;
#[doc = "See `sce_glbsfra.sv#L81 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L81>`__ (line numbers are approximate)"]
pub mod sfr_frerr;
#[doc = "SFR_ARCLR (rw) register accessor: See `sce_glbsfra.sv#L84 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L84>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_arclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_arclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_arclr`] module"]
#[doc(alias = "SFR_ARCLR")]
pub type SfrArclr = crate::Reg<sfr_arclr::SfrArclrSpec>;
#[doc = "See `sce_glbsfra.sv#L84 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L84>`__ (line numbers are approximate)"]
pub mod sfr_arclr;
#[doc = "SFR_TICKCYC (rw) register accessor: See `sce_glbsfra.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L116>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tickcyc`] module"]
#[doc(alias = "SFR_TICKCYC")]
pub type SfrTickcyc = crate::Reg<sfr_tickcyc::SfrTickcycSpec>;
#[doc = "See `sce_glbsfra.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L116>`__ (line numbers are approximate)"]
pub mod sfr_tickcyc;
#[doc = "SFR_TICKCNT (rw) register accessor: See `sce_glbsfra.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tickcnt`] module"]
#[doc(alias = "SFR_TICKCNT")]
pub type SfrTickcnt = crate::Reg<sfr_tickcnt::SfrTickcntSpec>;
#[doc = "See `sce_glbsfra.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_tickcnt;
#[doc = "SFR_FFEN (rw) register accessor: See `sce_glbsfra.sv#L90 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L90>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffen`] module"]
#[doc(alias = "SFR_FFEN")]
pub type SfrFfen = crate::Reg<sfr_ffen::SfrFfenSpec>;
#[doc = "See `sce_glbsfra.sv#L90 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L90>`__ (line numbers are approximate)"]
pub mod sfr_ffen;
#[doc = "SFR_FFCLR (rw) register accessor: See `sce_glbsfra.sv#L96 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L96>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffclr`] module"]
#[doc(alias = "SFR_FFCLR")]
pub type SfrFfclr = crate::Reg<sfr_ffclr::SfrFfclrSpec>;
#[doc = "See `sce_glbsfra.sv#L96 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L96>`__ (line numbers are approximate)"]
pub mod sfr_ffclr;
#[doc = "SFR_FFCNT_SR_FF0 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff0`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF0")]
pub type SfrFfcntSrFf0 = crate::Reg<sfr_ffcnt_sr_ff0::SfrFfcntSrFf0Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff0;
#[doc = "SFR_FFCNT_SR_FF1 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff1`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF1")]
pub type SfrFfcntSrFf1 = crate::Reg<sfr_ffcnt_sr_ff1::SfrFfcntSrFf1Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff1;
#[doc = "SFR_FFCNT_SR_FF2 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff2`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF2")]
pub type SfrFfcntSrFf2 = crate::Reg<sfr_ffcnt_sr_ff2::SfrFfcntSrFf2Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff2;
#[doc = "SFR_FFCNT_SR_FF3 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff3`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF3")]
pub type SfrFfcntSrFf3 = crate::Reg<sfr_ffcnt_sr_ff3::SfrFfcntSrFf3Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff3;
#[doc = "SFR_FFCNT_SR_FF4 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff4`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF4")]
pub type SfrFfcntSrFf4 = crate::Reg<sfr_ffcnt_sr_ff4::SfrFfcntSrFf4Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff4;
#[doc = "SFR_FFCNT_SR_FF5 (rw) register accessor: See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ffcnt_sr_ff5`] module"]
#[doc(alias = "SFR_FFCNT_SR_FF5")]
pub type SfrFfcntSrFf5 = crate::Reg<sfr_ffcnt_sr_ff5::SfrFfcntSrFf5Spec>;
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)"]
pub mod sfr_ffcnt_sr_ff5;
#[doc = "SFR_FRACERR (rw) register accessor: See `sce_glbsfra.sv#L86 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L86>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fracerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fracerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fracerr`] module"]
#[doc(alias = "SFR_FRACERR")]
pub type SfrFracerr = crate::Reg<sfr_fracerr::SfrFracerrSpec>;
#[doc = "See `sce_glbsfra.sv#L86 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L86>`__ (line numbers are approximate)"]
pub mod sfr_fracerr;
#[doc = "SFR_TS_SR_TS0 (rw) register accessor: See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ts_sr_ts0`] module"]
#[doc(alias = "SFR_TS_SR_TS0")]
pub type SfrTsSrTs0 = crate::Reg<sfr_ts_sr_ts0::SfrTsSrTs0Spec>;
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
pub mod sfr_ts_sr_ts0;
#[doc = "SFR_TS_SR_TS1 (rw) register accessor: See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ts_sr_ts1`] module"]
#[doc(alias = "SFR_TS_SR_TS1")]
pub type SfrTsSrTs1 = crate::Reg<sfr_ts_sr_ts1::SfrTsSrTs1Spec>;
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
pub mod sfr_ts_sr_ts1;
#[doc = "SFR_TS_SR_TS2 (rw) register accessor: See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ts_sr_ts2`] module"]
#[doc(alias = "SFR_TS_SR_TS2")]
pub type SfrTsSrTs2 = crate::Reg<sfr_ts_sr_ts2::SfrTsSrTs2Spec>;
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
pub mod sfr_ts_sr_ts2;
#[doc = "SFR_TS_SR_TS3 (rw) register accessor: See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ts_sr_ts3`] module"]
#[doc(alias = "SFR_TS_SR_TS3")]
pub type SfrTsSrTs3 = crate::Reg<sfr_ts_sr_ts3::SfrTsSrTs3Spec>;
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)"]
pub mod sfr_ts_sr_ts3;
