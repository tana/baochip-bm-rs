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
    reg_status: RegStatus,
    reg_scif_setup: RegScifSetup,
    reg_error: RegError,
    reg_irq_en: RegIrqEn,
    reg_valid: RegValid,
    reg_data: RegData,
    reg_scif_etu: RegScifEtu,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x10 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_saddr(&self) -> &RegTxSaddr {
        &self.reg_tx_saddr
    }
    #[doc = "0x14 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_size(&self) -> &RegTxSize {
        &self.reg_tx_size
    }
    #[doc = "0x18 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_cfg(&self) -> &RegTxCfg {
        &self.reg_tx_cfg
    }
    #[doc = "0x20 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_status(&self) -> &RegStatus {
        &self.reg_status
    }
    #[doc = "0x24 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_scif_setup(&self) -> &RegScifSetup {
        &self.reg_scif_setup
    }
    #[doc = "0x28 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_error(&self) -> &RegError {
        &self.reg_error
    }
    #[doc = "0x2c - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_irq_en(&self) -> &RegIrqEn {
        &self.reg_irq_en
    }
    #[doc = "0x30 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_valid(&self) -> &RegValid {
        &self.reg_valid
    }
    #[doc = "0x34 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_data(&self) -> &RegData {
        &self.reg_data
    }
    #[doc = "0x38 - See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
    #[inline(always)]
    pub const fn reg_scif_etu(&self) -> &RegScifEtu {
        &self.reg_scif_etu
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_TX_SADDR (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_saddr`] module"]
#[doc(alias = "REG_TX_SADDR")]
pub type RegTxSaddr = crate::Reg<reg_tx_saddr::RegTxSaddrSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_tx_saddr;
#[doc = "REG_TX_SIZE (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_size`] module"]
#[doc(alias = "REG_TX_SIZE")]
pub type RegTxSize = crate::Reg<reg_tx_size::RegTxSizeSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_tx_size;
#[doc = "REG_TX_CFG (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_cfg`] module"]
#[doc(alias = "REG_TX_CFG")]
pub type RegTxCfg = crate::Reg<reg_tx_cfg::RegTxCfgSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_tx_cfg;
#[doc = "REG_STATUS (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_status`] module"]
#[doc(alias = "REG_STATUS")]
pub type RegStatus = crate::Reg<reg_status::RegStatusSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_status;
#[doc = "REG_SCIF_SETUP (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scif_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_scif_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_scif_setup`] module"]
#[doc(alias = "REG_SCIF_SETUP")]
pub type RegScifSetup = crate::Reg<reg_scif_setup::RegScifSetupSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_scif_setup;
#[doc = "REG_ERROR (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_error`] module"]
#[doc(alias = "REG_ERROR")]
pub type RegError = crate::Reg<reg_error::RegErrorSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_error;
#[doc = "REG_IRQ_EN (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_irq_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_irq_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_irq_en`] module"]
#[doc(alias = "REG_IRQ_EN")]
pub type RegIrqEn = crate::Reg<reg_irq_en::RegIrqEnSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_irq_en;
#[doc = "REG_VALID (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_valid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_valid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_valid`] module"]
#[doc(alias = "REG_VALID")]
pub type RegValid = crate::Reg<reg_valid::RegValidSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_valid;
#[doc = "REG_DATA (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_data`] module"]
#[doc(alias = "REG_DATA")]
pub type RegData = crate::Reg<reg_data::RegDataSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_data;
#[doc = "REG_SCIF_ETU (rw) register accessor: See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scif_etu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_scif_etu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_scif_etu`] module"]
#[doc(alias = "REG_SCIF_ETU")]
pub type RegScifEtu = crate::Reg<reg_scif_etu::RegScifEtuSpec>;
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__"]
pub mod reg_scif_etu;
