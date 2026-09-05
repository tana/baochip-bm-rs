#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_crfunc: SfrCrfunc,
    sfr_ar: SfrAr,
    sfr_srmfsm: SfrSrmfsm,
    sfr_fr: SfrFr,
    sfr_crdivlen: SfrCrdivlen,
    sfr_srdivlen: SfrSrdivlen,
    sfr_opt: SfrOpt,
    sfr_optltx: SfrOptltx,
    _reserved8: [u8; 0x10],
    sfr_segptr_cr_segcfg0: SfrSegptrCrSegcfg0,
    sfr_segptr_cr_segcfg1: SfrSegptrCrSegcfg1,
    sfr_segptr_cr_segcfg2: SfrSegptrCrSegcfg2,
    sfr_segptr_cr_segcfg3: SfrSegptrCrSegcfg3,
}
impl RegisterBlock {
    #[doc = "0x00 - See `alu.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L136>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crfunc(&self) -> &SfrCrfunc {
        &self.sfr_crfunc
    }
    #[doc = "0x04 - See `alu.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L137>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar(&self) -> &SfrAr {
        &self.sfr_ar
    }
    #[doc = "0x08 - See `alu.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L138>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srmfsm(&self) -> &SfrSrmfsm {
        &self.sfr_srmfsm
    }
    #[doc = "0x0c - See `alu.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L139>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fr(&self) -> &SfrFr {
        &self.sfr_fr
    }
    #[doc = "0x10 - See `alu.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L141>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crdivlen(&self) -> &SfrCrdivlen {
        &self.sfr_crdivlen
    }
    #[doc = "0x14 - See `alu.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srdivlen(&self) -> &SfrSrdivlen {
        &self.sfr_srdivlen
    }
    #[doc = "0x18 - See `alu.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L143>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt(&self) -> &SfrOpt {
        &self.sfr_opt
    }
    #[doc = "0x1c - See `alu.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L144>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_optltx(&self) -> &SfrOptltx {
        &self.sfr_optltx
    }
    #[doc = "0x30 - See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_cr_segcfg0(&self) -> &SfrSegptrCrSegcfg0 {
        &self.sfr_segptr_cr_segcfg0
    }
    #[doc = "0x34 - See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_cr_segcfg1(&self) -> &SfrSegptrCrSegcfg1 {
        &self.sfr_segptr_cr_segcfg1
    }
    #[doc = "0x38 - See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_cr_segcfg2(&self) -> &SfrSegptrCrSegcfg2 {
        &self.sfr_segptr_cr_segcfg2
    }
    #[doc = "0x3c - See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_cr_segcfg3(&self) -> &SfrSegptrCrSegcfg3 {
        &self.sfr_segptr_cr_segcfg3
    }
}
#[doc = "SFR_CRFUNC (rw) register accessor: See `alu.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crfunc`] module"]
#[doc(alias = "SFR_CRFUNC")]
pub type SfrCrfunc = crate::Reg<sfr_crfunc::SfrCrfuncSpec>;
#[doc = "See `alu.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L136>`__ (line numbers are approximate)"]
pub mod sfr_crfunc;
#[doc = "SFR_AR (rw) register accessor: See `alu.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L137>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar`] module"]
#[doc(alias = "SFR_AR")]
pub type SfrAr = crate::Reg<sfr_ar::SfrArSpec>;
#[doc = "See `alu.sv#L137 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L137>`__ (line numbers are approximate)"]
pub mod sfr_ar;
#[doc = "SFR_SRMFSM (rw) register accessor: See `alu.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srmfsm`] module"]
#[doc(alias = "SFR_SRMFSM")]
pub type SfrSrmfsm = crate::Reg<sfr_srmfsm::SfrSrmfsmSpec>;
#[doc = "See `alu.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L138>`__ (line numbers are approximate)"]
pub mod sfr_srmfsm;
#[doc = "SFR_FR (rw) register accessor: See `alu.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L139>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fr`] module"]
#[doc(alias = "SFR_FR")]
pub type SfrFr = crate::Reg<sfr_fr::SfrFrSpec>;
#[doc = "See `alu.sv#L139 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L139>`__ (line numbers are approximate)"]
pub mod sfr_fr;
#[doc = "SFR_CRDIVLEN (rw) register accessor: See `alu.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L141>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crdivlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crdivlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crdivlen`] module"]
#[doc(alias = "SFR_CRDIVLEN")]
pub type SfrCrdivlen = crate::Reg<sfr_crdivlen::SfrCrdivlenSpec>;
#[doc = "See `alu.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L141>`__ (line numbers are approximate)"]
pub mod sfr_crdivlen;
#[doc = "SFR_SRDIVLEN (rw) register accessor: See `alu.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srdivlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srdivlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srdivlen`] module"]
#[doc(alias = "SFR_SRDIVLEN")]
pub type SfrSrdivlen = crate::Reg<sfr_srdivlen::SfrSrdivlenSpec>;
#[doc = "See `alu.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L142>`__ (line numbers are approximate)"]
pub mod sfr_srdivlen;
#[doc = "SFR_OPT (rw) register accessor: See `alu.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L143>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt`] module"]
#[doc(alias = "SFR_OPT")]
pub type SfrOpt = crate::Reg<sfr_opt::SfrOptSpec>;
#[doc = "See `alu.sv#L143 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L143>`__ (line numbers are approximate)"]
pub mod sfr_opt;
#[doc = "SFR_OPTLTX (rw) register accessor: See `alu.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L144>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optltx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optltx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_optltx`] module"]
#[doc(alias = "SFR_OPTLTX")]
pub type SfrOptltx = crate::Reg<sfr_optltx::SfrOptltxSpec>;
#[doc = "See `alu.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L144>`__ (line numbers are approximate)"]
pub mod sfr_optltx;
#[doc = "SFR_SEGPTR_CR_SEGCFG0 (rw) register accessor: See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_cr_segcfg0`] module"]
#[doc(alias = "SFR_SEGPTR_CR_SEGCFG0")]
pub type SfrSegptrCrSegcfg0 = crate::Reg<sfr_segptr_cr_segcfg0::SfrSegptrCrSegcfg0Spec>;
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_segptr_cr_segcfg0;
#[doc = "SFR_SEGPTR_CR_SEGCFG1 (rw) register accessor: See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_cr_segcfg1`] module"]
#[doc(alias = "SFR_SEGPTR_CR_SEGCFG1")]
pub type SfrSegptrCrSegcfg1 = crate::Reg<sfr_segptr_cr_segcfg1::SfrSegptrCrSegcfg1Spec>;
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_segptr_cr_segcfg1;
#[doc = "SFR_SEGPTR_CR_SEGCFG2 (rw) register accessor: See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_cr_segcfg2`] module"]
#[doc(alias = "SFR_SEGPTR_CR_SEGCFG2")]
pub type SfrSegptrCrSegcfg2 = crate::Reg<sfr_segptr_cr_segcfg2::SfrSegptrCrSegcfg2Spec>;
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_segptr_cr_segcfg2;
#[doc = "SFR_SEGPTR_CR_SEGCFG3 (rw) register accessor: See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_cr_segcfg3`] module"]
#[doc(alias = "SFR_SEGPTR_CR_SEGCFG3")]
pub type SfrSegptrCrSegcfg3 = crate::Reg<sfr_segptr_cr_segcfg3::SfrSegptrCrSegcfg3Spec>;
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)"]
pub mod sfr_segptr_cr_segcfg3;
