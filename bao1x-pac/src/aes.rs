#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_crfunc: SfrCrfunc,
    sfr_ar: SfrAr,
    sfr_srmfsm: SfrSrmfsm,
    sfr_fr: SfrFr,
    sfr_opt: SfrOpt,
    sfr_opt1: SfrOpt1,
    sfr_optltx: SfrOptltx,
    _reserved7: [u8; 0x04],
    sfr_maskseed: SfrMaskseed,
    sfr_maskseedar: SfrMaskseedar,
    _reserved9: [u8; 0x08],
    sfr_segptr_ptrid_iv: SfrSegptrPtridIv,
    sfr_segptr_ptrid_akey: SfrSegptrPtridAkey,
    sfr_segptr_ptrid_aib: SfrSegptrPtridAib,
    sfr_segptr_ptrid_aob: SfrSegptrPtridAob,
}
impl RegisterBlock {
    #[doc = "0x00 - See `aes.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crfunc(&self) -> &SfrCrfunc {
        &self.sfr_crfunc
    }
    #[doc = "0x04 - See `aes.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L141>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar(&self) -> &SfrAr {
        &self.sfr_ar
    }
    #[doc = "0x08 - See `aes.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srmfsm(&self) -> &SfrSrmfsm {
        &self.sfr_srmfsm
    }
    #[doc = "0x0c - See `aes.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L143>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fr(&self) -> &SfrFr {
        &self.sfr_fr
    }
    #[doc = "0x10 - See `aes.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L145>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt(&self) -> &SfrOpt {
        &self.sfr_opt
    }
    #[doc = "0x14 - See `aes.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt1(&self) -> &SfrOpt1 {
        &self.sfr_opt1
    }
    #[doc = "0x18 - See `aes.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optltx(&self) -> &SfrOptltx {
        &self.sfr_optltx
    }
    #[doc = "0x20 - See `aes.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_maskseed(&self) -> &SfrMaskseed {
        &self.sfr_maskseed
    }
    #[doc = "0x24 - See `aes.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_maskseedar(&self) -> &SfrMaskseedar {
        &self.sfr_maskseedar
    }
    #[doc = "0x30 - See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_iv(&self) -> &SfrSegptrPtridIv {
        &self.sfr_segptr_ptrid_iv
    }
    #[doc = "0x34 - See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_akey(&self) -> &SfrSegptrPtridAkey {
        &self.sfr_segptr_ptrid_akey
    }
    #[doc = "0x38 - See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_aib(&self) -> &SfrSegptrPtridAib {
        &self.sfr_segptr_ptrid_aib
    }
    #[doc = "0x3c - See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_aob(&self) -> &SfrSegptrPtridAob {
        &self.sfr_segptr_ptrid_aob
    }
}
#[doc = "SFR_CRFUNC (rw) register accessor: See `aes.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crfunc`] module"]
#[doc(alias = "SFR_CRFUNC")]
pub type SfrCrfunc = crate::Reg<sfr_crfunc::SfrCrfuncSpec>;
#[doc = "See `aes.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_crfunc;
#[doc = "SFR_AR (rw) register accessor: See `aes.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L141>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar`] module"]
#[doc(alias = "SFR_AR")]
pub type SfrAr = crate::Reg<sfr_ar::SfrArSpec>;
#[doc = "See `aes.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L141>`__ (line numbers are approximate)"]
pub mod sfr_ar;
#[doc = "SFR_SRMFSM (rw) register accessor: See `aes.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srmfsm`] module"]
#[doc(alias = "SFR_SRMFSM")]
pub type SfrSrmfsm = crate::Reg<sfr_srmfsm::SfrSrmfsmSpec>;
#[doc = "See `aes.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L142>`__ (line numbers are approximate)"]
pub mod sfr_srmfsm;
#[doc = "SFR_FR (rw) register accessor: See `aes.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L143>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fr`] module"]
#[doc(alias = "SFR_FR")]
pub type SfrFr = crate::Reg<sfr_fr::SfrFrSpec>;
#[doc = "See `aes.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L143>`__ (line numbers are approximate)"]
pub mod sfr_fr;
#[doc = "SFR_OPT (rw) register accessor: See `aes.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L145>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt`] module"]
#[doc(alias = "SFR_OPT")]
pub type SfrOpt = crate::Reg<sfr_opt::SfrOptSpec>;
#[doc = "See `aes.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L145>`__ (line numbers are approximate)"]
pub mod sfr_opt;
#[doc = "SFR_OPT1 (rw) register accessor: See `aes.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt1`] module"]
#[doc(alias = "SFR_OPT1")]
pub type SfrOpt1 = crate::Reg<sfr_opt1::SfrOpt1Spec>;
#[doc = "See `aes.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_opt1;
#[doc = "SFR_OPTLTX (rw) register accessor: See `aes.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optltx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optltx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optltx`] module"]
#[doc(alias = "SFR_OPTLTX")]
pub type SfrOptltx = crate::Reg<sfr_optltx::SfrOptltxSpec>;
#[doc = "See `aes.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_optltx;
#[doc = "SFR_MASKSEED (rw) register accessor: See `aes.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_maskseed`] module"]
#[doc(alias = "SFR_MASKSEED")]
pub type SfrMaskseed = crate::Reg<sfr_maskseed::SfrMaskseedSpec>;
#[doc = "See `aes.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L150>`__ (line numbers are approximate)"]
pub mod sfr_maskseed;
#[doc = "SFR_MASKSEEDAR (rw) register accessor: See `aes.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseedar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseedar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_maskseedar`] module"]
#[doc(alias = "SFR_MASKSEEDAR")]
pub type SfrMaskseedar = crate::Reg<sfr_maskseedar::SfrMaskseedarSpec>;
#[doc = "See `aes.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L151>`__ (line numbers are approximate)"]
pub mod sfr_maskseedar;
#[doc = "SFR_SEGPTR_PTRID_IV (rw) register accessor: See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_iv`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_IV")]
pub type SfrSegptrPtridIv = crate::Reg<sfr_segptr_ptrid_iv::SfrSegptrPtridIvSpec>;
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_iv;
#[doc = "SFR_SEGPTR_PTRID_AKEY (rw) register accessor: See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_akey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_akey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_akey`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_AKEY")]
pub type SfrSegptrPtridAkey = crate::Reg<sfr_segptr_ptrid_akey::SfrSegptrPtridAkeySpec>;
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_akey;
#[doc = "SFR_SEGPTR_PTRID_AIB (rw) register accessor: See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_aib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_aib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_aib`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_AIB")]
pub type SfrSegptrPtridAib = crate::Reg<sfr_segptr_ptrid_aib::SfrSegptrPtridAibSpec>;
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_aib;
#[doc = "SFR_SEGPTR_PTRID_AOB (rw) register accessor: See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_aob::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_aob::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_aob`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_AOB")]
pub type SfrSegptrPtridAob = crate::Reg<sfr_segptr_ptrid_aob::SfrSegptrPtridAobSpec>;
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_aob;
