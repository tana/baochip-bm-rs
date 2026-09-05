#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_cm7evsel_cm7evsel0: SfrCm7evselCm7evsel0,
    sfr_cm7evsel_cm7evsel1: SfrCm7evselCm7evsel1,
    sfr_cm7evsel_cm7evsel2: SfrCm7evselCm7evsel2,
    sfr_cm7evsel_cm7evsel3: SfrCm7evselCm7evsel3,
    sfr_cm7evsel_cm7evsel4: SfrCm7evselCm7evsel4,
    sfr_cm7evsel_cm7evsel5: SfrCm7evselCm7evsel5,
    sfr_cm7evsel_cm7evsel6: SfrCm7evselCm7evsel6,
    sfr_cm7evsel_cm7evsel7: SfrCm7evselCm7evsel7,
    sfr_cm7even: SfrCm7even,
    sfr_cm7evfr: SfrCm7evfr,
    _reserved10: [u8; 0x08],
    sfr_tmrevsel: SfrTmrevsel,
    sfr_tmreven: SfrTmreven,
    _reserved12: [u8; 0x08],
    sfr_ifeven_ifeven0: SfrIfevenIfeven0,
    sfr_ifeven_ifeven1: SfrIfevenIfeven1,
    sfr_ifeven_ifeven2: SfrIfevenIfeven2,
    sfr_ifeven_ifeven3: SfrIfevenIfeven3,
    sfr_ifeven_ifeven4: SfrIfevenIfeven4,
    sfr_ifeven_ifeven5: SfrIfevenIfeven5,
    sfr_ifeven_ifeven6: SfrIfevenIfeven6,
    sfr_ifeven_ifeven7: SfrIfevenIfeven7,
    sfr_ifeverrfr: SfrIfeverrfr,
    _reserved21: [u8; 0x1c],
    sfr_cm7errfr: SfrCm7errfr,
    sfr_cm7errcr: SfrCm7errcr,
    _reserved23: [u8; 0x08],
    sfr_rrcevsel_rrc_evsel0: SfrRrcevselRrcEvsel0,
    sfr_rrcevsel_rrc_evsel1: SfrRrcevselRrcEvsel1,
    sfr_rrcevsel_rrc_evsel2: SfrRrcevselRrcEvsel2,
    sfr_rrcevsel_rrc_evsel3: SfrRrcevselRrcEvsel3,
    sfr_rrceven: SfrRrceven,
}
impl RegisterBlock {
    #[doc = "0x00 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel0(&self) -> &SfrCm7evselCm7evsel0 {
        &self.sfr_cm7evsel_cm7evsel0
    }
    #[doc = "0x04 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel1(&self) -> &SfrCm7evselCm7evsel1 {
        &self.sfr_cm7evsel_cm7evsel1
    }
    #[doc = "0x08 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel2(&self) -> &SfrCm7evselCm7evsel2 {
        &self.sfr_cm7evsel_cm7evsel2
    }
    #[doc = "0x0c - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel3(&self) -> &SfrCm7evselCm7evsel3 {
        &self.sfr_cm7evsel_cm7evsel3
    }
    #[doc = "0x10 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel4(&self) -> &SfrCm7evselCm7evsel4 {
        &self.sfr_cm7evsel_cm7evsel4
    }
    #[doc = "0x14 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel5(&self) -> &SfrCm7evselCm7evsel5 {
        &self.sfr_cm7evsel_cm7evsel5
    }
    #[doc = "0x18 - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel6(&self) -> &SfrCm7evselCm7evsel6 {
        &self.sfr_cm7evsel_cm7evsel6
    }
    #[doc = "0x1c - See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evsel_cm7evsel7(&self) -> &SfrCm7evselCm7evsel7 {
        &self.sfr_cm7evsel_cm7evsel7
    }
    #[doc = "0x20 - See `evc.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L141>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7even(&self) -> &SfrCm7even {
        &self.sfr_cm7even
    }
    #[doc = "0x24 - See `evc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L142>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7evfr(&self) -> &SfrCm7evfr {
        &self.sfr_cm7evfr
    }
    #[doc = "0x30 - See `evc.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L144>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tmrevsel(&self) -> &SfrTmrevsel {
        &self.sfr_tmrevsel
    }
    #[doc = "0x34 - See `evc.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L145>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_tmreven(&self) -> &SfrTmreven {
        &self.sfr_tmreven
    }
    #[doc = "0x40 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven0(&self) -> &SfrIfevenIfeven0 {
        &self.sfr_ifeven_ifeven0
    }
    #[doc = "0x44 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven1(&self) -> &SfrIfevenIfeven1 {
        &self.sfr_ifeven_ifeven1
    }
    #[doc = "0x48 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven2(&self) -> &SfrIfevenIfeven2 {
        &self.sfr_ifeven_ifeven2
    }
    #[doc = "0x4c - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven3(&self) -> &SfrIfevenIfeven3 {
        &self.sfr_ifeven_ifeven3
    }
    #[doc = "0x50 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven4(&self) -> &SfrIfevenIfeven4 {
        &self.sfr_ifeven_ifeven4
    }
    #[doc = "0x54 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven5(&self) -> &SfrIfevenIfeven5 {
        &self.sfr_ifeven_ifeven5
    }
    #[doc = "0x58 - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven6(&self) -> &SfrIfevenIfeven6 {
        &self.sfr_ifeven_ifeven6
    }
    #[doc = "0x5c - See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeven_ifeven7(&self) -> &SfrIfevenIfeven7 {
        &self.sfr_ifeven_ifeven7
    }
    #[doc = "0x60 - See `evc.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L148>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ifeverrfr(&self) -> &SfrIfeverrfr {
        &self.sfr_ifeverrfr
    }
    #[doc = "0x80 - See `evc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L150>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7errfr(&self) -> &SfrCm7errfr {
        &self.sfr_cm7errfr
    }
    #[doc = "0x84 - See `evc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L151>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cm7errcr(&self) -> &SfrCm7errcr {
        &self.sfr_cm7errcr
    }
    #[doc = "0x90 - See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcevsel_rrc_evsel0(&self) -> &SfrRrcevselRrcEvsel0 {
        &self.sfr_rrcevsel_rrc_evsel0
    }
    #[doc = "0x94 - See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcevsel_rrc_evsel1(&self) -> &SfrRrcevselRrcEvsel1 {
        &self.sfr_rrcevsel_rrc_evsel1
    }
    #[doc = "0x98 - See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcevsel_rrc_evsel2(&self) -> &SfrRrcevselRrcEvsel2 {
        &self.sfr_rrcevsel_rrc_evsel2
    }
    #[doc = "0x9c - See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcevsel_rrc_evsel3(&self) -> &SfrRrcevselRrcEvsel3 {
        &self.sfr_rrcevsel_rrc_evsel3
    }
    #[doc = "0xa0 - See `evc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L154>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrceven(&self) -> &SfrRrceven {
        &self.sfr_rrceven
    }
}
#[doc = "SFR_CM7EVSEL_CM7EVSEL0 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel0`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL0")]
pub type SfrCm7evselCm7evsel0 = crate::Reg<sfr_cm7evsel_cm7evsel0::SfrCm7evselCm7evsel0Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel0;
#[doc = "SFR_CM7EVSEL_CM7EVSEL1 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel1`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL1")]
pub type SfrCm7evselCm7evsel1 = crate::Reg<sfr_cm7evsel_cm7evsel1::SfrCm7evselCm7evsel1Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel1;
#[doc = "SFR_CM7EVSEL_CM7EVSEL2 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel2`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL2")]
pub type SfrCm7evselCm7evsel2 = crate::Reg<sfr_cm7evsel_cm7evsel2::SfrCm7evselCm7evsel2Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel2;
#[doc = "SFR_CM7EVSEL_CM7EVSEL3 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel3`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL3")]
pub type SfrCm7evselCm7evsel3 = crate::Reg<sfr_cm7evsel_cm7evsel3::SfrCm7evselCm7evsel3Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel3;
#[doc = "SFR_CM7EVSEL_CM7EVSEL4 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel4`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL4")]
pub type SfrCm7evselCm7evsel4 = crate::Reg<sfr_cm7evsel_cm7evsel4::SfrCm7evselCm7evsel4Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel4;
#[doc = "SFR_CM7EVSEL_CM7EVSEL5 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel5`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL5")]
pub type SfrCm7evselCm7evsel5 = crate::Reg<sfr_cm7evsel_cm7evsel5::SfrCm7evselCm7evsel5Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel5;
#[doc = "SFR_CM7EVSEL_CM7EVSEL6 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel6`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL6")]
pub type SfrCm7evselCm7evsel6 = crate::Reg<sfr_cm7evsel_cm7evsel6::SfrCm7evselCm7evsel6Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel6;
#[doc = "SFR_CM7EVSEL_CM7EVSEL7 (rw) register accessor: See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evsel_cm7evsel7`] module"]
#[doc(alias = "SFR_CM7EVSEL_CM7EVSEL7")]
pub type SfrCm7evselCm7evsel7 = crate::Reg<sfr_cm7evsel_cm7evsel7::SfrCm7evselCm7evsel7Spec>;
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)"]
pub mod sfr_cm7evsel_cm7evsel7;
#[doc = "SFR_CM7EVEN (rw) register accessor: See `evc.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L141>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7even::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7even::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7even`] module"]
#[doc(alias = "SFR_CM7EVEN")]
pub type SfrCm7even = crate::Reg<sfr_cm7even::SfrCm7evenSpec>;
#[doc = "See `evc.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L141>`__ (line numbers are approximate)"]
pub mod sfr_cm7even;
#[doc = "SFR_CM7EVFR (rw) register accessor: See `evc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7evfr`] module"]
#[doc(alias = "SFR_CM7EVFR")]
pub type SfrCm7evfr = crate::Reg<sfr_cm7evfr::SfrCm7evfrSpec>;
#[doc = "See `evc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L142>`__ (line numbers are approximate)"]
pub mod sfr_cm7evfr;
#[doc = "SFR_TMREVSEL (rw) register accessor: See `evc.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L144>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tmrevsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tmrevsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tmrevsel`] module"]
#[doc(alias = "SFR_TMREVSEL")]
pub type SfrTmrevsel = crate::Reg<sfr_tmrevsel::SfrTmrevselSpec>;
#[doc = "See `evc.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L144>`__ (line numbers are approximate)"]
pub mod sfr_tmrevsel;
#[doc = "SFR_TMREVEN (rw) register accessor: See `evc.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L145>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tmreven::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tmreven::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_tmreven`] module"]
#[doc(alias = "SFR_TMREVEN")]
pub type SfrTmreven = crate::Reg<sfr_tmreven::SfrTmrevenSpec>;
#[doc = "See `evc.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L145>`__ (line numbers are approximate)"]
pub mod sfr_tmreven;
#[doc = "SFR_IFEVEN_IFEVEN0 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven0`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN0")]
pub type SfrIfevenIfeven0 = crate::Reg<sfr_ifeven_ifeven0::SfrIfevenIfeven0Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven0;
#[doc = "SFR_IFEVEN_IFEVEN1 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven1`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN1")]
pub type SfrIfevenIfeven1 = crate::Reg<sfr_ifeven_ifeven1::SfrIfevenIfeven1Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven1;
#[doc = "SFR_IFEVEN_IFEVEN2 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven2`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN2")]
pub type SfrIfevenIfeven2 = crate::Reg<sfr_ifeven_ifeven2::SfrIfevenIfeven2Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven2;
#[doc = "SFR_IFEVEN_IFEVEN3 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven3`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN3")]
pub type SfrIfevenIfeven3 = crate::Reg<sfr_ifeven_ifeven3::SfrIfevenIfeven3Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven3;
#[doc = "SFR_IFEVEN_IFEVEN4 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven4`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN4")]
pub type SfrIfevenIfeven4 = crate::Reg<sfr_ifeven_ifeven4::SfrIfevenIfeven4Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven4;
#[doc = "SFR_IFEVEN_IFEVEN5 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven5`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN5")]
pub type SfrIfevenIfeven5 = crate::Reg<sfr_ifeven_ifeven5::SfrIfevenIfeven5Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven5;
#[doc = "SFR_IFEVEN_IFEVEN6 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven6`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN6")]
pub type SfrIfevenIfeven6 = crate::Reg<sfr_ifeven_ifeven6::SfrIfevenIfeven6Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven6;
#[doc = "SFR_IFEVEN_IFEVEN7 (rw) register accessor: See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeven_ifeven7`] module"]
#[doc(alias = "SFR_IFEVEN_IFEVEN7")]
pub type SfrIfevenIfeven7 = crate::Reg<sfr_ifeven_ifeven7::SfrIfevenIfeven7Spec>;
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)"]
pub mod sfr_ifeven_ifeven7;
#[doc = "SFR_IFEVERRFR (rw) register accessor: See `evc.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeverrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeverrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ifeverrfr`] module"]
#[doc(alias = "SFR_IFEVERRFR")]
pub type SfrIfeverrfr = crate::Reg<sfr_ifeverrfr::SfrIfeverrfrSpec>;
#[doc = "See `evc.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L148>`__ (line numbers are approximate)"]
pub mod sfr_ifeverrfr;
#[doc = "SFR_CM7ERRFR (rw) register accessor: See `evc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7errfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7errfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7errfr`] module"]
#[doc(alias = "SFR_CM7ERRFR")]
pub type SfrCm7errfr = crate::Reg<sfr_cm7errfr::SfrCm7errfrSpec>;
#[doc = "See `evc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L150>`__ (line numbers are approximate)"]
pub mod sfr_cm7errfr;
#[doc = "SFR_CM7ERRCR (rw) register accessor: See `evc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L151>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7errcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7errcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cm7errcr`] module"]
#[doc(alias = "SFR_CM7ERRCR")]
pub type SfrCm7errcr = crate::Reg<sfr_cm7errcr::SfrCm7errcrSpec>;
#[doc = "See `evc.sv#L151 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L151>`__ (line numbers are approximate)"]
pub mod sfr_cm7errcr;
#[doc = "SFR_RRCEVSEL_RRC_EVSEL0 (rw) register accessor: See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcevsel_rrc_evsel0`] module"]
#[doc(alias = "SFR_RRCEVSEL_RRC_EVSEL0")]
pub type SfrRrcevselRrcEvsel0 = crate::Reg<sfr_rrcevsel_rrc_evsel0::SfrRrcevselRrcEvsel0Spec>;
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_rrcevsel_rrc_evsel0;
#[doc = "SFR_RRCEVSEL_RRC_EVSEL1 (rw) register accessor: See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcevsel_rrc_evsel1`] module"]
#[doc(alias = "SFR_RRCEVSEL_RRC_EVSEL1")]
pub type SfrRrcevselRrcEvsel1 = crate::Reg<sfr_rrcevsel_rrc_evsel1::SfrRrcevselRrcEvsel1Spec>;
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_rrcevsel_rrc_evsel1;
#[doc = "SFR_RRCEVSEL_RRC_EVSEL2 (rw) register accessor: See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcevsel_rrc_evsel2`] module"]
#[doc(alias = "SFR_RRCEVSEL_RRC_EVSEL2")]
pub type SfrRrcevselRrcEvsel2 = crate::Reg<sfr_rrcevsel_rrc_evsel2::SfrRrcevselRrcEvsel2Spec>;
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_rrcevsel_rrc_evsel2;
#[doc = "SFR_RRCEVSEL_RRC_EVSEL3 (rw) register accessor: See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcevsel_rrc_evsel3`] module"]
#[doc(alias = "SFR_RRCEVSEL_RRC_EVSEL3")]
pub type SfrRrcevselRrcEvsel3 = crate::Reg<sfr_rrcevsel_rrc_evsel3::SfrRrcevselRrcEvsel3Spec>;
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)"]
pub mod sfr_rrcevsel_rrc_evsel3;
#[doc = "SFR_RRCEVEN (rw) register accessor: See `evc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrceven::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrceven::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrceven`] module"]
#[doc(alias = "SFR_RRCEVEN")]
pub type SfrRrceven = crate::Reg<sfr_rrceven::SfrRrcevenSpec>;
#[doc = "See `evc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L154>`__ (line numbers are approximate)"]
pub mod sfr_rrceven;
