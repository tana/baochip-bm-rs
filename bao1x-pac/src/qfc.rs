#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_io: SfrIo,
    sfr_ar: SfrAr,
    sfr_iodrv: SfrIodrv,
    _reserved3: [u8; 0x04],
    cr_xip_addrmode: CrXipAddrmode,
    cr_xip_opcode: CrXipOpcode,
    cr_xip_width: CrXipWidth,
    cr_xip_ssel: CrXipSsel,
    cr_xip_dumcyc: CrXipDumcyc,
    cr_xip_cfg: CrXipCfg,
    _reserved9: [u8; 0x18],
    cr_aeskey_aeskeyin0: CrAeskeyAeskeyin0,
    cr_aeskey_aeskeyin1: CrAeskeyAeskeyin1,
    cr_aeskey_aeskeyin2: CrAeskeyAeskeyin2,
    cr_aeskey_aeskeyin3: CrAeskeyAeskeyin3,
    cr_aesena: CrAesena,
}
impl RegisterBlock {
    #[doc = "0x00 - See `qfc.sv#L189 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L189>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_io(&self) -> &SfrIo {
        &self.sfr_io
    }
    #[doc = "0x04 - See `qfc.sv#L190 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L190>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ar(&self) -> &SfrAr {
        &self.sfr_ar
    }
    #[doc = "0x08 - See `qfc.sv#L191 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L191>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_iodrv(&self) -> &SfrIodrv {
        &self.sfr_iodrv
    }
    #[doc = "0x10 - See `qfc.sv#L193 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L193>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_addrmode(&self) -> &CrXipAddrmode {
        &self.cr_xip_addrmode
    }
    #[doc = "0x14 - See `qfc.sv#L194 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L194>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_opcode(&self) -> &CrXipOpcode {
        &self.cr_xip_opcode
    }
    #[doc = "0x18 - See `qfc.sv#L195 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L195>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_width(&self) -> &CrXipWidth {
        &self.cr_xip_width
    }
    #[doc = "0x1c - See `qfc.sv#L196 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L196>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_ssel(&self) -> &CrXipSsel {
        &self.cr_xip_ssel
    }
    #[doc = "0x20 - See `qfc.sv#L197 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L197>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_dumcyc(&self) -> &CrXipDumcyc {
        &self.cr_xip_dumcyc
    }
    #[doc = "0x24 - See `qfc.sv#L198 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L198>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_xip_cfg(&self) -> &CrXipCfg {
        &self.cr_xip_cfg
    }
    #[doc = "0x40 - See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_aeskey_aeskeyin0(&self) -> &CrAeskeyAeskeyin0 {
        &self.cr_aeskey_aeskeyin0
    }
    #[doc = "0x44 - See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_aeskey_aeskeyin1(&self) -> &CrAeskeyAeskeyin1 {
        &self.cr_aeskey_aeskeyin1
    }
    #[doc = "0x48 - See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_aeskey_aeskeyin2(&self) -> &CrAeskeyAeskeyin2 {
        &self.cr_aeskey_aeskeyin2
    }
    #[doc = "0x4c - See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_aeskey_aeskeyin3(&self) -> &CrAeskeyAeskeyin3 {
        &self.cr_aeskey_aeskeyin3
    }
    #[doc = "0x50 - See `qfc.sv#L201 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L201>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_aesena(&self) -> &CrAesena {
        &self.cr_aesena
    }
}
#[doc = "SFR_IO (rw) register accessor: See `qfc.sv#L189 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L189>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_io`] module"]
#[doc(alias = "SFR_IO")]
pub type SfrIo = crate::Reg<sfr_io::SfrIoSpec>;
#[doc = "See `qfc.sv#L189 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L189>`__ (line numbers are approximate)"]
pub mod sfr_io;
#[doc = "SFR_AR (rw) register accessor: See `qfc.sv#L190 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L190>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ar`] module"]
#[doc(alias = "SFR_AR")]
pub type SfrAr = crate::Reg<sfr_ar::SfrArSpec>;
#[doc = "See `qfc.sv#L190 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L190>`__ (line numbers are approximate)"]
pub mod sfr_ar;
#[doc = "SFR_IODRV (rw) register accessor: See `qfc.sv#L191 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L191>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iodrv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iodrv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_iodrv`] module"]
#[doc(alias = "SFR_IODRV")]
pub type SfrIodrv = crate::Reg<sfr_iodrv::SfrIodrvSpec>;
#[doc = "See `qfc.sv#L191 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L191>`__ (line numbers are approximate)"]
pub mod sfr_iodrv;
#[doc = "CR_XIP_ADDRMODE (rw) register accessor: See `qfc.sv#L193 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L193>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_addrmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_addrmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_addrmode`] module"]
#[doc(alias = "CR_XIP_ADDRMODE")]
pub type CrXipAddrmode = crate::Reg<cr_xip_addrmode::CrXipAddrmodeSpec>;
#[doc = "See `qfc.sv#L193 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L193>`__ (line numbers are approximate)"]
pub mod cr_xip_addrmode;
#[doc = "CR_XIP_OPCODE (rw) register accessor: See `qfc.sv#L194 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L194>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_opcode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_opcode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_opcode`] module"]
#[doc(alias = "CR_XIP_OPCODE")]
pub type CrXipOpcode = crate::Reg<cr_xip_opcode::CrXipOpcodeSpec>;
#[doc = "See `qfc.sv#L194 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L194>`__ (line numbers are approximate)"]
pub mod cr_xip_opcode;
#[doc = "CR_XIP_WIDTH (rw) register accessor: See `qfc.sv#L195 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L195>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_width`] module"]
#[doc(alias = "CR_XIP_WIDTH")]
pub type CrXipWidth = crate::Reg<cr_xip_width::CrXipWidthSpec>;
#[doc = "See `qfc.sv#L195 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L195>`__ (line numbers are approximate)"]
pub mod cr_xip_width;
#[doc = "CR_XIP_SSEL (rw) register accessor: See `qfc.sv#L196 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L196>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_ssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_ssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_ssel`] module"]
#[doc(alias = "CR_XIP_SSEL")]
pub type CrXipSsel = crate::Reg<cr_xip_ssel::CrXipSselSpec>;
#[doc = "See `qfc.sv#L196 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L196>`__ (line numbers are approximate)"]
pub mod cr_xip_ssel;
#[doc = "CR_XIP_DUMCYC (rw) register accessor: See `qfc.sv#L197 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L197>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_dumcyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_dumcyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_dumcyc`] module"]
#[doc(alias = "CR_XIP_DUMCYC")]
pub type CrXipDumcyc = crate::Reg<cr_xip_dumcyc::CrXipDumcycSpec>;
#[doc = "See `qfc.sv#L197 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L197>`__ (line numbers are approximate)"]
pub mod cr_xip_dumcyc;
#[doc = "CR_XIP_CFG (rw) register accessor: See `qfc.sv#L198 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L198>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_xip_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_xip_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_xip_cfg`] module"]
#[doc(alias = "CR_XIP_CFG")]
pub type CrXipCfg = crate::Reg<cr_xip_cfg::CrXipCfgSpec>;
#[doc = "See `qfc.sv#L198 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L198>`__ (line numbers are approximate)"]
pub mod cr_xip_cfg;
#[doc = "CR_AESKEY_AESKEYIN0 (rw) register accessor: See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aeskey_aeskeyin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aeskey_aeskeyin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_aeskey_aeskeyin0`] module"]
#[doc(alias = "CR_AESKEY_AESKEYIN0")]
pub type CrAeskeyAeskeyin0 = crate::Reg<cr_aeskey_aeskeyin0::CrAeskeyAeskeyin0Spec>;
#[doc = "See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
pub mod cr_aeskey_aeskeyin0;
#[doc = "CR_AESKEY_AESKEYIN1 (rw) register accessor: See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aeskey_aeskeyin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aeskey_aeskeyin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_aeskey_aeskeyin1`] module"]
#[doc(alias = "CR_AESKEY_AESKEYIN1")]
pub type CrAeskeyAeskeyin1 = crate::Reg<cr_aeskey_aeskeyin1::CrAeskeyAeskeyin1Spec>;
#[doc = "See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
pub mod cr_aeskey_aeskeyin1;
#[doc = "CR_AESKEY_AESKEYIN2 (rw) register accessor: See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aeskey_aeskeyin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aeskey_aeskeyin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_aeskey_aeskeyin2`] module"]
#[doc(alias = "CR_AESKEY_AESKEYIN2")]
pub type CrAeskeyAeskeyin2 = crate::Reg<cr_aeskey_aeskeyin2::CrAeskeyAeskeyin2Spec>;
#[doc = "See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
pub mod cr_aeskey_aeskeyin2;
#[doc = "CR_AESKEY_AESKEYIN3 (rw) register accessor: See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aeskey_aeskeyin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aeskey_aeskeyin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_aeskey_aeskeyin3`] module"]
#[doc(alias = "CR_AESKEY_AESKEYIN3")]
pub type CrAeskeyAeskeyin3 = crate::Reg<cr_aeskey_aeskeyin3::CrAeskeyAeskeyin3Spec>;
#[doc = "See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)"]
pub mod cr_aeskey_aeskeyin3;
#[doc = "CR_AESENA (rw) register accessor: See `qfc.sv#L201 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L201>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aesena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aesena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_aesena`] module"]
#[doc(alias = "CR_AESENA")]
pub type CrAesena = crate::Reg<cr_aesena::CrAesenaSpec>;
#[doc = "See `qfc.sv#L201 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L201>`__ (line numbers are approximate)"]
pub mod cr_aesena;
