#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_bureg_cr_buregs0: SfrBuregCrBuregs0,
    sfr_bureg_cr_buregs1: SfrBuregCrBuregs1,
    sfr_bureg_cr_buregs2: SfrBuregCrBuregs2,
    sfr_bureg_cr_buregs3: SfrBuregCrBuregs3,
    sfr_bureg_cr_buregs4: SfrBuregCrBuregs4,
    sfr_bureg_cr_buregs5: SfrBuregCrBuregs5,
    sfr_bureg_cr_buregs6: SfrBuregCrBuregs6,
    sfr_bureg_cr_buregs7: SfrBuregCrBuregs7,
}
impl RegisterBlock {
    #[doc = "0x00 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs0(&self) -> &SfrBuregCrBuregs0 {
        &self.sfr_bureg_cr_buregs0
    }
    #[doc = "0x04 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs1(&self) -> &SfrBuregCrBuregs1 {
        &self.sfr_bureg_cr_buregs1
    }
    #[doc = "0x08 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs2(&self) -> &SfrBuregCrBuregs2 {
        &self.sfr_bureg_cr_buregs2
    }
    #[doc = "0x0c - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs3(&self) -> &SfrBuregCrBuregs3 {
        &self.sfr_bureg_cr_buregs3
    }
    #[doc = "0x10 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs4(&self) -> &SfrBuregCrBuregs4 {
        &self.sfr_bureg_cr_buregs4
    }
    #[doc = "0x14 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs5(&self) -> &SfrBuregCrBuregs5 {
        &self.sfr_bureg_cr_buregs5
    }
    #[doc = "0x18 - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs6(&self) -> &SfrBuregCrBuregs6 {
        &self.sfr_bureg_cr_buregs6
    }
    #[doc = "0x1c - See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_bureg_cr_buregs7(&self) -> &SfrBuregCrBuregs7 {
        &self.sfr_bureg_cr_buregs7
    }
}
#[doc = "SFR_BUREG_CR_BUREGS0 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs0`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS0")]
pub type SfrBuregCrBuregs0 = crate::Reg<sfr_bureg_cr_buregs0::SfrBuregCrBuregs0Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs0;
#[doc = "SFR_BUREG_CR_BUREGS1 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs1`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS1")]
pub type SfrBuregCrBuregs1 = crate::Reg<sfr_bureg_cr_buregs1::SfrBuregCrBuregs1Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs1;
#[doc = "SFR_BUREG_CR_BUREGS2 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs2`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS2")]
pub type SfrBuregCrBuregs2 = crate::Reg<sfr_bureg_cr_buregs2::SfrBuregCrBuregs2Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs2;
#[doc = "SFR_BUREG_CR_BUREGS3 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs3`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS3")]
pub type SfrBuregCrBuregs3 = crate::Reg<sfr_bureg_cr_buregs3::SfrBuregCrBuregs3Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs3;
#[doc = "SFR_BUREG_CR_BUREGS4 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs4`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS4")]
pub type SfrBuregCrBuregs4 = crate::Reg<sfr_bureg_cr_buregs4::SfrBuregCrBuregs4Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs4;
#[doc = "SFR_BUREG_CR_BUREGS5 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs5`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS5")]
pub type SfrBuregCrBuregs5 = crate::Reg<sfr_bureg_cr_buregs5::SfrBuregCrBuregs5Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs5;
#[doc = "SFR_BUREG_CR_BUREGS6 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs6`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS6")]
pub type SfrBuregCrBuregs6 = crate::Reg<sfr_bureg_cr_buregs6::SfrBuregCrBuregs6Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs6;
#[doc = "SFR_BUREG_CR_BUREGS7 (rw) register accessor: See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_bureg_cr_buregs7`] module"]
#[doc(alias = "SFR_BUREG_CR_BUREGS7")]
pub type SfrBuregCrBuregs7 = crate::Reg<sfr_bureg_cr_buregs7::SfrBuregCrBuregs7Spec>;
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)"]
pub mod sfr_bureg_cr_buregs7;
