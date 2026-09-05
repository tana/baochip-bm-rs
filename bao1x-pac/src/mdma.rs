#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_evsel_cr_evsel0: SfrEvselCrEvsel0,
    sfr_evsel_cr_evsel1: SfrEvselCrEvsel1,
    sfr_evsel_cr_evsel2: SfrEvselCrEvsel2,
    sfr_evsel_cr_evsel3: SfrEvselCrEvsel3,
    sfr_evsel_cr_evsel4: SfrEvselCrEvsel4,
    sfr_evsel_cr_evsel5: SfrEvselCrEvsel5,
    sfr_evsel_cr_evsel6: SfrEvselCrEvsel6,
    sfr_evsel_cr_evsel7: SfrEvselCrEvsel7,
    sfr_cr_cr_mdmareq0: SfrCrCrMdmareq0,
    sfr_cr_cr_mdmareq1: SfrCrCrMdmareq1,
    sfr_cr_cr_mdmareq2: SfrCrCrMdmareq2,
    sfr_cr_cr_mdmareq3: SfrCrCrMdmareq3,
    sfr_cr_cr_mdmareq4: SfrCrCrMdmareq4,
    sfr_cr_cr_mdmareq5: SfrCrCrMdmareq5,
    sfr_cr_cr_mdmareq6: SfrCrCrMdmareq6,
    sfr_cr_cr_mdmareq7: SfrCrCrMdmareq7,
    sfr_sr_sr_mdmareq0: SfrSrSrMdmareq0,
    sfr_sr_sr_mdmareq1: SfrSrSrMdmareq1,
    sfr_sr_sr_mdmareq2: SfrSrSrMdmareq2,
    sfr_sr_sr_mdmareq3: SfrSrSrMdmareq3,
    sfr_sr_sr_mdmareq4: SfrSrSrMdmareq4,
    sfr_sr_sr_mdmareq5: SfrSrSrMdmareq5,
    sfr_sr_sr_mdmareq6: SfrSrSrMdmareq6,
    sfr_sr_sr_mdmareq7: SfrSrSrMdmareq7,
}
impl RegisterBlock {
    #[doc = "0x00 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel0(&self) -> &SfrEvselCrEvsel0 {
        &self.sfr_evsel_cr_evsel0
    }
    #[doc = "0x04 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel1(&self) -> &SfrEvselCrEvsel1 {
        &self.sfr_evsel_cr_evsel1
    }
    #[doc = "0x08 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel2(&self) -> &SfrEvselCrEvsel2 {
        &self.sfr_evsel_cr_evsel2
    }
    #[doc = "0x0c - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel3(&self) -> &SfrEvselCrEvsel3 {
        &self.sfr_evsel_cr_evsel3
    }
    #[doc = "0x10 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel4(&self) -> &SfrEvselCrEvsel4 {
        &self.sfr_evsel_cr_evsel4
    }
    #[doc = "0x14 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel5(&self) -> &SfrEvselCrEvsel5 {
        &self.sfr_evsel_cr_evsel5
    }
    #[doc = "0x18 - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel6(&self) -> &SfrEvselCrEvsel6 {
        &self.sfr_evsel_cr_evsel6
    }
    #[doc = "0x1c - See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_evsel_cr_evsel7(&self) -> &SfrEvselCrEvsel7 {
        &self.sfr_evsel_cr_evsel7
    }
    #[doc = "0x20 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq0(&self) -> &SfrCrCrMdmareq0 {
        &self.sfr_cr_cr_mdmareq0
    }
    #[doc = "0x24 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq1(&self) -> &SfrCrCrMdmareq1 {
        &self.sfr_cr_cr_mdmareq1
    }
    #[doc = "0x28 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq2(&self) -> &SfrCrCrMdmareq2 {
        &self.sfr_cr_cr_mdmareq2
    }
    #[doc = "0x2c - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq3(&self) -> &SfrCrCrMdmareq3 {
        &self.sfr_cr_cr_mdmareq3
    }
    #[doc = "0x30 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq4(&self) -> &SfrCrCrMdmareq4 {
        &self.sfr_cr_cr_mdmareq4
    }
    #[doc = "0x34 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq5(&self) -> &SfrCrCrMdmareq5 {
        &self.sfr_cr_cr_mdmareq5
    }
    #[doc = "0x38 - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq6(&self) -> &SfrCrCrMdmareq6 {
        &self.sfr_cr_cr_mdmareq6
    }
    #[doc = "0x3c - See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cr_cr_mdmareq7(&self) -> &SfrCrCrMdmareq7 {
        &self.sfr_cr_cr_mdmareq7
    }
    #[doc = "0x40 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq0(&self) -> &SfrSrSrMdmareq0 {
        &self.sfr_sr_sr_mdmareq0
    }
    #[doc = "0x44 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq1(&self) -> &SfrSrSrMdmareq1 {
        &self.sfr_sr_sr_mdmareq1
    }
    #[doc = "0x48 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq2(&self) -> &SfrSrSrMdmareq2 {
        &self.sfr_sr_sr_mdmareq2
    }
    #[doc = "0x4c - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq3(&self) -> &SfrSrSrMdmareq3 {
        &self.sfr_sr_sr_mdmareq3
    }
    #[doc = "0x50 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq4(&self) -> &SfrSrSrMdmareq4 {
        &self.sfr_sr_sr_mdmareq4
    }
    #[doc = "0x54 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq5(&self) -> &SfrSrSrMdmareq5 {
        &self.sfr_sr_sr_mdmareq5
    }
    #[doc = "0x58 - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq6(&self) -> &SfrSrSrMdmareq6 {
        &self.sfr_sr_sr_mdmareq6
    }
    #[doc = "0x5c - See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sr_sr_mdmareq7(&self) -> &SfrSrSrMdmareq7 {
        &self.sfr_sr_sr_mdmareq7
    }
}
#[doc = "SFR_EVSEL_CR_EVSEL0 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel0`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL0")]
pub type SfrEvselCrEvsel0 = crate::Reg<sfr_evsel_cr_evsel0::SfrEvselCrEvsel0Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel0;
#[doc = "SFR_EVSEL_CR_EVSEL1 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel1`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL1")]
pub type SfrEvselCrEvsel1 = crate::Reg<sfr_evsel_cr_evsel1::SfrEvselCrEvsel1Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel1;
#[doc = "SFR_EVSEL_CR_EVSEL2 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel2`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL2")]
pub type SfrEvselCrEvsel2 = crate::Reg<sfr_evsel_cr_evsel2::SfrEvselCrEvsel2Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel2;
#[doc = "SFR_EVSEL_CR_EVSEL3 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel3`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL3")]
pub type SfrEvselCrEvsel3 = crate::Reg<sfr_evsel_cr_evsel3::SfrEvselCrEvsel3Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel3;
#[doc = "SFR_EVSEL_CR_EVSEL4 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel4`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL4")]
pub type SfrEvselCrEvsel4 = crate::Reg<sfr_evsel_cr_evsel4::SfrEvselCrEvsel4Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel4;
#[doc = "SFR_EVSEL_CR_EVSEL5 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel5`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL5")]
pub type SfrEvselCrEvsel5 = crate::Reg<sfr_evsel_cr_evsel5::SfrEvselCrEvsel5Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel5;
#[doc = "SFR_EVSEL_CR_EVSEL6 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel6`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL6")]
pub type SfrEvselCrEvsel6 = crate::Reg<sfr_evsel_cr_evsel6::SfrEvselCrEvsel6Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel6;
#[doc = "SFR_EVSEL_CR_EVSEL7 (rw) register accessor: See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_evsel_cr_evsel7`] module"]
#[doc(alias = "SFR_EVSEL_CR_EVSEL7")]
pub type SfrEvselCrEvsel7 = crate::Reg<sfr_evsel_cr_evsel7::SfrEvselCrEvsel7Spec>;
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_evsel_cr_evsel7;
#[doc = "SFR_CR_CR_MDMAREQ0 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq0`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ0")]
pub type SfrCrCrMdmareq0 = crate::Reg<sfr_cr_cr_mdmareq0::SfrCrCrMdmareq0Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq0;
#[doc = "SFR_CR_CR_MDMAREQ1 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq1`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ1")]
pub type SfrCrCrMdmareq1 = crate::Reg<sfr_cr_cr_mdmareq1::SfrCrCrMdmareq1Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq1;
#[doc = "SFR_CR_CR_MDMAREQ2 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq2`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ2")]
pub type SfrCrCrMdmareq2 = crate::Reg<sfr_cr_cr_mdmareq2::SfrCrCrMdmareq2Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq2;
#[doc = "SFR_CR_CR_MDMAREQ3 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq3`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ3")]
pub type SfrCrCrMdmareq3 = crate::Reg<sfr_cr_cr_mdmareq3::SfrCrCrMdmareq3Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq3;
#[doc = "SFR_CR_CR_MDMAREQ4 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq4`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ4")]
pub type SfrCrCrMdmareq4 = crate::Reg<sfr_cr_cr_mdmareq4::SfrCrCrMdmareq4Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq4;
#[doc = "SFR_CR_CR_MDMAREQ5 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq5`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ5")]
pub type SfrCrCrMdmareq5 = crate::Reg<sfr_cr_cr_mdmareq5::SfrCrCrMdmareq5Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq5;
#[doc = "SFR_CR_CR_MDMAREQ6 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq6`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ6")]
pub type SfrCrCrMdmareq6 = crate::Reg<sfr_cr_cr_mdmareq6::SfrCrCrMdmareq6Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq6;
#[doc = "SFR_CR_CR_MDMAREQ7 (rw) register accessor: See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cr_cr_mdmareq7`] module"]
#[doc(alias = "SFR_CR_CR_MDMAREQ7")]
pub type SfrCrCrMdmareq7 = crate::Reg<sfr_cr_cr_mdmareq7::SfrCrCrMdmareq7Spec>;
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)"]
pub mod sfr_cr_cr_mdmareq7;
#[doc = "SFR_SR_SR_MDMAREQ0 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq0`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ0")]
pub type SfrSrSrMdmareq0 = crate::Reg<sfr_sr_sr_mdmareq0::SfrSrSrMdmareq0Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq0;
#[doc = "SFR_SR_SR_MDMAREQ1 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq1`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ1")]
pub type SfrSrSrMdmareq1 = crate::Reg<sfr_sr_sr_mdmareq1::SfrSrSrMdmareq1Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq1;
#[doc = "SFR_SR_SR_MDMAREQ2 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq2`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ2")]
pub type SfrSrSrMdmareq2 = crate::Reg<sfr_sr_sr_mdmareq2::SfrSrSrMdmareq2Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq2;
#[doc = "SFR_SR_SR_MDMAREQ3 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq3`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ3")]
pub type SfrSrSrMdmareq3 = crate::Reg<sfr_sr_sr_mdmareq3::SfrSrSrMdmareq3Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq3;
#[doc = "SFR_SR_SR_MDMAREQ4 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq4`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ4")]
pub type SfrSrSrMdmareq4 = crate::Reg<sfr_sr_sr_mdmareq4::SfrSrSrMdmareq4Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq4;
#[doc = "SFR_SR_SR_MDMAREQ5 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq5`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ5")]
pub type SfrSrSrMdmareq5 = crate::Reg<sfr_sr_sr_mdmareq5::SfrSrSrMdmareq5Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq5;
#[doc = "SFR_SR_SR_MDMAREQ6 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq6`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ6")]
pub type SfrSrSrMdmareq6 = crate::Reg<sfr_sr_sr_mdmareq6::SfrSrSrMdmareq6Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq6;
#[doc = "SFR_SR_SR_MDMAREQ7 (rw) register accessor: See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sr_sr_mdmareq7`] module"]
#[doc(alias = "SFR_SR_SR_MDMAREQ7")]
pub type SfrSrSrMdmareq7 = crate::Reg<sfr_sr_sr_mdmareq7::SfrSrSrMdmareq7Spec>;
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sr_sr_mdmareq7;
