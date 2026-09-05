#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_cg: RegCg,
    reg_cfg_evt: RegCfgEvt,
    reg_rst: RegRst,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
    #[inline(always)]
    pub const fn reg_cg(&self) -> &RegCg {
        &self.reg_cg
    }
    #[doc = "0x04 - See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
    #[inline(always)]
    pub const fn reg_cfg_evt(&self) -> &RegCfgEvt {
        &self.reg_cfg_evt
    }
    #[doc = "0x08 - See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
    #[inline(always)]
    pub const fn reg_rst(&self) -> &RegRst {
        &self.reg_rst
    }
}
#[doc = "REG_CG (rw) register accessor: See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cg`] module"]
#[doc(alias = "REG_CG")]
pub type RegCg = crate::Reg<reg_cg::RegCgSpec>;
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
pub mod reg_cg;
#[doc = "REG_CFG_EVT (rw) register accessor: See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cfg_evt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cfg_evt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cfg_evt`] module"]
#[doc(alias = "REG_CFG_EVT")]
pub type RegCfgEvt = crate::Reg<reg_cfg_evt::RegCfgEvtSpec>;
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
pub mod reg_cfg_evt;
#[doc = "REG_RST (rw) register accessor: See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rst`] module"]
#[doc(alias = "REG_RST")]
pub type RegRst = crate::Reg<reg_rst::RegRstSpec>;
#[doc = "See `udma_ctrl.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips/udma/ udma_core/rtl/common/udma_ctrl.sv>`__"]
pub mod reg_rst;
