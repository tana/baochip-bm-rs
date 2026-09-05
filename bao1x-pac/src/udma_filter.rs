#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_tx_ch0_add: RegTxCh0Add,
    reg_tx_ch0_cfg: RegTxCh0Cfg,
    reg_tx_ch0_len0: RegTxCh0Len0,
    reg_tx_ch0_len1: RegTxCh0Len1,
    reg_tx_ch0_len2: RegTxCh0Len2,
    reg_tx_ch1_add: RegTxCh1Add,
    reg_tx_ch1_cfg: RegTxCh1Cfg,
    reg_tx_ch1_len0: RegTxCh1Len0,
    reg_tx_ch1_len1: RegTxCh1Len1,
    reg_tx_ch1_len2: RegTxCh1Len2,
    reg_rx_ch_add: RegRxChAdd,
    reg_rx_ch_cfg: RegRxChCfg,
    reg_rx_ch_len0: RegRxChLen0,
    reg_rx_ch_len1: RegRxChLen1,
    reg_rx_ch_len2: RegRxChLen2,
    reg_au_cfg: RegAuCfg,
    reg_au_reg0: RegAuReg0,
    reg_au_reg1: RegAuReg1,
    reg_bincu_th: RegBincuTh,
    reg_bincu_cnt: RegBincuCnt,
    reg_bincu_setup: RegBincuSetup,
    reg_bincu_val: RegBincuVal,
    reg_filt: RegFilt,
    _reserved23: [u8; 0x04],
    reg_status: RegStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch0_add(&self) -> &RegTxCh0Add {
        &self.reg_tx_ch0_add
    }
    #[doc = "0x04 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch0_cfg(&self) -> &RegTxCh0Cfg {
        &self.reg_tx_ch0_cfg
    }
    #[doc = "0x08 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch0_len0(&self) -> &RegTxCh0Len0 {
        &self.reg_tx_ch0_len0
    }
    #[doc = "0x0c - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch0_len1(&self) -> &RegTxCh0Len1 {
        &self.reg_tx_ch0_len1
    }
    #[doc = "0x10 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch0_len2(&self) -> &RegTxCh0Len2 {
        &self.reg_tx_ch0_len2
    }
    #[doc = "0x14 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch1_add(&self) -> &RegTxCh1Add {
        &self.reg_tx_ch1_add
    }
    #[doc = "0x18 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch1_cfg(&self) -> &RegTxCh1Cfg {
        &self.reg_tx_ch1_cfg
    }
    #[doc = "0x1c - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch1_len0(&self) -> &RegTxCh1Len0 {
        &self.reg_tx_ch1_len0
    }
    #[doc = "0x20 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch1_len1(&self) -> &RegTxCh1Len1 {
        &self.reg_tx_ch1_len1
    }
    #[doc = "0x24 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tx_ch1_len2(&self) -> &RegTxCh1Len2 {
        &self.reg_tx_ch1_len2
    }
    #[doc = "0x28 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_ch_add(&self) -> &RegRxChAdd {
        &self.reg_rx_ch_add
    }
    #[doc = "0x2c - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_ch_cfg(&self) -> &RegRxChCfg {
        &self.reg_rx_ch_cfg
    }
    #[doc = "0x30 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_ch_len0(&self) -> &RegRxChLen0 {
        &self.reg_rx_ch_len0
    }
    #[doc = "0x34 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_ch_len1(&self) -> &RegRxChLen1 {
        &self.reg_rx_ch_len1
    }
    #[doc = "0x38 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_rx_ch_len2(&self) -> &RegRxChLen2 {
        &self.reg_rx_ch_len2
    }
    #[doc = "0x3c - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_au_cfg(&self) -> &RegAuCfg {
        &self.reg_au_cfg
    }
    #[doc = "0x40 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_au_reg0(&self) -> &RegAuReg0 {
        &self.reg_au_reg0
    }
    #[doc = "0x44 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_au_reg1(&self) -> &RegAuReg1 {
        &self.reg_au_reg1
    }
    #[doc = "0x48 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_bincu_th(&self) -> &RegBincuTh {
        &self.reg_bincu_th
    }
    #[doc = "0x4c - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_bincu_cnt(&self) -> &RegBincuCnt {
        &self.reg_bincu_cnt
    }
    #[doc = "0x50 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_bincu_setup(&self) -> &RegBincuSetup {
        &self.reg_bincu_setup
    }
    #[doc = "0x54 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_bincu_val(&self) -> &RegBincuVal {
        &self.reg_bincu_val
    }
    #[doc = "0x58 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_filt(&self) -> &RegFilt {
        &self.reg_filt
    }
    #[doc = "0x60 - See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_status(&self) -> &RegStatus {
        &self.reg_status
    }
}
#[doc = "REG_TX_CH0_ADD (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch0_add`] module"]
#[doc(alias = "REG_TX_CH0_ADD")]
pub type RegTxCh0Add = crate::Reg<reg_tx_ch0_add::RegTxCh0AddSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch0_add;
#[doc = "REG_TX_CH0_CFG (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch0_cfg`] module"]
#[doc(alias = "REG_TX_CH0_CFG")]
pub type RegTxCh0Cfg = crate::Reg<reg_tx_ch0_cfg::RegTxCh0CfgSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch0_cfg;
#[doc = "REG_TX_CH0_LEN0 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_len0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_len0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch0_len0`] module"]
#[doc(alias = "REG_TX_CH0_LEN0")]
pub type RegTxCh0Len0 = crate::Reg<reg_tx_ch0_len0::RegTxCh0Len0Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch0_len0;
#[doc = "REG_TX_CH0_LEN1 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_len1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_len1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch0_len1`] module"]
#[doc(alias = "REG_TX_CH0_LEN1")]
pub type RegTxCh0Len1 = crate::Reg<reg_tx_ch0_len1::RegTxCh0Len1Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch0_len1;
#[doc = "REG_TX_CH0_LEN2 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_len2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_len2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch0_len2`] module"]
#[doc(alias = "REG_TX_CH0_LEN2")]
pub type RegTxCh0Len2 = crate::Reg<reg_tx_ch0_len2::RegTxCh0Len2Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch0_len2;
#[doc = "REG_TX_CH1_ADD (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch1_add`] module"]
#[doc(alias = "REG_TX_CH1_ADD")]
pub type RegTxCh1Add = crate::Reg<reg_tx_ch1_add::RegTxCh1AddSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch1_add;
#[doc = "REG_TX_CH1_CFG (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch1_cfg`] module"]
#[doc(alias = "REG_TX_CH1_CFG")]
pub type RegTxCh1Cfg = crate::Reg<reg_tx_ch1_cfg::RegTxCh1CfgSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch1_cfg;
#[doc = "REG_TX_CH1_LEN0 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_len0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_len0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch1_len0`] module"]
#[doc(alias = "REG_TX_CH1_LEN0")]
pub type RegTxCh1Len0 = crate::Reg<reg_tx_ch1_len0::RegTxCh1Len0Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch1_len0;
#[doc = "REG_TX_CH1_LEN1 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_len1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_len1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch1_len1`] module"]
#[doc(alias = "REG_TX_CH1_LEN1")]
pub type RegTxCh1Len1 = crate::Reg<reg_tx_ch1_len1::RegTxCh1Len1Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch1_len1;
#[doc = "REG_TX_CH1_LEN2 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_len2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_len2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tx_ch1_len2`] module"]
#[doc(alias = "REG_TX_CH1_LEN2")]
pub type RegTxCh1Len2 = crate::Reg<reg_tx_ch1_len2::RegTxCh1Len2Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_tx_ch1_len2;
#[doc = "REG_RX_CH_ADD (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_add::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_add::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_ch_add`] module"]
#[doc(alias = "REG_RX_CH_ADD")]
pub type RegRxChAdd = crate::Reg<reg_rx_ch_add::RegRxChAddSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_rx_ch_add;
#[doc = "REG_RX_CH_CFG (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_ch_cfg`] module"]
#[doc(alias = "REG_RX_CH_CFG")]
pub type RegRxChCfg = crate::Reg<reg_rx_ch_cfg::RegRxChCfgSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_rx_ch_cfg;
#[doc = "REG_RX_CH_LEN0 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_ch_len0`] module"]
#[doc(alias = "REG_RX_CH_LEN0")]
pub type RegRxChLen0 = crate::Reg<reg_rx_ch_len0::RegRxChLen0Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_rx_ch_len0;
#[doc = "REG_RX_CH_LEN1 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_ch_len1`] module"]
#[doc(alias = "REG_RX_CH_LEN1")]
pub type RegRxChLen1 = crate::Reg<reg_rx_ch_len1::RegRxChLen1Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_rx_ch_len1;
#[doc = "REG_RX_CH_LEN2 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rx_ch_len2`] module"]
#[doc(alias = "REG_RX_CH_LEN2")]
pub type RegRxChLen2 = crate::Reg<reg_rx_ch_len2::RegRxChLen2Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_rx_ch_len2;
#[doc = "REG_AU_CFG (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_au_cfg`] module"]
#[doc(alias = "REG_AU_CFG")]
pub type RegAuCfg = crate::Reg<reg_au_cfg::RegAuCfgSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_au_cfg;
#[doc = "REG_AU_REG0 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_au_reg0`] module"]
#[doc(alias = "REG_AU_REG0")]
pub type RegAuReg0 = crate::Reg<reg_au_reg0::RegAuReg0Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_au_reg0;
#[doc = "REG_AU_REG1 (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_au_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_au_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_au_reg1`] module"]
#[doc(alias = "REG_AU_REG1")]
pub type RegAuReg1 = crate::Reg<reg_au_reg1::RegAuReg1Spec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_au_reg1;
#[doc = "REG_BINCU_TH (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_bincu_th`] module"]
#[doc(alias = "REG_BINCU_TH")]
pub type RegBincuTh = crate::Reg<reg_bincu_th::RegBincuThSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_bincu_th;
#[doc = "REG_BINCU_CNT (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_bincu_cnt`] module"]
#[doc(alias = "REG_BINCU_CNT")]
pub type RegBincuCnt = crate::Reg<reg_bincu_cnt::RegBincuCntSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_bincu_cnt;
#[doc = "REG_BINCU_SETUP (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_bincu_setup`] module"]
#[doc(alias = "REG_BINCU_SETUP")]
pub type RegBincuSetup = crate::Reg<reg_bincu_setup::RegBincuSetupSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_bincu_setup;
#[doc = "REG_BINCU_VAL (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bincu_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bincu_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_bincu_val`] module"]
#[doc(alias = "REG_BINCU_VAL")]
pub type RegBincuVal = crate::Reg<reg_bincu_val::RegBincuValSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_bincu_val;
#[doc = "REG_FILT (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_filt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_filt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_filt`] module"]
#[doc(alias = "REG_FILT")]
pub type RegFilt = crate::Reg<reg_filt::RegFiltSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_filt;
#[doc = "REG_STATUS (rw) register accessor: See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_status`] module"]
#[doc(alias = "REG_STATUS")]
pub type RegStatus = crate::Reg<reg_status::RegStatusSpec>;
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__"]
pub mod reg_status;
