#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_vdmask0: SfrVdmask0,
    sfr_vdmask1: SfrVdmask1,
    sfr_vdsr: SfrVdsr,
    sfr_vdfr: SfrVdfr,
    sfr_ldmask: SfrLdmask,
    sfr_ldsr: SfrLdsr,
    sfr_ldcfg: SfrLdcfg,
    _reserved7: [u8; 0x04],
    sfr_vdcfg_cr_vdcfg0: SfrVdcfgCrVdcfg0,
    sfr_vdcfg_cr_vdcfg1: SfrVdcfgCrVdcfg1,
    sfr_vdcfg_cr_vdcfg2: SfrVdcfgCrVdcfg2,
    sfr_vdcfg_cr_vdcfg3: SfrVdcfgCrVdcfg3,
    sfr_vdcfg_cr_vdcfg4: SfrVdcfgCrVdcfg4,
    sfr_vdcfg_cr_vdcfg5: SfrVdcfgCrVdcfg5,
    sfr_vdcfg_cr_vdcfg6: SfrVdcfgCrVdcfg6,
    sfr_vdcfg_cr_vdcfg7: SfrVdcfgCrVdcfg7,
    sfr_vdip_ena: SfrVdipEna,
    sfr_vdip_test: SfrVdipTest,
    sfr_ldip_test: SfrLdipTest,
    sfr_ldip_fd: SfrLdipFd,
}
impl RegisterBlock {
    #[doc = "0x00 - See `sensorc.sv#L63 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L63>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdmask0(&self) -> &SfrVdmask0 {
        &self.sfr_vdmask0
    }
    #[doc = "0x04 - See `sensorc.sv#L64 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L64>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdmask1(&self) -> &SfrVdmask1 {
        &self.sfr_vdmask1
    }
    #[doc = "0x08 - See `sensorc.sv#L65 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L65>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdsr(&self) -> &SfrVdsr {
        &self.sfr_vdsr
    }
    #[doc = "0x0c - See `sensorc.sv#L66 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L66>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdfr(&self) -> &SfrVdfr {
        &self.sfr_vdfr
    }
    #[doc = "0x10 - See `sensorc.sv#L68 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L68>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ldmask(&self) -> &SfrLdmask {
        &self.sfr_ldmask
    }
    #[doc = "0x14 - See `sensorc.sv#L69 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L69>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ldsr(&self) -> &SfrLdsr {
        &self.sfr_ldsr
    }
    #[doc = "0x18 - See `sensorc.sv#L70 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L70>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ldcfg(&self) -> &SfrLdcfg {
        &self.sfr_ldcfg
    }
    #[doc = "0x20 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg0(&self) -> &SfrVdcfgCrVdcfg0 {
        &self.sfr_vdcfg_cr_vdcfg0
    }
    #[doc = "0x24 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg1(&self) -> &SfrVdcfgCrVdcfg1 {
        &self.sfr_vdcfg_cr_vdcfg1
    }
    #[doc = "0x28 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg2(&self) -> &SfrVdcfgCrVdcfg2 {
        &self.sfr_vdcfg_cr_vdcfg2
    }
    #[doc = "0x2c - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg3(&self) -> &SfrVdcfgCrVdcfg3 {
        &self.sfr_vdcfg_cr_vdcfg3
    }
    #[doc = "0x30 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg4(&self) -> &SfrVdcfgCrVdcfg4 {
        &self.sfr_vdcfg_cr_vdcfg4
    }
    #[doc = "0x34 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg5(&self) -> &SfrVdcfgCrVdcfg5 {
        &self.sfr_vdcfg_cr_vdcfg5
    }
    #[doc = "0x38 - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg6(&self) -> &SfrVdcfgCrVdcfg6 {
        &self.sfr_vdcfg_cr_vdcfg6
    }
    #[doc = "0x3c - See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdcfg_cr_vdcfg7(&self) -> &SfrVdcfgCrVdcfg7 {
        &self.sfr_vdcfg_cr_vdcfg7
    }
    #[doc = "0x40 - See `sensorc.sv#L74 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L74>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdip_ena(&self) -> &SfrVdipEna {
        &self.sfr_vdip_ena
    }
    #[doc = "0x44 - See `sensorc.sv#L75 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L75>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_vdip_test(&self) -> &SfrVdipTest {
        &self.sfr_vdip_test
    }
    #[doc = "0x48 - See `sensorc.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L77>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ldip_test(&self) -> &SfrLdipTest {
        &self.sfr_ldip_test
    }
    #[doc = "0x4c - See `sensorc.sv#L78 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L78>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ldip_fd(&self) -> &SfrLdipFd {
        &self.sfr_ldip_fd
    }
}
#[doc = "SFR_VDMASK0 (rw) register accessor: See `sensorc.sv#L63 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L63>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdmask0`] module"]
#[doc(alias = "SFR_VDMASK0")]
pub type SfrVdmask0 = crate::Reg<sfr_vdmask0::SfrVdmask0Spec>;
#[doc = "See `sensorc.sv#L63 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L63>`__ (line numbers are approximate)"]
pub mod sfr_vdmask0;
#[doc = "SFR_VDMASK1 (rw) register accessor: See `sensorc.sv#L64 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L64>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdmask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdmask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdmask1`] module"]
#[doc(alias = "SFR_VDMASK1")]
pub type SfrVdmask1 = crate::Reg<sfr_vdmask1::SfrVdmask1Spec>;
#[doc = "See `sensorc.sv#L64 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L64>`__ (line numbers are approximate)"]
pub mod sfr_vdmask1;
#[doc = "SFR_VDSR (rw) register accessor: See `sensorc.sv#L65 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L65>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdsr`] module"]
#[doc(alias = "SFR_VDSR")]
pub type SfrVdsr = crate::Reg<sfr_vdsr::SfrVdsrSpec>;
#[doc = "See `sensorc.sv#L65 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L65>`__ (line numbers are approximate)"]
pub mod sfr_vdsr;
#[doc = "SFR_VDFR (rw) register accessor: See `sensorc.sv#L66 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L66>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdfr`] module"]
#[doc(alias = "SFR_VDFR")]
pub type SfrVdfr = crate::Reg<sfr_vdfr::SfrVdfrSpec>;
#[doc = "See `sensorc.sv#L66 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L66>`__ (line numbers are approximate)"]
pub mod sfr_vdfr;
#[doc = "SFR_LDMASK (rw) register accessor: See `sensorc.sv#L68 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L68>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ldmask`] module"]
#[doc(alias = "SFR_LDMASK")]
pub type SfrLdmask = crate::Reg<sfr_ldmask::SfrLdmaskSpec>;
#[doc = "See `sensorc.sv#L68 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L68>`__ (line numbers are approximate)"]
pub mod sfr_ldmask;
#[doc = "SFR_LDSR (rw) register accessor: See `sensorc.sv#L69 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L69>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ldsr`] module"]
#[doc(alias = "SFR_LDSR")]
pub type SfrLdsr = crate::Reg<sfr_ldsr::SfrLdsrSpec>;
#[doc = "See `sensorc.sv#L69 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L69>`__ (line numbers are approximate)"]
pub mod sfr_ldsr;
#[doc = "SFR_LDCFG (rw) register accessor: See `sensorc.sv#L70 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L70>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ldcfg`] module"]
#[doc(alias = "SFR_LDCFG")]
pub type SfrLdcfg = crate::Reg<sfr_ldcfg::SfrLdcfgSpec>;
#[doc = "See `sensorc.sv#L70 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L70>`__ (line numbers are approximate)"]
pub mod sfr_ldcfg;
#[doc = "SFR_VDCFG_CR_VDCFG0 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg0`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG0")]
pub type SfrVdcfgCrVdcfg0 = crate::Reg<sfr_vdcfg_cr_vdcfg0::SfrVdcfgCrVdcfg0Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg0;
#[doc = "SFR_VDCFG_CR_VDCFG1 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg1`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG1")]
pub type SfrVdcfgCrVdcfg1 = crate::Reg<sfr_vdcfg_cr_vdcfg1::SfrVdcfgCrVdcfg1Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg1;
#[doc = "SFR_VDCFG_CR_VDCFG2 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg2`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG2")]
pub type SfrVdcfgCrVdcfg2 = crate::Reg<sfr_vdcfg_cr_vdcfg2::SfrVdcfgCrVdcfg2Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg2;
#[doc = "SFR_VDCFG_CR_VDCFG3 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg3`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG3")]
pub type SfrVdcfgCrVdcfg3 = crate::Reg<sfr_vdcfg_cr_vdcfg3::SfrVdcfgCrVdcfg3Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg3;
#[doc = "SFR_VDCFG_CR_VDCFG4 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg4`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG4")]
pub type SfrVdcfgCrVdcfg4 = crate::Reg<sfr_vdcfg_cr_vdcfg4::SfrVdcfgCrVdcfg4Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg4;
#[doc = "SFR_VDCFG_CR_VDCFG5 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg5`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG5")]
pub type SfrVdcfgCrVdcfg5 = crate::Reg<sfr_vdcfg_cr_vdcfg5::SfrVdcfgCrVdcfg5Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg5;
#[doc = "SFR_VDCFG_CR_VDCFG6 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg6`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG6")]
pub type SfrVdcfgCrVdcfg6 = crate::Reg<sfr_vdcfg_cr_vdcfg6::SfrVdcfgCrVdcfg6Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg6;
#[doc = "SFR_VDCFG_CR_VDCFG7 (rw) register accessor: See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdcfg_cr_vdcfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdcfg_cr_vdcfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdcfg_cr_vdcfg7`] module"]
#[doc(alias = "SFR_VDCFG_CR_VDCFG7")]
pub type SfrVdcfgCrVdcfg7 = crate::Reg<sfr_vdcfg_cr_vdcfg7::SfrVdcfgCrVdcfg7Spec>;
#[doc = "See `sensorc.sv#L72 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L72>`__ (line numbers are approximate)"]
pub mod sfr_vdcfg_cr_vdcfg7;
#[doc = "SFR_VDIP_ENA (rw) register accessor: See `sensorc.sv#L74 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L74>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdip_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdip_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdip_ena`] module"]
#[doc(alias = "SFR_VDIP_ENA")]
pub type SfrVdipEna = crate::Reg<sfr_vdip_ena::SfrVdipEnaSpec>;
#[doc = "See `sensorc.sv#L74 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L74>`__ (line numbers are approximate)"]
pub mod sfr_vdip_ena;
#[doc = "SFR_VDIP_TEST (rw) register accessor: See `sensorc.sv#L75 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L75>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdip_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdip_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_vdip_test`] module"]
#[doc(alias = "SFR_VDIP_TEST")]
pub type SfrVdipTest = crate::Reg<sfr_vdip_test::SfrVdipTestSpec>;
#[doc = "See `sensorc.sv#L75 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L75>`__ (line numbers are approximate)"]
pub mod sfr_vdip_test;
#[doc = "SFR_LDIP_TEST (rw) register accessor: See `sensorc.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L77>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldip_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldip_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ldip_test`] module"]
#[doc(alias = "SFR_LDIP_TEST")]
pub type SfrLdipTest = crate::Reg<sfr_ldip_test::SfrLdipTestSpec>;
#[doc = "See `sensorc.sv#L77 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L77>`__ (line numbers are approximate)"]
pub mod sfr_ldip_test;
#[doc = "SFR_LDIP_FD (rw) register accessor: See `sensorc.sv#L78 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L78>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ldip_fd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ldip_fd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ldip_fd`] module"]
#[doc(alias = "SFR_LDIP_FD")]
pub type SfrLdipFd = crate::Reg<sfr_ldip_fd::SfrLdipFdSpec>;
#[doc = "See `sensorc.sv#L78 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L78>`__ (line numbers are approximate)"]
pub mod sfr_ldip_fd;
