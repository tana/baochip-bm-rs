#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_rrccr: SfrRrccr,
    sfr_rrcfd: SfrRrcfd,
    sfr_rrcsr: SfrRrcsr,
    sfr_rrcfr: SfrRrcfr,
    _reserved4: [u8; 0x04],
    sfr_rrcsr_set0: SfrRrcsrSet0,
    sfr_rrcsr_set1: SfrRrcsrSet1,
    sfr_rrcsr_rst0: SfrRrcsrRst0,
    sfr_rrcsr_rst1: SfrRrcsrRst1,
    sfr_rrcsr_rd0: SfrRrcsrRd0,
    sfr_rrcsr_rd1: SfrRrcsrRd1,
    _reserved10: [u8; 0xc4],
    sfr_rrcar: SfrRrcar,
}
impl RegisterBlock {
    #[doc = "0x00 - See `rrc.sv#L261 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L261>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrccr(&self) -> &SfrRrccr {
        &self.sfr_rrccr
    }
    #[doc = "0x04 - See `rrc.sv#L262 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L262>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcfd(&self) -> &SfrRrcfd {
        &self.sfr_rrcfd
    }
    #[doc = "0x08 - See `rrc.sv#L263 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L263>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr(&self) -> &SfrRrcsr {
        &self.sfr_rrcsr
    }
    #[doc = "0x0c - See `rrc.sv#L264 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L264>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcfr(&self) -> &SfrRrcfr {
        &self.sfr_rrcfr
    }
    #[doc = "0x14 - See `rrc.sv#L266 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L266>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_set0(&self) -> &SfrRrcsrSet0 {
        &self.sfr_rrcsr_set0
    }
    #[doc = "0x18 - See `rrc.sv#L267 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L267>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_set1(&self) -> &SfrRrcsrSet1 {
        &self.sfr_rrcsr_set1
    }
    #[doc = "0x1c - See `rrc.sv#L268 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L268>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_rst0(&self) -> &SfrRrcsrRst0 {
        &self.sfr_rrcsr_rst0
    }
    #[doc = "0x20 - See `rrc.sv#L269 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L269>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_rst1(&self) -> &SfrRrcsrRst1 {
        &self.sfr_rrcsr_rst1
    }
    #[doc = "0x24 - See `rrc.sv#L270 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L270>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_rd0(&self) -> &SfrRrcsrRd0 {
        &self.sfr_rrcsr_rd0
    }
    #[doc = "0x28 - See `rrc.sv#L271 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L271>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcsr_rd1(&self) -> &SfrRrcsrRd1 {
        &self.sfr_rrcsr_rd1
    }
    #[doc = "0xf0 - See `rrc.sv#L273 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L273>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rrcar(&self) -> &SfrRrcar {
        &self.sfr_rrcar
    }
}
#[doc = "SFR_RRCCR (rw) register accessor: See `rrc.sv#L261 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L261>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrccr`] module"]
#[doc(alias = "SFR_RRCCR")]
pub type SfrRrccr = crate::Reg<sfr_rrccr::SfrRrccrSpec>;
#[doc = "See `rrc.sv#L261 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L261>`__ (line numbers are approximate)"]
pub mod sfr_rrccr;
#[doc = "SFR_RRCFD (rw) register accessor: See `rrc.sv#L262 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L262>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcfd`] module"]
#[doc(alias = "SFR_RRCFD")]
pub type SfrRrcfd = crate::Reg<sfr_rrcfd::SfrRrcfdSpec>;
#[doc = "See `rrc.sv#L262 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L262>`__ (line numbers are approximate)"]
pub mod sfr_rrcfd;
#[doc = "SFR_RRCSR (rw) register accessor: See `rrc.sv#L263 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L263>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr`] module"]
#[doc(alias = "SFR_RRCSR")]
pub type SfrRrcsr = crate::Reg<sfr_rrcsr::SfrRrcsrSpec>;
#[doc = "See `rrc.sv#L263 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L263>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr;
#[doc = "SFR_RRCFR (rw) register accessor: See `rrc.sv#L264 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L264>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcfr`] module"]
#[doc(alias = "SFR_RRCFR")]
pub type SfrRrcfr = crate::Reg<sfr_rrcfr::SfrRrcfrSpec>;
#[doc = "See `rrc.sv#L264 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L264>`__ (line numbers are approximate)"]
pub mod sfr_rrcfr;
#[doc = "SFR_RRCSR_SET0 (rw) register accessor: See `rrc.sv#L266 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L266>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_set0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_set0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_set0`] module"]
#[doc(alias = "SFR_RRCSR_SET0")]
pub type SfrRrcsrSet0 = crate::Reg<sfr_rrcsr_set0::SfrRrcsrSet0Spec>;
#[doc = "See `rrc.sv#L266 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L266>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_set0;
#[doc = "SFR_RRCSR_SET1 (rw) register accessor: See `rrc.sv#L267 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L267>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_set1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_set1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_set1`] module"]
#[doc(alias = "SFR_RRCSR_SET1")]
pub type SfrRrcsrSet1 = crate::Reg<sfr_rrcsr_set1::SfrRrcsrSet1Spec>;
#[doc = "See `rrc.sv#L267 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L267>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_set1;
#[doc = "SFR_RRCSR_RST0 (rw) register accessor: See `rrc.sv#L268 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L268>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_rst0`] module"]
#[doc(alias = "SFR_RRCSR_RST0")]
pub type SfrRrcsrRst0 = crate::Reg<sfr_rrcsr_rst0::SfrRrcsrRst0Spec>;
#[doc = "See `rrc.sv#L268 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L268>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_rst0;
#[doc = "SFR_RRCSR_RST1 (rw) register accessor: See `rrc.sv#L269 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L269>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_rst1`] module"]
#[doc(alias = "SFR_RRCSR_RST1")]
pub type SfrRrcsrRst1 = crate::Reg<sfr_rrcsr_rst1::SfrRrcsrRst1Spec>;
#[doc = "See `rrc.sv#L269 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L269>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_rst1;
#[doc = "SFR_RRCSR_RD0 (rw) register accessor: See `rrc.sv#L270 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L270>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_rd0`] module"]
#[doc(alias = "SFR_RRCSR_RD0")]
pub type SfrRrcsrRd0 = crate::Reg<sfr_rrcsr_rd0::SfrRrcsrRd0Spec>;
#[doc = "See `rrc.sv#L270 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L270>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_rd0;
#[doc = "SFR_RRCSR_RD1 (rw) register accessor: See `rrc.sv#L271 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L271>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcsr_rd1`] module"]
#[doc(alias = "SFR_RRCSR_RD1")]
pub type SfrRrcsrRd1 = crate::Reg<sfr_rrcsr_rd1::SfrRrcsrRd1Spec>;
#[doc = "See `rrc.sv#L271 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L271>`__ (line numbers are approximate)"]
pub mod sfr_rrcsr_rd1;
#[doc = "SFR_RRCAR (rw) register accessor: See `rrc.sv#L273 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L273>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rrcar`] module"]
#[doc(alias = "SFR_RRCAR")]
pub type SfrRrcar = crate::Reg<sfr_rrcar::SfrRrcarSpec>;
#[doc = "See `rrc.sv#L273 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L273>`__ (line numbers are approximate)"]
pub mod sfr_rrcar;
