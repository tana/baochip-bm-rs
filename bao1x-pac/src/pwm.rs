#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_tim0_cmd: RegTim0Cmd,
    reg_tim0_cfg: RegTim0Cfg,
    _reserved2: [u8; 0x04],
    reg_tim0_ch0_th: RegTim0Ch0Th,
    reg_tim0_ch1_th: RegTim0Ch1Th,
    reg_tim0_ch2_th: RegTim0Ch2Th,
    reg_tim0_ch3_th: RegTim0Ch3Th,
    reg_tim0_ch0_lut: RegTim0Ch0Lut,
    reg_tim0_ch1_lut: RegTim0Ch1Lut,
    reg_tim0_ch2_lut: RegTim0Ch2Lut,
    reg_tim0_ch3_lut: RegTim0Ch3Lut,
    _reserved10: [u8; 0x14],
    reg_tim1_cmd: RegTim1Cmd,
    reg_tim1_cfg: RegTim1Cfg,
    _reserved12: [u8; 0x04],
    reg_tim1_ch0_th: RegTim1Ch0Th,
    reg_tim1_ch1_th: RegTim1Ch1Th,
    reg_tim1_ch2_th: RegTim1Ch2Th,
    reg_tim1_ch3_th: RegTim1Ch3Th,
    reg_tim1_ch0_lut: RegTim1Ch0Lut,
    reg_tim1_ch1_lut: RegTim1Ch1Lut,
    reg_tim1_ch2_lut: RegTim1Ch2Lut,
    reg_tim1_ch3_lut: RegTim1Ch3Lut,
    _reserved20: [u8; 0x14],
    reg_tim2_cmd: RegTim2Cmd,
    reg_tim2_cfg: RegTim2Cfg,
    _reserved22: [u8; 0x04],
    reg_tim2_ch0_th: RegTim2Ch0Th,
    reg_tim2_ch1_th: RegTim2Ch1Th,
    reg_tim2_ch2_th: RegTim2Ch2Th,
    reg_tim2_ch3_th: RegTim2Ch3Th,
    reg_tim2_ch0_lut: RegTim2Ch0Lut,
    reg_tim2_ch1_lut: RegTim2Ch1Lut,
    reg_tim2_ch2_lut: RegTim2Ch2Lut,
    reg_tim2_ch3_lut: RegTim2Ch3Lut,
    _reserved30: [u8; 0x14],
    reg_tim3_cmd: RegTim3Cmd,
    reg_tim3_cfg: RegTim3Cfg,
    _reserved32: [u8; 0x04],
    reg_tim3_ch0_th: RegTim3Ch0Th,
    reg_tim3_ch1_th: RegTim3Ch1Th,
    reg_tim3_ch2_th: RegTim3Ch2Th,
    reg_tim3_ch3_th: RegTim3Ch3Th,
    reg_tim3_ch0_lut: RegTim3Ch0Lut,
    reg_tim3_ch1_lut: RegTim3Ch1Lut,
    reg_tim3_ch2_lut: RegTim3Ch2Lut,
    reg_tim3_ch3_lut: RegTim3Ch3Lut,
    _reserved40: [u8; 0x14],
    reg_event_cfg: RegEventCfg,
    reg_ch_en: RegChEn,
    _reserved42: [u8; 0x38],
    reg_prefd0: RegPrefd0,
    reg_prefd1: RegPrefd1,
    reg_prefd2: RegPrefd2,
    reg_prefd3: RegPrefd3,
}
impl RegisterBlock {
    #[doc = "0x00 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_cmd(&self) -> &RegTim0Cmd {
        &self.reg_tim0_cmd
    }
    #[doc = "0x04 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_cfg(&self) -> &RegTim0Cfg {
        &self.reg_tim0_cfg
    }
    #[doc = "0x0c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch0_th(&self) -> &RegTim0Ch0Th {
        &self.reg_tim0_ch0_th
    }
    #[doc = "0x10 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch1_th(&self) -> &RegTim0Ch1Th {
        &self.reg_tim0_ch1_th
    }
    #[doc = "0x14 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch2_th(&self) -> &RegTim0Ch2Th {
        &self.reg_tim0_ch2_th
    }
    #[doc = "0x18 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch3_th(&self) -> &RegTim0Ch3Th {
        &self.reg_tim0_ch3_th
    }
    #[doc = "0x1c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch0_lut(&self) -> &RegTim0Ch0Lut {
        &self.reg_tim0_ch0_lut
    }
    #[doc = "0x20 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch1_lut(&self) -> &RegTim0Ch1Lut {
        &self.reg_tim0_ch1_lut
    }
    #[doc = "0x24 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch2_lut(&self) -> &RegTim0Ch2Lut {
        &self.reg_tim0_ch2_lut
    }
    #[doc = "0x28 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim0_ch3_lut(&self) -> &RegTim0Ch3Lut {
        &self.reg_tim0_ch3_lut
    }
    #[doc = "0x40 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_cmd(&self) -> &RegTim1Cmd {
        &self.reg_tim1_cmd
    }
    #[doc = "0x44 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_cfg(&self) -> &RegTim1Cfg {
        &self.reg_tim1_cfg
    }
    #[doc = "0x4c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch0_th(&self) -> &RegTim1Ch0Th {
        &self.reg_tim1_ch0_th
    }
    #[doc = "0x50 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch1_th(&self) -> &RegTim1Ch1Th {
        &self.reg_tim1_ch1_th
    }
    #[doc = "0x54 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch2_th(&self) -> &RegTim1Ch2Th {
        &self.reg_tim1_ch2_th
    }
    #[doc = "0x58 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch3_th(&self) -> &RegTim1Ch3Th {
        &self.reg_tim1_ch3_th
    }
    #[doc = "0x5c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch0_lut(&self) -> &RegTim1Ch0Lut {
        &self.reg_tim1_ch0_lut
    }
    #[doc = "0x60 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch1_lut(&self) -> &RegTim1Ch1Lut {
        &self.reg_tim1_ch1_lut
    }
    #[doc = "0x64 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch2_lut(&self) -> &RegTim1Ch2Lut {
        &self.reg_tim1_ch2_lut
    }
    #[doc = "0x68 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim1_ch3_lut(&self) -> &RegTim1Ch3Lut {
        &self.reg_tim1_ch3_lut
    }
    #[doc = "0x80 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_cmd(&self) -> &RegTim2Cmd {
        &self.reg_tim2_cmd
    }
    #[doc = "0x84 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_cfg(&self) -> &RegTim2Cfg {
        &self.reg_tim2_cfg
    }
    #[doc = "0x8c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch0_th(&self) -> &RegTim2Ch0Th {
        &self.reg_tim2_ch0_th
    }
    #[doc = "0x90 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch1_th(&self) -> &RegTim2Ch1Th {
        &self.reg_tim2_ch1_th
    }
    #[doc = "0x94 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch2_th(&self) -> &RegTim2Ch2Th {
        &self.reg_tim2_ch2_th
    }
    #[doc = "0x98 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch3_th(&self) -> &RegTim2Ch3Th {
        &self.reg_tim2_ch3_th
    }
    #[doc = "0x9c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch0_lut(&self) -> &RegTim2Ch0Lut {
        &self.reg_tim2_ch0_lut
    }
    #[doc = "0xa0 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch1_lut(&self) -> &RegTim2Ch1Lut {
        &self.reg_tim2_ch1_lut
    }
    #[doc = "0xa4 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch2_lut(&self) -> &RegTim2Ch2Lut {
        &self.reg_tim2_ch2_lut
    }
    #[doc = "0xa8 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim2_ch3_lut(&self) -> &RegTim2Ch3Lut {
        &self.reg_tim2_ch3_lut
    }
    #[doc = "0xc0 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_cmd(&self) -> &RegTim3Cmd {
        &self.reg_tim3_cmd
    }
    #[doc = "0xc4 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_cfg(&self) -> &RegTim3Cfg {
        &self.reg_tim3_cfg
    }
    #[doc = "0xcc - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch0_th(&self) -> &RegTim3Ch0Th {
        &self.reg_tim3_ch0_th
    }
    #[doc = "0xd0 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch1_th(&self) -> &RegTim3Ch1Th {
        &self.reg_tim3_ch1_th
    }
    #[doc = "0xd4 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch2_th(&self) -> &RegTim3Ch2Th {
        &self.reg_tim3_ch2_th
    }
    #[doc = "0xd8 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch3_th(&self) -> &RegTim3Ch3Th {
        &self.reg_tim3_ch3_th
    }
    #[doc = "0xdc - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch0_lut(&self) -> &RegTim3Ch0Lut {
        &self.reg_tim3_ch0_lut
    }
    #[doc = "0xe0 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch1_lut(&self) -> &RegTim3Ch1Lut {
        &self.reg_tim3_ch1_lut
    }
    #[doc = "0xe4 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch2_lut(&self) -> &RegTim3Ch2Lut {
        &self.reg_tim3_ch2_lut
    }
    #[doc = "0xe8 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_tim3_ch3_lut(&self) -> &RegTim3Ch3Lut {
        &self.reg_tim3_ch3_lut
    }
    #[doc = "0x100 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_event_cfg(&self) -> &RegEventCfg {
        &self.reg_event_cfg
    }
    #[doc = "0x104 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_ch_en(&self) -> &RegChEn {
        &self.reg_ch_en
    }
    #[doc = "0x140 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_prefd0(&self) -> &RegPrefd0 {
        &self.reg_prefd0
    }
    #[doc = "0x144 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_prefd1(&self) -> &RegPrefd1 {
        &self.reg_prefd1
    }
    #[doc = "0x148 - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_prefd2(&self) -> &RegPrefd2 {
        &self.reg_prefd2
    }
    #[doc = "0x14c - See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
    #[inline(always)]
    pub const fn reg_prefd3(&self) -> &RegPrefd3 {
        &self.reg_prefd3
    }
}
#[doc = "REG_TIM0_CMD (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_cmd`] module"]
#[doc(alias = "REG_TIM0_CMD")]
pub type RegTim0Cmd = crate::Reg<reg_tim0_cmd::RegTim0CmdSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_cmd;
#[doc = "REG_TIM0_CFG (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_cfg`] module"]
#[doc(alias = "REG_TIM0_CFG")]
pub type RegTim0Cfg = crate::Reg<reg_tim0_cfg::RegTim0CfgSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_cfg;
#[doc = "REG_TIM0_CH0_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch0_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch0_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch0_th`] module"]
#[doc(alias = "REG_TIM0_CH0_TH")]
pub type RegTim0Ch0Th = crate::Reg<reg_tim0_ch0_th::RegTim0Ch0ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch0_th;
#[doc = "REG_TIM0_CH1_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch1_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch1_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch1_th`] module"]
#[doc(alias = "REG_TIM0_CH1_TH")]
pub type RegTim0Ch1Th = crate::Reg<reg_tim0_ch1_th::RegTim0Ch1ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch1_th;
#[doc = "REG_TIM0_CH2_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch2_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch2_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch2_th`] module"]
#[doc(alias = "REG_TIM0_CH2_TH")]
pub type RegTim0Ch2Th = crate::Reg<reg_tim0_ch2_th::RegTim0Ch2ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch2_th;
#[doc = "REG_TIM0_CH3_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch3_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch3_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch3_th`] module"]
#[doc(alias = "REG_TIM0_CH3_TH")]
pub type RegTim0Ch3Th = crate::Reg<reg_tim0_ch3_th::RegTim0Ch3ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch3_th;
#[doc = "REG_TIM0_CH0_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch0_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch0_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch0_lut`] module"]
#[doc(alias = "REG_TIM0_CH0_LUT")]
pub type RegTim0Ch0Lut = crate::Reg<reg_tim0_ch0_lut::RegTim0Ch0LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch0_lut;
#[doc = "REG_TIM0_CH1_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch1_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch1_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch1_lut`] module"]
#[doc(alias = "REG_TIM0_CH1_LUT")]
pub type RegTim0Ch1Lut = crate::Reg<reg_tim0_ch1_lut::RegTim0Ch1LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch1_lut;
#[doc = "REG_TIM0_CH2_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch2_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch2_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch2_lut`] module"]
#[doc(alias = "REG_TIM0_CH2_LUT")]
pub type RegTim0Ch2Lut = crate::Reg<reg_tim0_ch2_lut::RegTim0Ch2LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch2_lut;
#[doc = "REG_TIM0_CH3_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim0_ch3_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim0_ch3_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim0_ch3_lut`] module"]
#[doc(alias = "REG_TIM0_CH3_LUT")]
pub type RegTim0Ch3Lut = crate::Reg<reg_tim0_ch3_lut::RegTim0Ch3LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim0_ch3_lut;
#[doc = "REG_TIM1_CMD (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_cmd`] module"]
#[doc(alias = "REG_TIM1_CMD")]
pub type RegTim1Cmd = crate::Reg<reg_tim1_cmd::RegTim1CmdSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_cmd;
#[doc = "REG_TIM1_CFG (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_cfg`] module"]
#[doc(alias = "REG_TIM1_CFG")]
pub type RegTim1Cfg = crate::Reg<reg_tim1_cfg::RegTim1CfgSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_cfg;
#[doc = "REG_TIM1_CH0_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch0_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch0_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch0_th`] module"]
#[doc(alias = "REG_TIM1_CH0_TH")]
pub type RegTim1Ch0Th = crate::Reg<reg_tim1_ch0_th::RegTim1Ch0ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch0_th;
#[doc = "REG_TIM1_CH1_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch1_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch1_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch1_th`] module"]
#[doc(alias = "REG_TIM1_CH1_TH")]
pub type RegTim1Ch1Th = crate::Reg<reg_tim1_ch1_th::RegTim1Ch1ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch1_th;
#[doc = "REG_TIM1_CH2_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch2_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch2_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch2_th`] module"]
#[doc(alias = "REG_TIM1_CH2_TH")]
pub type RegTim1Ch2Th = crate::Reg<reg_tim1_ch2_th::RegTim1Ch2ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch2_th;
#[doc = "REG_TIM1_CH3_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch3_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch3_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch3_th`] module"]
#[doc(alias = "REG_TIM1_CH3_TH")]
pub type RegTim1Ch3Th = crate::Reg<reg_tim1_ch3_th::RegTim1Ch3ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch3_th;
#[doc = "REG_TIM1_CH0_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch0_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch0_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch0_lut`] module"]
#[doc(alias = "REG_TIM1_CH0_LUT")]
pub type RegTim1Ch0Lut = crate::Reg<reg_tim1_ch0_lut::RegTim1Ch0LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch0_lut;
#[doc = "REG_TIM1_CH1_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch1_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch1_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch1_lut`] module"]
#[doc(alias = "REG_TIM1_CH1_LUT")]
pub type RegTim1Ch1Lut = crate::Reg<reg_tim1_ch1_lut::RegTim1Ch1LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch1_lut;
#[doc = "REG_TIM1_CH2_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch2_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch2_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch2_lut`] module"]
#[doc(alias = "REG_TIM1_CH2_LUT")]
pub type RegTim1Ch2Lut = crate::Reg<reg_tim1_ch2_lut::RegTim1Ch2LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch2_lut;
#[doc = "REG_TIM1_CH3_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim1_ch3_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim1_ch3_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim1_ch3_lut`] module"]
#[doc(alias = "REG_TIM1_CH3_LUT")]
pub type RegTim1Ch3Lut = crate::Reg<reg_tim1_ch3_lut::RegTim1Ch3LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim1_ch3_lut;
#[doc = "REG_TIM2_CMD (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_cmd`] module"]
#[doc(alias = "REG_TIM2_CMD")]
pub type RegTim2Cmd = crate::Reg<reg_tim2_cmd::RegTim2CmdSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_cmd;
#[doc = "REG_TIM2_CFG (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_cfg`] module"]
#[doc(alias = "REG_TIM2_CFG")]
pub type RegTim2Cfg = crate::Reg<reg_tim2_cfg::RegTim2CfgSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_cfg;
#[doc = "REG_TIM2_CH0_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch0_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch0_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch0_th`] module"]
#[doc(alias = "REG_TIM2_CH0_TH")]
pub type RegTim2Ch0Th = crate::Reg<reg_tim2_ch0_th::RegTim2Ch0ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch0_th;
#[doc = "REG_TIM2_CH1_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch1_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch1_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch1_th`] module"]
#[doc(alias = "REG_TIM2_CH1_TH")]
pub type RegTim2Ch1Th = crate::Reg<reg_tim2_ch1_th::RegTim2Ch1ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch1_th;
#[doc = "REG_TIM2_CH2_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch2_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch2_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch2_th`] module"]
#[doc(alias = "REG_TIM2_CH2_TH")]
pub type RegTim2Ch2Th = crate::Reg<reg_tim2_ch2_th::RegTim2Ch2ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch2_th;
#[doc = "REG_TIM2_CH3_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch3_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch3_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch3_th`] module"]
#[doc(alias = "REG_TIM2_CH3_TH")]
pub type RegTim2Ch3Th = crate::Reg<reg_tim2_ch3_th::RegTim2Ch3ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch3_th;
#[doc = "REG_TIM2_CH0_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch0_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch0_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch0_lut`] module"]
#[doc(alias = "REG_TIM2_CH0_LUT")]
pub type RegTim2Ch0Lut = crate::Reg<reg_tim2_ch0_lut::RegTim2Ch0LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch0_lut;
#[doc = "REG_TIM2_CH1_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch1_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch1_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch1_lut`] module"]
#[doc(alias = "REG_TIM2_CH1_LUT")]
pub type RegTim2Ch1Lut = crate::Reg<reg_tim2_ch1_lut::RegTim2Ch1LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch1_lut;
#[doc = "REG_TIM2_CH2_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch2_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch2_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch2_lut`] module"]
#[doc(alias = "REG_TIM2_CH2_LUT")]
pub type RegTim2Ch2Lut = crate::Reg<reg_tim2_ch2_lut::RegTim2Ch2LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch2_lut;
#[doc = "REG_TIM2_CH3_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim2_ch3_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim2_ch3_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim2_ch3_lut`] module"]
#[doc(alias = "REG_TIM2_CH3_LUT")]
pub type RegTim2Ch3Lut = crate::Reg<reg_tim2_ch3_lut::RegTim2Ch3LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim2_ch3_lut;
#[doc = "REG_TIM3_CMD (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_cmd`] module"]
#[doc(alias = "REG_TIM3_CMD")]
pub type RegTim3Cmd = crate::Reg<reg_tim3_cmd::RegTim3CmdSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_cmd;
#[doc = "REG_TIM3_CFG (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_cfg`] module"]
#[doc(alias = "REG_TIM3_CFG")]
pub type RegTim3Cfg = crate::Reg<reg_tim3_cfg::RegTim3CfgSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_cfg;
#[doc = "REG_TIM3_CH0_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch0_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch0_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch0_th`] module"]
#[doc(alias = "REG_TIM3_CH0_TH")]
pub type RegTim3Ch0Th = crate::Reg<reg_tim3_ch0_th::RegTim3Ch0ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch0_th;
#[doc = "REG_TIM3_CH1_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch1_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch1_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch1_th`] module"]
#[doc(alias = "REG_TIM3_CH1_TH")]
pub type RegTim3Ch1Th = crate::Reg<reg_tim3_ch1_th::RegTim3Ch1ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch1_th;
#[doc = "REG_TIM3_CH2_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch2_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch2_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch2_th`] module"]
#[doc(alias = "REG_TIM3_CH2_TH")]
pub type RegTim3Ch2Th = crate::Reg<reg_tim3_ch2_th::RegTim3Ch2ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch2_th;
#[doc = "REG_TIM3_CH3_TH (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch3_th::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch3_th::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch3_th`] module"]
#[doc(alias = "REG_TIM3_CH3_TH")]
pub type RegTim3Ch3Th = crate::Reg<reg_tim3_ch3_th::RegTim3Ch3ThSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch3_th;
#[doc = "REG_TIM3_CH0_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch0_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch0_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch0_lut`] module"]
#[doc(alias = "REG_TIM3_CH0_LUT")]
pub type RegTim3Ch0Lut = crate::Reg<reg_tim3_ch0_lut::RegTim3Ch0LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch0_lut;
#[doc = "REG_TIM3_CH1_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch1_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch1_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch1_lut`] module"]
#[doc(alias = "REG_TIM3_CH1_LUT")]
pub type RegTim3Ch1Lut = crate::Reg<reg_tim3_ch1_lut::RegTim3Ch1LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch1_lut;
#[doc = "REG_TIM3_CH2_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch2_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch2_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch2_lut`] module"]
#[doc(alias = "REG_TIM3_CH2_LUT")]
pub type RegTim3Ch2Lut = crate::Reg<reg_tim3_ch2_lut::RegTim3Ch2LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch2_lut;
#[doc = "REG_TIM3_CH3_LUT (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tim3_ch3_lut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tim3_ch3_lut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_tim3_ch3_lut`] module"]
#[doc(alias = "REG_TIM3_CH3_LUT")]
pub type RegTim3Ch3Lut = crate::Reg<reg_tim3_ch3_lut::RegTim3Ch3LutSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_tim3_ch3_lut;
#[doc = "REG_EVENT_CFG (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_event_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_event_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_event_cfg`] module"]
#[doc(alias = "REG_EVENT_CFG")]
pub type RegEventCfg = crate::Reg<reg_event_cfg::RegEventCfgSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_event_cfg;
#[doc = "REG_CH_EN (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_ch_en`] module"]
#[doc(alias = "REG_CH_EN")]
pub type RegChEn = crate::Reg<reg_ch_en::RegChEnSpec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_ch_en;
#[doc = "REG_PREFD0 (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_prefd0`] module"]
#[doc(alias = "REG_PREFD0")]
pub type RegPrefd0 = crate::Reg<reg_prefd0::RegPrefd0Spec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_prefd0;
#[doc = "REG_PREFD1 (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_prefd1`] module"]
#[doc(alias = "REG_PREFD1")]
pub type RegPrefd1 = crate::Reg<reg_prefd1::RegPrefd1Spec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_prefd1;
#[doc = "REG_PREFD2 (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_prefd2`] module"]
#[doc(alias = "REG_PREFD2")]
pub type RegPrefd2 = crate::Reg<reg_prefd2::RegPrefd2Spec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_prefd2;
#[doc = "REG_PREFD3 (rw) register accessor: See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_prefd3`] module"]
#[doc(alias = "REG_PREFD3")]
pub type RegPrefd3 = crate::Reg<reg_prefd3::RegPrefd3Spec>;
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__"]
pub mod reg_prefd3;
