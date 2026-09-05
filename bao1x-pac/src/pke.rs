#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_crfunc: SfrCrfunc,
    sfr_ar2: SfrAr2,
    sfr_srmfsm: SfrSrmfsm,
    sfr_fr: SfrFr,
    sfr_optnw: SfrOptnw,
    sfr_optew: SfrOptew,
    sfr_optrw: SfrOptrw,
    sfr_optltx: SfrOptltx,
    sfr_optmask: SfrOptmask,
    sfr_mimmcr: SfrMimmcr,
    _reserved10: [u8; 0x08],
    sfr_segptr_ptrid_pcon: SfrSegptrPtridPcon,
    sfr_segptr_ptrid_pib0: SfrSegptrPtridPib0,
    sfr_segptr_ptrid_pib1: SfrSegptrPtridPib1,
    sfr_segptr_ptrid_pkb: SfrSegptrPtridPkb,
    sfr_segptr_ptrid_pob: SfrSegptrPtridPob,
    _reserved15: [u8; 0x0c],
    sfr_tickcyc: SfrTickcyc,
    sfr_tickcnt: SfrTickcnt,
    _reserved17: [u8; 0x08],
    sfr_maskseed: SfrMaskseed,
    sfr_maskseedar: SfrMaskseedar,
}
impl RegisterBlock {
    #[doc = "0x00 - See `pke.sv#L294 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L294>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crfunc(&self) -> &SfrCrfunc {
        &self.sfr_crfunc
    }
    #[doc = "0x04 - See `pke.sv#L296 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L296>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar2(&self) -> &SfrAr2 {
        &self.sfr_ar2
    }
    #[doc = "0x08 - See `pke.sv#L297 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L297>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srmfsm(&self) -> &SfrSrmfsm {
        &self.sfr_srmfsm
    }
    #[doc = "0x0c - See `pke.sv#L298 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L298>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fr(&self) -> &SfrFr {
        &self.sfr_fr
    }
    #[doc = "0x10 - See `pke.sv#L300 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L300>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optnw(&self) -> &SfrOptnw {
        &self.sfr_optnw
    }
    #[doc = "0x14 - See `pke.sv#L301 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L301>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optew(&self) -> &SfrOptew {
        &self.sfr_optew
    }
    #[doc = "0x18 - See `pke.sv#L302 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L302>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optrw(&self) -> &SfrOptrw {
        &self.sfr_optrw
    }
    #[doc = "0x1c - See `pke.sv#L303 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L303>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optltx(&self) -> &SfrOptltx {
        &self.sfr_optltx
    }
    #[doc = "0x20 - See `pke.sv#L305 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L305>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optmask(&self) -> &SfrOptmask {
        &self.sfr_optmask
    }
    #[doc = "0x24 - See `pke.sv#L306 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L306>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mimmcr(&self) -> &SfrMimmcr {
        &self.sfr_mimmcr
    }
    #[doc = "0x30 - See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_pcon(&self) -> &SfrSegptrPtridPcon {
        &self.sfr_segptr_ptrid_pcon
    }
    #[doc = "0x34 - See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_pib0(&self) -> &SfrSegptrPtridPib0 {
        &self.sfr_segptr_ptrid_pib0
    }
    #[doc = "0x38 - See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_pib1(&self) -> &SfrSegptrPtridPib1 {
        &self.sfr_segptr_ptrid_pib1
    }
    #[doc = "0x3c - See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_pkb(&self) -> &SfrSegptrPtridPkb {
        &self.sfr_segptr_ptrid_pkb
    }
    #[doc = "0x40 - See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_ptrid_pob(&self) -> &SfrSegptrPtridPob {
        &self.sfr_segptr_ptrid_pob
    }
    #[doc = "0x50 - See `pke.sv#L309 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L309>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tickcyc(&self) -> &SfrTickcyc {
        &self.sfr_tickcyc
    }
    #[doc = "0x54 - See `pke.sv#L310 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L310>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tickcnt(&self) -> &SfrTickcnt {
        &self.sfr_tickcnt
    }
    #[doc = "0x60 - See `pke.sv#L312 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L312>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_maskseed(&self) -> &SfrMaskseed {
        &self.sfr_maskseed
    }
    #[doc = "0x64 - See `pke.sv#L313 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L313>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_maskseedar(&self) -> &SfrMaskseedar {
        &self.sfr_maskseedar
    }
}
#[doc = "SFR_CRFUNC (rw) register accessor: See `pke.sv#L294 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L294>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crfunc`] module"]
#[doc(alias = "SFR_CRFUNC")]
pub type SfrCrfunc = crate::Reg<sfr_crfunc::SfrCrfuncSpec>;
#[doc = "See `pke.sv#L294 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L294>`__ (line numbers are approximate)"]
pub mod sfr_crfunc;
#[doc = "SFR_AR2 (rw) register accessor: See `pke.sv#L296 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L296>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar2`] module"]
#[doc(alias = "SFR_AR2")]
pub type SfrAr2 = crate::Reg<sfr_ar2::SfrAr2Spec>;
#[doc = "See `pke.sv#L296 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L296>`__ (line numbers are approximate)"]
pub mod sfr_ar2;
#[doc = "SFR_SRMFSM (rw) register accessor: See `pke.sv#L297 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L297>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srmfsm`] module"]
#[doc(alias = "SFR_SRMFSM")]
pub type SfrSrmfsm = crate::Reg<sfr_srmfsm::SfrSrmfsmSpec>;
#[doc = "See `pke.sv#L297 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L297>`__ (line numbers are approximate)"]
pub mod sfr_srmfsm;
#[doc = "SFR_FR (rw) register accessor: See `pke.sv#L298 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L298>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fr`] module"]
#[doc(alias = "SFR_FR")]
pub type SfrFr = crate::Reg<sfr_fr::SfrFrSpec>;
#[doc = "See `pke.sv#L298 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L298>`__ (line numbers are approximate)"]
pub mod sfr_fr;
#[doc = "SFR_OPTNW (rw) register accessor: See `pke.sv#L300 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L300>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optnw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optnw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optnw`] module"]
#[doc(alias = "SFR_OPTNW")]
pub type SfrOptnw = crate::Reg<sfr_optnw::SfrOptnwSpec>;
#[doc = "See `pke.sv#L300 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L300>`__ (line numbers are approximate)"]
pub mod sfr_optnw;
#[doc = "SFR_OPTEW (rw) register accessor: See `pke.sv#L301 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L301>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optew::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optew::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optew`] module"]
#[doc(alias = "SFR_OPTEW")]
pub type SfrOptew = crate::Reg<sfr_optew::SfrOptewSpec>;
#[doc = "See `pke.sv#L301 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L301>`__ (line numbers are approximate)"]
pub mod sfr_optew;
#[doc = "SFR_OPTRW (rw) register accessor: See `pke.sv#L302 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L302>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optrw`] module"]
#[doc(alias = "SFR_OPTRW")]
pub type SfrOptrw = crate::Reg<sfr_optrw::SfrOptrwSpec>;
#[doc = "See `pke.sv#L302 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L302>`__ (line numbers are approximate)"]
pub mod sfr_optrw;
#[doc = "SFR_OPTLTX (rw) register accessor: See `pke.sv#L303 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L303>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optltx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optltx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optltx`] module"]
#[doc(alias = "SFR_OPTLTX")]
pub type SfrOptltx = crate::Reg<sfr_optltx::SfrOptltxSpec>;
#[doc = "See `pke.sv#L303 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L303>`__ (line numbers are approximate)"]
pub mod sfr_optltx;
#[doc = "SFR_OPTMASK (rw) register accessor: See `pke.sv#L305 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L305>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optmask`] module"]
#[doc(alias = "SFR_OPTMASK")]
pub type SfrOptmask = crate::Reg<sfr_optmask::SfrOptmaskSpec>;
#[doc = "See `pke.sv#L305 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L305>`__ (line numbers are approximate)"]
pub mod sfr_optmask;
#[doc = "SFR_MIMMCR (rw) register accessor: See `pke.sv#L306 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L306>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mimmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mimmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mimmcr`] module"]
#[doc(alias = "SFR_MIMMCR")]
pub type SfrMimmcr = crate::Reg<sfr_mimmcr::SfrMimmcrSpec>;
#[doc = "See `pke.sv#L306 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L306>`__ (line numbers are approximate)"]
pub mod sfr_mimmcr;
#[doc = "SFR_SEGPTR_PTRID_PCON (rw) register accessor: See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_pcon`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_PCON")]
pub type SfrSegptrPtridPcon = crate::Reg<sfr_segptr_ptrid_pcon::SfrSegptrPtridPconSpec>;
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_pcon;
#[doc = "SFR_SEGPTR_PTRID_PIB0 (rw) register accessor: See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_pib0`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_PIB0")]
pub type SfrSegptrPtridPib0 = crate::Reg<sfr_segptr_ptrid_pib0::SfrSegptrPtridPib0Spec>;
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_pib0;
#[doc = "SFR_SEGPTR_PTRID_PIB1 (rw) register accessor: See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_pib1`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_PIB1")]
pub type SfrSegptrPtridPib1 = crate::Reg<sfr_segptr_ptrid_pib1::SfrSegptrPtridPib1Spec>;
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_pib1;
#[doc = "SFR_SEGPTR_PTRID_PKB (rw) register accessor: See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pkb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pkb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_pkb`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_PKB")]
pub type SfrSegptrPtridPkb = crate::Reg<sfr_segptr_ptrid_pkb::SfrSegptrPtridPkbSpec>;
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_pkb;
#[doc = "SFR_SEGPTR_PTRID_POB (rw) register accessor: See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pob::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pob::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_ptrid_pob`] module"]
#[doc(alias = "SFR_SEGPTR_PTRID_POB")]
pub type SfrSegptrPtridPob = crate::Reg<sfr_segptr_ptrid_pob::SfrSegptrPtridPobSpec>;
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)"]
pub mod sfr_segptr_ptrid_pob;
#[doc = "SFR_TICKCYC (rw) register accessor: See `pke.sv#L309 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L309>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tickcyc`] module"]
#[doc(alias = "SFR_TICKCYC")]
pub type SfrTickcyc = crate::Reg<sfr_tickcyc::SfrTickcycSpec>;
#[doc = "See `pke.sv#L309 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L309>`__ (line numbers are approximate)"]
pub mod sfr_tickcyc;
#[doc = "SFR_TICKCNT (rw) register accessor: See `pke.sv#L310 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L310>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tickcnt`] module"]
#[doc(alias = "SFR_TICKCNT")]
pub type SfrTickcnt = crate::Reg<sfr_tickcnt::SfrTickcntSpec>;
#[doc = "See `pke.sv#L310 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L310>`__ (line numbers are approximate)"]
pub mod sfr_tickcnt;
#[doc = "SFR_MASKSEED (rw) register accessor: See `pke.sv#L312 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L312>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_maskseed`] module"]
#[doc(alias = "SFR_MASKSEED")]
pub type SfrMaskseed = crate::Reg<sfr_maskseed::SfrMaskseedSpec>;
#[doc = "See `pke.sv#L312 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L312>`__ (line numbers are approximate)"]
pub mod sfr_maskseed;
#[doc = "SFR_MASKSEEDAR (rw) register accessor: See `pke.sv#L313 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L313>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_maskseedar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_maskseedar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_maskseedar`] module"]
#[doc(alias = "SFR_MASKSEEDAR")]
pub type SfrMaskseedar = crate::Reg<sfr_maskseedar::SfrMaskseedarSpec>;
#[doc = "See `pke.sv#L313 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L313>`__ (line numbers are approximate)"]
pub mod sfr_maskseedar;
