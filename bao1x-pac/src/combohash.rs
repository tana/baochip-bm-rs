#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_crfunc: SfrCrfunc,
    sfr_ar: SfrAr,
    sfr_srmfsm: SfrSrmfsm,
    sfr_fr: SfrFr,
    sfr_opt1: SfrOpt1,
    sfr_opt2: SfrOpt2,
    sfr_opt3: SfrOpt3,
    sfr_blkt0: SfrBlkt0,
    sfr_segptr_segid_lkey: SfrSegptrSegidLkey,
    sfr_segptr_segid_key: SfrSegptrSegidKey,
    _reserved10: [u8; 0x04],
    sfr_segptr_segid_scrt: SfrSegptrSegidScrt,
    sfr_segptr_segid_msg: SfrSegptrSegidMsg,
    sfr_segptr_segid_hout: SfrSegptrSegidHout,
    _reserved13: [u8; 0x04],
    sfr_segptr_segid_hout2: SfrSegptrSegidHout2,
    _reserved14: [u8; 0x20],
    sfr_keyidx: SfrKeyidx,
}
impl RegisterBlock {
    #[doc = "0x00 - See `combohasha.sv#L208 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L208>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_crfunc(&self) -> &SfrCrfunc {
        &self.sfr_crfunc
    }
    #[doc = "0x04 - See `combohasha.sv#L209 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L209>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar(&self) -> &SfrAr {
        &self.sfr_ar
    }
    #[doc = "0x08 - See `combohasha.sv#L210 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L210>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_srmfsm(&self) -> &SfrSrmfsm {
        &self.sfr_srmfsm
    }
    #[doc = "0x0c - See `combohasha.sv#L211 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L211>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_fr(&self) -> &SfrFr {
        &self.sfr_fr
    }
    #[doc = "0x10 - See `combohasha.sv#L213 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L213>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt1(&self) -> &SfrOpt1 {
        &self.sfr_opt1
    }
    #[doc = "0x14 - See `combohasha.sv#L214 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L214>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt2(&self) -> &SfrOpt2 {
        &self.sfr_opt2
    }
    #[doc = "0x18 - See `combohasha.sv#L215 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L215>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_opt3(&self) -> &SfrOpt3 {
        &self.sfr_opt3
    }
    #[doc = "0x1c - See `combohasha.sv#L216 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L216>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_blkt0(&self) -> &SfrBlkt0 {
        &self.sfr_blkt0
    }
    #[doc = "0x20 - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_lkey(&self) -> &SfrSegptrSegidLkey {
        &self.sfr_segptr_segid_lkey
    }
    #[doc = "0x24 - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_key(&self) -> &SfrSegptrSegidKey {
        &self.sfr_segptr_segid_key
    }
    #[doc = "0x2c - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_scrt(&self) -> &SfrSegptrSegidScrt {
        &self.sfr_segptr_segid_scrt
    }
    #[doc = "0x30 - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_msg(&self) -> &SfrSegptrSegidMsg {
        &self.sfr_segptr_segid_msg
    }
    #[doc = "0x34 - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_hout(&self) -> &SfrSegptrSegidHout {
        &self.sfr_segptr_segid_hout
    }
    #[doc = "0x3c - See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_segptr_segid_hout2(&self) -> &SfrSegptrSegidHout2 {
        &self.sfr_segptr_segid_hout2
    }
    #[doc = "0x60 - See `combohasha.sv#L217 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L217>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_keyidx(&self) -> &SfrKeyidx {
        &self.sfr_keyidx
    }
}
#[doc = "SFR_CRFUNC (rw) register accessor: See `combohasha.sv#L208 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L208>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_crfunc`] module"]
#[doc(alias = "SFR_CRFUNC")]
pub type SfrCrfunc = crate::Reg<sfr_crfunc::SfrCrfuncSpec>;
#[doc = "See `combohasha.sv#L208 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L208>`__ (line numbers are approximate)"]
pub mod sfr_crfunc;
#[doc = "SFR_AR (rw) register accessor: See `combohasha.sv#L209 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L209>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar`] module"]
#[doc(alias = "SFR_AR")]
pub type SfrAr = crate::Reg<sfr_ar::SfrArSpec>;
#[doc = "See `combohasha.sv#L209 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L209>`__ (line numbers are approximate)"]
pub mod sfr_ar;
#[doc = "SFR_SRMFSM (rw) register accessor: See `combohasha.sv#L210 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L210>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_srmfsm`] module"]
#[doc(alias = "SFR_SRMFSM")]
pub type SfrSrmfsm = crate::Reg<sfr_srmfsm::SfrSrmfsmSpec>;
#[doc = "See `combohasha.sv#L210 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L210>`__ (line numbers are approximate)"]
pub mod sfr_srmfsm;
#[doc = "SFR_FR (rw) register accessor: See `combohasha.sv#L211 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L211>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_fr`] module"]
#[doc(alias = "SFR_FR")]
pub type SfrFr = crate::Reg<sfr_fr::SfrFrSpec>;
#[doc = "See `combohasha.sv#L211 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L211>`__ (line numbers are approximate)"]
pub mod sfr_fr;
#[doc = "SFR_OPT1 (rw) register accessor: See `combohasha.sv#L213 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L213>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt1`] module"]
#[doc(alias = "SFR_OPT1")]
pub type SfrOpt1 = crate::Reg<sfr_opt1::SfrOpt1Spec>;
#[doc = "See `combohasha.sv#L213 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L213>`__ (line numbers are approximate)"]
pub mod sfr_opt1;
#[doc = "SFR_OPT2 (rw) register accessor: See `combohasha.sv#L214 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L214>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt2`] module"]
#[doc(alias = "SFR_OPT2")]
pub type SfrOpt2 = crate::Reg<sfr_opt2::SfrOpt2Spec>;
#[doc = "See `combohasha.sv#L214 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L214>`__ (line numbers are approximate)"]
pub mod sfr_opt2;
#[doc = "SFR_OPT3 (rw) register accessor: See `combohasha.sv#L215 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L215>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_opt3`] module"]
#[doc(alias = "SFR_OPT3")]
pub type SfrOpt3 = crate::Reg<sfr_opt3::SfrOpt3Spec>;
#[doc = "See `combohasha.sv#L215 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L215>`__ (line numbers are approximate)"]
pub mod sfr_opt3;
#[doc = "SFR_BLKT0 (rw) register accessor: See `combohasha.sv#L216 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L216>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_blkt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_blkt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_blkt0`] module"]
#[doc(alias = "SFR_BLKT0")]
pub type SfrBlkt0 = crate::Reg<sfr_blkt0::SfrBlkt0Spec>;
#[doc = "See `combohasha.sv#L216 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L216>`__ (line numbers are approximate)"]
pub mod sfr_blkt0;
#[doc = "SFR_SEGPTR_SEGID_LKEY (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_lkey::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_lkey::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_lkey`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_LKEY")]
pub type SfrSegptrSegidLkey = crate::Reg<sfr_segptr_segid_lkey::SfrSegptrSegidLkeySpec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_lkey;
#[doc = "SFR_SEGPTR_SEGID_KEY (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_key`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_KEY")]
pub type SfrSegptrSegidKey = crate::Reg<sfr_segptr_segid_key::SfrSegptrSegidKeySpec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_key;
#[doc = "SFR_SEGPTR_SEGID_SCRT (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_scrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_scrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_scrt`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_SCRT")]
pub type SfrSegptrSegidScrt = crate::Reg<sfr_segptr_segid_scrt::SfrSegptrSegidScrtSpec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_scrt;
#[doc = "SFR_SEGPTR_SEGID_MSG (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_msg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_msg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_msg`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_MSG")]
pub type SfrSegptrSegidMsg = crate::Reg<sfr_segptr_segid_msg::SfrSegptrSegidMsgSpec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_msg;
#[doc = "SFR_SEGPTR_SEGID_HOUT (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_hout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_hout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_hout`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_HOUT")]
pub type SfrSegptrSegidHout = crate::Reg<sfr_segptr_segid_hout::SfrSegptrSegidHoutSpec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_hout;
#[doc = "SFR_SEGPTR_SEGID_HOUT2 (rw) register accessor: See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_hout2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_hout2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_segptr_segid_hout2`] module"]
#[doc(alias = "SFR_SEGPTR_SEGID_HOUT2")]
pub type SfrSegptrSegidHout2 = crate::Reg<sfr_segptr_segid_hout2::SfrSegptrSegidHout2Spec>;
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)"]
pub mod sfr_segptr_segid_hout2;
#[doc = "SFR_KEYIDX (rw) register accessor: See `combohasha.sv#L217 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L217>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_keyidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_keyidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_keyidx`] module"]
#[doc(alias = "SFR_KEYIDX")]
pub type SfrKeyidx = crate::Reg<sfr_keyidx::SfrKeyidxSpec>;
#[doc = "See `combohasha.sv#L217 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L217>`__ (line numbers are approximate)"]
pub mod sfr_keyidx;
