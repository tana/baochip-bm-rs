#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_cfg0: SfrCfg0,
    sfr_cfg1: SfrCfg1,
    sfr_cfg2: SfrCfg2,
    sfr_cfg3: SfrCfg3,
    sfr_sr0: SfrSr0,
    sfr_sr1: SfrSr1,
    _reserved6: [u8; 0x18],
    sfr_cfg4: SfrCfg4,
}
impl RegisterBlock {
    #[doc = "0x00 - See `dkpc.sv#L167 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L167>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg0(&self) -> &SfrCfg0 {
        &self.sfr_cfg0
    }
    #[doc = "0x04 - See `dkpc.sv#L168 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L168>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg1(&self) -> &SfrCfg1 {
        &self.sfr_cfg1
    }
    #[doc = "0x08 - See `dkpc.sv#L169 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L169>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg2(&self) -> &SfrCfg2 {
        &self.sfr_cfg2
    }
    #[doc = "0x0c - See `dkpc.sv#L170 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L170>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg3(&self) -> &SfrCfg3 {
        &self.sfr_cfg3
    }
    #[doc = "0x10 - See `dkpc.sv#L173 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L173>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr0(&self) -> &SfrSr0 {
        &self.sfr_sr0
    }
    #[doc = "0x14 - See `dkpc.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L174>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr1(&self) -> &SfrSr1 {
        &self.sfr_sr1
    }
    #[doc = "0x30 - See `dkpc.sv#L171 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L171>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cfg4(&self) -> &SfrCfg4 {
        &self.sfr_cfg4
    }
}
#[doc = "SFR_CFG0 (rw) register accessor: See `dkpc.sv#L167 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L167>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg0`] module"]
#[doc(alias = "SFR_CFG0")]
pub type SfrCfg0 = crate::Reg<sfr_cfg0::SfrCfg0Spec>;
#[doc = "See `dkpc.sv#L167 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L167>`__ (line numbers are approximate)"]
pub mod sfr_cfg0;
#[doc = "SFR_CFG1 (rw) register accessor: See `dkpc.sv#L168 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L168>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg1`] module"]
#[doc(alias = "SFR_CFG1")]
pub type SfrCfg1 = crate::Reg<sfr_cfg1::SfrCfg1Spec>;
#[doc = "See `dkpc.sv#L168 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L168>`__ (line numbers are approximate)"]
pub mod sfr_cfg1;
#[doc = "SFR_CFG2 (rw) register accessor: See `dkpc.sv#L169 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L169>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg2`] module"]
#[doc(alias = "SFR_CFG2")]
pub type SfrCfg2 = crate::Reg<sfr_cfg2::SfrCfg2Spec>;
#[doc = "See `dkpc.sv#L169 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L169>`__ (line numbers are approximate)"]
pub mod sfr_cfg2;
#[doc = "SFR_CFG3 (rw) register accessor: See `dkpc.sv#L170 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L170>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg3`] module"]
#[doc(alias = "SFR_CFG3")]
pub type SfrCfg3 = crate::Reg<sfr_cfg3::SfrCfg3Spec>;
#[doc = "See `dkpc.sv#L170 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L170>`__ (line numbers are approximate)"]
pub mod sfr_cfg3;
#[doc = "SFR_SR0 (rw) register accessor: See `dkpc.sv#L173 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L173>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr0`] module"]
#[doc(alias = "SFR_SR0")]
pub type SfrSr0 = crate::Reg<sfr_sr0::SfrSr0Spec>;
#[doc = "See `dkpc.sv#L173 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L173>`__ (line numbers are approximate)"]
pub mod sfr_sr0;
#[doc = "SFR_SR1 (rw) register accessor: See `dkpc.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L174>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr1`] module"]
#[doc(alias = "SFR_SR1")]
pub type SfrSr1 = crate::Reg<sfr_sr1::SfrSr1Spec>;
#[doc = "See `dkpc.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L174>`__ (line numbers are approximate)"]
pub mod sfr_sr1;
#[doc = "SFR_CFG4 (rw) register accessor: See `dkpc.sv#L171 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L171>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cfg4`] module"]
#[doc(alias = "SFR_CFG4")]
pub type SfrCfg4 = crate::Reg<sfr_cfg4::SfrCfg4Spec>;
#[doc = "See `dkpc.sv#L171 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L171>`__ (line numbers are approximate)"]
pub mod sfr_cfg4;
