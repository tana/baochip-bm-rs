#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_mldrv_cr_mldrv0: SfrMldrvCrMldrv0,
    sfr_mldrv_cr_mldrv1: SfrMldrvCrMldrv1,
    _reserved2: [u8; 0x08],
    sfr_mlie_cr_mlie0: SfrMlieCrMlie0,
    sfr_mlie_cr_mlie1: SfrMlieCrMlie1,
    _reserved4: [u8; 0x08],
    sfr_mlsr_sr_mlsr0: SfrMlsrSrMlsr0,
    sfr_mlsr_sr_mlsr1: SfrMlsrSrMlsr1,
    sfr_mlsr_sr_mlsr2: SfrMlsrSrMlsr2,
    sfr_mlsr_sr_mlsr3: SfrMlsrSrMlsr3,
    sfr_mlsr_sr_mlsr4: SfrMlsrSrMlsr4,
    sfr_mlsr_sr_mlsr5: SfrMlsrSrMlsr5,
    sfr_mlsr_sr_mlsr6: SfrMlsrSrMlsr6,
    sfr_mlsr_sr_mlsr7: SfrMlsrSrMlsr7,
}
impl RegisterBlock {
    #[doc = "0x00 - See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mldrv_cr_mldrv0(&self) -> &SfrMldrvCrMldrv0 {
        &self.sfr_mldrv_cr_mldrv0
    }
    #[doc = "0x04 - See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mldrv_cr_mldrv1(&self) -> &SfrMldrvCrMldrv1 {
        &self.sfr_mldrv_cr_mldrv1
    }
    #[doc = "0x10 - See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlie_cr_mlie0(&self) -> &SfrMlieCrMlie0 {
        &self.sfr_mlie_cr_mlie0
    }
    #[doc = "0x14 - See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlie_cr_mlie1(&self) -> &SfrMlieCrMlie1 {
        &self.sfr_mlie_cr_mlie1
    }
    #[doc = "0x20 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr0(&self) -> &SfrMlsrSrMlsr0 {
        &self.sfr_mlsr_sr_mlsr0
    }
    #[doc = "0x24 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr1(&self) -> &SfrMlsrSrMlsr1 {
        &self.sfr_mlsr_sr_mlsr1
    }
    #[doc = "0x28 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr2(&self) -> &SfrMlsrSrMlsr2 {
        &self.sfr_mlsr_sr_mlsr2
    }
    #[doc = "0x2c - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr3(&self) -> &SfrMlsrSrMlsr3 {
        &self.sfr_mlsr_sr_mlsr3
    }
    #[doc = "0x30 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr4(&self) -> &SfrMlsrSrMlsr4 {
        &self.sfr_mlsr_sr_mlsr4
    }
    #[doc = "0x34 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr5(&self) -> &SfrMlsrSrMlsr5 {
        &self.sfr_mlsr_sr_mlsr5
    }
    #[doc = "0x38 - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr6(&self) -> &SfrMlsrSrMlsr6 {
        &self.sfr_mlsr_sr_mlsr6
    }
    #[doc = "0x3c - See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_mlsr_sr_mlsr7(&self) -> &SfrMlsrSrMlsr7 {
        &self.sfr_mlsr_sr_mlsr7
    }
}
#[doc = "SFR_MLDRV_CR_MLDRV0 (rw) register accessor: See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mldrv_cr_mldrv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mldrv_cr_mldrv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mldrv_cr_mldrv0`] module"]
#[doc(alias = "SFR_MLDRV_CR_MLDRV0")]
pub type SfrMldrvCrMldrv0 = crate::Reg<sfr_mldrv_cr_mldrv0::SfrMldrvCrMldrv0Spec>;
#[doc = "See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)"]
pub mod sfr_mldrv_cr_mldrv0;
#[doc = "SFR_MLDRV_CR_MLDRV1 (rw) register accessor: See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mldrv_cr_mldrv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mldrv_cr_mldrv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mldrv_cr_mldrv1`] module"]
#[doc(alias = "SFR_MLDRV_CR_MLDRV1")]
pub type SfrMldrvCrMldrv1 = crate::Reg<sfr_mldrv_cr_mldrv1::SfrMldrvCrMldrv1Spec>;
#[doc = "See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)"]
pub mod sfr_mldrv_cr_mldrv1;
#[doc = "SFR_MLIE_CR_MLIE0 (rw) register accessor: See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlie_cr_mlie0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlie_cr_mlie0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlie_cr_mlie0`] module"]
#[doc(alias = "SFR_MLIE_CR_MLIE0")]
pub type SfrMlieCrMlie0 = crate::Reg<sfr_mlie_cr_mlie0::SfrMlieCrMlie0Spec>;
#[doc = "See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)"]
pub mod sfr_mlie_cr_mlie0;
#[doc = "SFR_MLIE_CR_MLIE1 (rw) register accessor: See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlie_cr_mlie1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlie_cr_mlie1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlie_cr_mlie1`] module"]
#[doc(alias = "SFR_MLIE_CR_MLIE1")]
pub type SfrMlieCrMlie1 = crate::Reg<sfr_mlie_cr_mlie1::SfrMlieCrMlie1Spec>;
#[doc = "See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)"]
pub mod sfr_mlie_cr_mlie1;
#[doc = "SFR_MLSR_SR_MLSR0 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr0`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR0")]
pub type SfrMlsrSrMlsr0 = crate::Reg<sfr_mlsr_sr_mlsr0::SfrMlsrSrMlsr0Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr0;
#[doc = "SFR_MLSR_SR_MLSR1 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr1`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR1")]
pub type SfrMlsrSrMlsr1 = crate::Reg<sfr_mlsr_sr_mlsr1::SfrMlsrSrMlsr1Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr1;
#[doc = "SFR_MLSR_SR_MLSR2 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr2`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR2")]
pub type SfrMlsrSrMlsr2 = crate::Reg<sfr_mlsr_sr_mlsr2::SfrMlsrSrMlsr2Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr2;
#[doc = "SFR_MLSR_SR_MLSR3 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr3`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR3")]
pub type SfrMlsrSrMlsr3 = crate::Reg<sfr_mlsr_sr_mlsr3::SfrMlsrSrMlsr3Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr3;
#[doc = "SFR_MLSR_SR_MLSR4 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr4`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR4")]
pub type SfrMlsrSrMlsr4 = crate::Reg<sfr_mlsr_sr_mlsr4::SfrMlsrSrMlsr4Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr4;
#[doc = "SFR_MLSR_SR_MLSR5 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr5`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR5")]
pub type SfrMlsrSrMlsr5 = crate::Reg<sfr_mlsr_sr_mlsr5::SfrMlsrSrMlsr5Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr5;
#[doc = "SFR_MLSR_SR_MLSR6 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr6`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR6")]
pub type SfrMlsrSrMlsr6 = crate::Reg<sfr_mlsr_sr_mlsr6::SfrMlsrSrMlsr6Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr6;
#[doc = "SFR_MLSR_SR_MLSR7 (rw) register accessor: See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_mlsr_sr_mlsr7`] module"]
#[doc(alias = "SFR_MLSR_SR_MLSR7")]
pub type SfrMlsrSrMlsr7 = crate::Reg<sfr_mlsr_sr_mlsr7::SfrMlsrSrMlsr7Spec>;
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)"]
pub mod sfr_mlsr_sr_mlsr7;
