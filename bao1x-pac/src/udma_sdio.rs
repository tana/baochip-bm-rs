#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_rx_saddr: RegRxSaddr,
    reg_rx_size: RegRxSize,
    reg_rx_cfg: RegRxCfg,
    _reserved3: [u8; 0x04],
    reg_tx_saddr: RegTxSaddr,
    reg_tx_size: RegTxSize,
    reg_tx_cfg: RegTxCfg,
    _reserved6: [u8; 0x04],
    reg_cmd_op: RegCmdOp,
    _reserved7: [u8; 0x04],
    reg_data_setup: RegDataSetup,
    reg_start: RegStart,
    reg_rsp0: RegRsp0,
    reg_rsp1: RegRsp1,
    reg_rsp2: RegRsp2,
    reg_rsp3: RegRsp3,
    reg_clk_div: RegClkDiv,
    reg_status: RegStatus,
    reg_data_timeout: RegDataTimeout,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x10 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_saddr(&self) -> &RegTxSaddr {
        &self.reg_tx_saddr
    }
    #[doc = "0x14 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_size(&self) -> &RegTxSize {
        &self.reg_tx_size
    }
    #[doc = "0x18 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_cfg(&self) -> &RegTxCfg {
        &self.reg_tx_cfg
    }
    #[doc = "0x20 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cmd_op(&self) -> &RegCmdOp {
        &self.reg_cmd_op
    }
    #[doc = "0x28 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_data_setup(&self) -> &RegDataSetup {
        &self.reg_data_setup
    }
    #[doc = "0x2c - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_start(&self) -> &RegStart {
        &self.reg_start
    }
    #[doc = "0x30 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rsp0(&self) -> &RegRsp0 {
        &self.reg_rsp0
    }
    #[doc = "0x34 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rsp1(&self) -> &RegRsp1 {
        &self.reg_rsp1
    }
    #[doc = "0x38 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rsp2(&self) -> &RegRsp2 {
        &self.reg_rsp2
    }
    #[doc = "0x3c - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rsp3(&self) -> &RegRsp3 {
        &self.reg_rsp3
    }
    #[doc = "0x40 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_clk_div(&self) -> &RegClkDiv {
        &self.reg_clk_div
    }
    #[doc = "0x44 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_status(&self) -> &RegStatus {
        &self.reg_status
    }
    #[doc = "0x48 - See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_data_timeout(&self) -> &RegDataTimeout {
        &self.reg_data_timeout
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_TX_SADDR (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_saddr`] module"]
#[doc(alias = "REG_TX_SADDR")]
pub type RegTxSaddr = crate::Reg<reg_tx_saddr::RegTxSaddrSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_tx_saddr;
#[doc = "REG_TX_SIZE (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_size`] module"]
#[doc(alias = "REG_TX_SIZE")]
pub type RegTxSize = crate::Reg<reg_tx_size::RegTxSizeSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_tx_size;
#[doc = "REG_TX_CFG (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_cfg`] module"]
#[doc(alias = "REG_TX_CFG")]
pub type RegTxCfg = crate::Reg<reg_tx_cfg::RegTxCfgSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_tx_cfg;
#[doc = "REG_CMD_OP (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_op::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_op::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmd_op`] module"]
#[doc(alias = "REG_CMD_OP")]
pub type RegCmdOp = crate::Reg<reg_cmd_op::RegCmdOpSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_cmd_op;
#[doc = "REG_DATA_SETUP (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_data_setup`] module"]
#[doc(alias = "REG_DATA_SETUP")]
pub type RegDataSetup = crate::Reg<reg_data_setup::RegDataSetupSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_data_setup;
#[doc = "REG_START (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_start`] module"]
#[doc(alias = "REG_START")]
pub type RegStart = crate::Reg<reg_start::RegStartSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_start;
#[doc = "REG_RSP0 (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rsp0`] module"]
#[doc(alias = "REG_RSP0")]
pub type RegRsp0 = crate::Reg<reg_rsp0::RegRsp0Spec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rsp0;
#[doc = "REG_RSP1 (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rsp1`] module"]
#[doc(alias = "REG_RSP1")]
pub type RegRsp1 = crate::Reg<reg_rsp1::RegRsp1Spec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rsp1;
#[doc = "REG_RSP2 (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rsp2`] module"]
#[doc(alias = "REG_RSP2")]
pub type RegRsp2 = crate::Reg<reg_rsp2::RegRsp2Spec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rsp2;
#[doc = "REG_RSP3 (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsp3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsp3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rsp3`] module"]
#[doc(alias = "REG_RSP3")]
pub type RegRsp3 = crate::Reg<reg_rsp3::RegRsp3Spec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_rsp3;
#[doc = "REG_CLK_DIV (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clk_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clk_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_clk_div`] module"]
#[doc(alias = "REG_CLK_DIV")]
pub type RegClkDiv = crate::Reg<reg_clk_div::RegClkDivSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_clk_div;
#[doc = "REG_STATUS (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_status`] module"]
#[doc(alias = "REG_STATUS")]
pub type RegStatus = crate::Reg<reg_status::RegStatusSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_status;
#[doc = "REG_DATA_TIMEOUT (rw) register accessor: See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_data_timeout`] module"]
#[doc(alias = "REG_DATA_TIMEOUT")]
pub type RegDataTimeout = crate::Reg<reg_data_timeout::RegDataTimeoutSpec>;
#[doc = "See `udma_sdio_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_sdio/rtl/udma_sdio_reg_if.sv>`__"]
pub mod reg_data_timeout;
