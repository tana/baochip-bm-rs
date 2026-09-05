#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_rx_saddr: RegRxSaddr,
    reg_rx_size: RegRxSize,
    reg_rx_cfg: RegRxCfg,
    _reserved3: [u8; 0x04],
    reg_cr_adc: RegCrAdc,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_saddr(&self) -> &RegRxSaddr {
        &self.reg_rx_saddr
    }
    #[doc = "0x04 - See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_size(&self) -> &RegRxSize {
        &self.reg_rx_size
    }
    #[doc = "0x08 - See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_cfg(&self) -> &RegRxCfg {
        &self.reg_rx_cfg
    }
    #[doc = "0x10 - See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_cr_adc(&self) -> &RegCrAdc {
        &self.reg_cr_adc
    }
}
#[doc = "REG_RX_SADDR (rw) register accessor: See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_saddr`] module"]
#[doc(alias = "REG_RX_SADDR")]
pub type RegRxSaddr = crate::Reg<reg_rx_saddr::RegRxSaddrSpec>;
#[doc = "See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
pub mod reg_rx_saddr;
#[doc = "REG_RX_SIZE (rw) register accessor: See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_size`] module"]
#[doc(alias = "REG_RX_SIZE")]
pub type RegRxSize = crate::Reg<reg_rx_size::RegRxSizeSpec>;
#[doc = "See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
pub mod reg_rx_size;
#[doc = "REG_RX_CFG (rw) register accessor: See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_cfg`] module"]
#[doc(alias = "REG_RX_CFG")]
pub type RegRxCfg = crate::Reg<reg_rx_cfg::RegRxCfgSpec>;
#[doc = "See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
pub mod reg_rx_cfg;
#[doc = "REG_CR_ADC (rw) register accessor: See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cr_adc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cr_adc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_cr_adc`] module"]
#[doc(alias = "REG_CR_ADC")]
pub type RegCrAdc = crate::Reg<reg_cr_adc::RegCrAdcSpec>;
#[doc = "See `udma_adc_ts_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ modules/ifsub/rtl/udma_adc_ts_reg_if.sv>`__"]
pub mod reg_cr_adc;
