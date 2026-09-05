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
    reg_i2s_clkcfg_setup: RegI2sClkcfgSetup,
    reg_i2s_slv_setup: RegI2sSlvSetup,
    reg_i2s_mst_setup: RegI2sMstSetup,
    reg_i2s_pdm_setup: RegI2sPdmSetup,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x10 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_saddr(&self) -> &RegTxSaddr {
        &self.reg_tx_saddr
    }
    #[doc = "0x14 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_size(&self) -> &RegTxSize {
        &self.reg_tx_size
    }
    #[doc = "0x18 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_cfg(&self) -> &RegTxCfg {
        &self.reg_tx_cfg
    }
    #[doc = "0x20 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_i2s_clkcfg_setup(&self) -> &RegI2sClkcfgSetup {
        &self.reg_i2s_clkcfg_setup
    }
    #[doc = "0x24 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_i2s_slv_setup(&self) -> &RegI2sSlvSetup {
        &self.reg_i2s_slv_setup
    }
    #[doc = "0x28 - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_i2s_mst_setup(&self) -> &RegI2sMstSetup {
        &self.reg_i2s_mst_setup
    }
    #[doc = "0x2c - See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_i2s_pdm_setup(&self) -> &RegI2sPdmSetup {
        &self.reg_i2s_pdm_setup
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_TX_SADDR (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_saddr`] module"]
#[doc(alias = "REG_TX_SADDR")]
pub type RegTxSaddr = crate::Reg<reg_tx_saddr::RegTxSaddrSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_tx_saddr;
#[doc = "REG_TX_SIZE (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_size`] module"]
#[doc(alias = "REG_TX_SIZE")]
pub type RegTxSize = crate::Reg<reg_tx_size::RegTxSizeSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_tx_size;
#[doc = "REG_TX_CFG (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_cfg`] module"]
#[doc(alias = "REG_TX_CFG")]
pub type RegTxCfg = crate::Reg<reg_tx_cfg::RegTxCfgSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_tx_cfg;
#[doc = "REG_I2S_CLKCFG_SETUP (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_clkcfg_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_clkcfg_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_i2s_clkcfg_setup`] module"]
#[doc(alias = "REG_I2S_CLKCFG_SETUP")]
pub type RegI2sClkcfgSetup = crate::Reg<reg_i2s_clkcfg_setup::RegI2sClkcfgSetupSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_i2s_clkcfg_setup;
#[doc = "REG_I2S_SLV_SETUP (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_slv_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_slv_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_i2s_slv_setup`] module"]
#[doc(alias = "REG_I2S_SLV_SETUP")]
pub type RegI2sSlvSetup = crate::Reg<reg_i2s_slv_setup::RegI2sSlvSetupSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_i2s_slv_setup;
#[doc = "REG_I2S_MST_SETUP (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_mst_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_mst_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_i2s_mst_setup`] module"]
#[doc(alias = "REG_I2S_MST_SETUP")]
pub type RegI2sMstSetup = crate::Reg<reg_i2s_mst_setup::RegI2sMstSetupSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_i2s_mst_setup;
#[doc = "REG_I2S_PDM_SETUP (rw) register accessor: See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2s_pdm_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2s_pdm_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_i2s_pdm_setup`] module"]
#[doc(alias = "REG_I2S_PDM_SETUP")]
pub type RegI2sPdmSetup = crate::Reg<reg_i2s_pdm_setup::RegI2sPdmSetupSpec>;
#[doc = "See `udma_i2s_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ips /udma/udma_i2s/rtl/udma_i2s_reg_if.sv>`__"]
pub mod reg_i2s_pdm_setup;
