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
    reg_cmd_saddr: RegCmdSaddr,
    reg_cmd_size: RegCmdSize,
    reg_cmd_cfg: RegCmdCfg,
    _reserved9: [u8; 0x04],
    reg_status: RegStatus,
    _reserved10: [u8; 0x04],
    reg_ack: RegAck,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x10 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_saddr(&self) -> &RegTxSaddr {
        &self.reg_tx_saddr
    }
    #[doc = "0x14 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_size(&self) -> &RegTxSize {
        &self.reg_tx_size
    }
    #[doc = "0x18 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_cfg(&self) -> &RegTxCfg {
        &self.reg_tx_cfg
    }
    #[doc = "0x20 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cmd_saddr(&self) -> &RegCmdSaddr {
        &self.reg_cmd_saddr
    }
    #[doc = "0x24 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cmd_size(&self) -> &RegCmdSize {
        &self.reg_cmd_size
    }
    #[doc = "0x28 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cmd_cfg(&self) -> &RegCmdCfg {
        &self.reg_cmd_cfg
    }
    #[doc = "0x30 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_status(&self) -> &RegStatus {
        &self.reg_status
    }
    #[doc = "0x38 - See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_ack(&self) -> &RegAck {
        &self.reg_ack
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_TX_SADDR (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_saddr`] module"]
#[doc(alias = "REG_TX_SADDR")]
pub type RegTxSaddr = crate::Reg<reg_tx_saddr::RegTxSaddrSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_tx_saddr;
#[doc = "REG_TX_SIZE (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_size`] module"]
#[doc(alias = "REG_TX_SIZE")]
pub type RegTxSize = crate::Reg<reg_tx_size::RegTxSizeSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_tx_size;
#[doc = "REG_TX_CFG (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_cfg`] module"]
#[doc(alias = "REG_TX_CFG")]
pub type RegTxCfg = crate::Reg<reg_tx_cfg::RegTxCfgSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_tx_cfg;
#[doc = "REG_CMD_SADDR (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmd_saddr`] module"]
#[doc(alias = "REG_CMD_SADDR")]
pub type RegCmdSaddr = crate::Reg<reg_cmd_saddr::RegCmdSaddrSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_cmd_saddr;
#[doc = "REG_CMD_SIZE (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmd_size`] module"]
#[doc(alias = "REG_CMD_SIZE")]
pub type RegCmdSize = crate::Reg<reg_cmd_size::RegCmdSizeSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_cmd_size;
#[doc = "REG_CMD_CFG (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cmd_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cmd_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cmd_cfg`] module"]
#[doc(alias = "REG_CMD_CFG")]
pub type RegCmdCfg = crate::Reg<reg_cmd_cfg::RegCmdCfgSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_cmd_cfg;
#[doc = "REG_STATUS (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_status`] module"]
#[doc(alias = "REG_STATUS")]
pub type RegStatus = crate::Reg<reg_status::RegStatusSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_status;
#[doc = "REG_ACK (rw) register accessor: See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ack::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ack::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_ack`] module"]
#[doc(alias = "REG_ACK")]
pub type RegAck = crate::Reg<reg_ack::RegAckSpec>;
#[doc = "See `udma_i2c_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2c/rtl/udma_i2c_reg_if.sv>`__"]
pub mod reg_ack;
