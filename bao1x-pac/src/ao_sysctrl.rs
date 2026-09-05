#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr_cr: CrCr,
    cr_clk1hzfd: CrClk1hzfd,
    cr_wkupmask: CrWkupmask,
    cr_rstcrmask: CrRstcrmask,
    sfr_pmucsr: SfrPmucsr,
    sfr_pmucrlp: SfrPmucrlp,
    sfr_pmucrpd: SfrPmucrpd,
    sfr_pmudftsr: SfrPmudftsr,
    sfr_pmutrm0csr: SfrPmutrm0csr,
    sfr_pmutrm1csr: SfrPmutrm1csr,
    sfr_pmutrmlp0: SfrPmutrmlp0,
    sfr_pmutrmlp1: SfrPmutrmlp1,
    _reserved12: [u8; 0x04],
    sfr_osccr: SfrOsccr,
    sfr_pmusr: SfrPmusr,
    sfr_pmufr: SfrPmufr,
    sfr_aofr: SfrAofr,
    sfr_pmupdar: SfrPmupdar,
    _reserved17: [u8; 0x08],
    ar_aoperi_clrint: ArAoperiClrint,
    _reserved18: [u8; 0x0c],
    sfr_iox: SfrIox,
    sfr_aopadpu: SfrAopadpu,
}
impl RegisterBlock {
    #[doc = "0x00 - See `ao_sysctrl.sv#L367 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L367>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_cr(&self) -> &CrCr {
        &self.cr_cr
    }
    #[doc = "0x04 - See `ao_sysctrl.sv#L368 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L368>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_clk1hzfd(&self) -> &CrClk1hzfd {
        &self.cr_clk1hzfd
    }
    #[doc = "0x08 - See `ao_sysctrl.sv#L369 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L369>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_wkupmask(&self) -> &CrWkupmask {
        &self.cr_wkupmask
    }
    #[doc = "0x0c - See `ao_sysctrl.sv#L370 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L370>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn cr_rstcrmask(&self) -> &CrRstcrmask {
        &self.cr_rstcrmask
    }
    #[doc = "0x10 - See `ao_sysctrl.sv#L376 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L376>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmucsr(&self) -> &SfrPmucsr {
        &self.sfr_pmucsr
    }
    #[doc = "0x14 - See `ao_sysctrl.sv#L373 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L373>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmucrlp(&self) -> &SfrPmucrlp {
        &self.sfr_pmucrlp
    }
    #[doc = "0x18 - See `ao_sysctrl.sv#L374 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L374>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmucrpd(&self) -> &SfrPmucrpd {
        &self.sfr_pmucrpd
    }
    #[doc = "0x1c - See `ao_sysctrl.sv#L377 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L377>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmudftsr(&self) -> &SfrPmudftsr {
        &self.sfr_pmudftsr
    }
    #[doc = "0x20 - See `ao_sysctrl.sv#L383 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L383>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmutrm0csr(&self) -> &SfrPmutrm0csr {
        &self.sfr_pmutrm0csr
    }
    #[doc = "0x24 - See `ao_sysctrl.sv#L384 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L384>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmutrm1csr(&self) -> &SfrPmutrm1csr {
        &self.sfr_pmutrm1csr
    }
    #[doc = "0x28 - See `ao_sysctrl.sv#L381 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L381>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmutrmlp0(&self) -> &SfrPmutrmlp0 {
        &self.sfr_pmutrmlp0
    }
    #[doc = "0x2c - See `ao_sysctrl.sv#L382 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L382>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmutrmlp1(&self) -> &SfrPmutrmlp1 {
        &self.sfr_pmutrmlp1
    }
    #[doc = "0x34 - See `ao_sysctrl.sv#L386 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L386>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_osccr(&self) -> &SfrOsccr {
        &self.sfr_osccr
    }
    #[doc = "0x38 - See `ao_sysctrl.sv#L387 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L387>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmusr(&self) -> &SfrPmusr {
        &self.sfr_pmusr
    }
    #[doc = "0x3c - See `ao_sysctrl.sv#L388 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L388>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmufr(&self) -> &SfrPmufr {
        &self.sfr_pmufr
    }
    #[doc = "0x40 - See `ao_sysctrl.sv#L390 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L390>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_aofr(&self) -> &SfrAofr {
        &self.sfr_aofr
    }
    #[doc = "0x44 - See `ao_sysctrl.sv#L391 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L391>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pmupdar(&self) -> &SfrPmupdar {
        &self.sfr_pmupdar
    }
    #[doc = "0x50 - See `ao_sysctrl.sv#L393 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L393>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn ar_aoperi_clrint(&self) -> &ArAoperiClrint {
        &self.ar_aoperi_clrint
    }
    #[doc = "0x60 - See `ao_sysctrl.sv#L400 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L400>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_iox(&self) -> &SfrIox {
        &self.sfr_iox
    }
    #[doc = "0x64 - See `ao_sysctrl.sv#L401 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L401>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_aopadpu(&self) -> &SfrAopadpu {
        &self.sfr_aopadpu
    }
}
#[doc = "CR_CR (rw) register accessor: See `ao_sysctrl.sv#L367 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L367>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_cr`] module"]
#[doc(alias = "CR_CR")]
pub type CrCr = crate::Reg<cr_cr::CrCrSpec>;
#[doc = "See `ao_sysctrl.sv#L367 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L367>`__ (line numbers are approximate)"]
pub mod cr_cr;
#[doc = "CR_CLK1HZFD (rw) register accessor: See `ao_sysctrl.sv#L368 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L368>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_clk1hzfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_clk1hzfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_clk1hzfd`] module"]
#[doc(alias = "CR_CLK1HZFD")]
pub type CrClk1hzfd = crate::Reg<cr_clk1hzfd::CrClk1hzfdSpec>;
#[doc = "See `ao_sysctrl.sv#L368 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L368>`__ (line numbers are approximate)"]
pub mod cr_clk1hzfd;
#[doc = "CR_WKUPMASK (rw) register accessor: See `ao_sysctrl.sv#L369 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L369>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_wkupmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_wkupmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_wkupmask`] module"]
#[doc(alias = "CR_WKUPMASK")]
pub type CrWkupmask = crate::Reg<cr_wkupmask::CrWkupmaskSpec>;
#[doc = "See `ao_sysctrl.sv#L369 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L369>`__ (line numbers are approximate)"]
pub mod cr_wkupmask;
#[doc = "CR_RSTCRMASK (rw) register accessor: See `ao_sysctrl.sv#L370 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L370>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rstcrmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rstcrmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_rstcrmask`] module"]
#[doc(alias = "CR_RSTCRMASK")]
pub type CrRstcrmask = crate::Reg<cr_rstcrmask::CrRstcrmaskSpec>;
#[doc = "See `ao_sysctrl.sv#L370 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L370>`__ (line numbers are approximate)"]
pub mod cr_rstcrmask;
#[doc = "SFR_PMUCSR (rw) register accessor: See `ao_sysctrl.sv#L376 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L376>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmucsr`] module"]
#[doc(alias = "SFR_PMUCSR")]
pub type SfrPmucsr = crate::Reg<sfr_pmucsr::SfrPmucsrSpec>;
#[doc = "See `ao_sysctrl.sv#L376 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L376>`__ (line numbers are approximate)"]
pub mod sfr_pmucsr;
#[doc = "SFR_PMUCRLP (rw) register accessor: See `ao_sysctrl.sv#L373 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L373>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucrlp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucrlp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmucrlp`] module"]
#[doc(alias = "SFR_PMUCRLP")]
pub type SfrPmucrlp = crate::Reg<sfr_pmucrlp::SfrPmucrlpSpec>;
#[doc = "See `ao_sysctrl.sv#L373 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L373>`__ (line numbers are approximate)"]
pub mod sfr_pmucrlp;
#[doc = "SFR_PMUCRPD (rw) register accessor: See `ao_sysctrl.sv#L374 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L374>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucrpd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucrpd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmucrpd`] module"]
#[doc(alias = "SFR_PMUCRPD")]
pub type SfrPmucrpd = crate::Reg<sfr_pmucrpd::SfrPmucrpdSpec>;
#[doc = "See `ao_sysctrl.sv#L374 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L374>`__ (line numbers are approximate)"]
pub mod sfr_pmucrpd;
#[doc = "SFR_PMUDFTSR (rw) register accessor: See `ao_sysctrl.sv#L377 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L377>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmudftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmudftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmudftsr`] module"]
#[doc(alias = "SFR_PMUDFTSR")]
pub type SfrPmudftsr = crate::Reg<sfr_pmudftsr::SfrPmudftsrSpec>;
#[doc = "See `ao_sysctrl.sv#L377 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L377>`__ (line numbers are approximate)"]
pub mod sfr_pmudftsr;
#[doc = "SFR_PMUTRM0CSR (rw) register accessor: See `ao_sysctrl.sv#L383 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L383>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrm0csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrm0csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmutrm0csr`] module"]
#[doc(alias = "SFR_PMUTRM0CSR")]
pub type SfrPmutrm0csr = crate::Reg<sfr_pmutrm0csr::SfrPmutrm0csrSpec>;
#[doc = "See `ao_sysctrl.sv#L383 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L383>`__ (line numbers are approximate)"]
pub mod sfr_pmutrm0csr;
#[doc = "SFR_PMUTRM1CSR (rw) register accessor: See `ao_sysctrl.sv#L384 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L384>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrm1csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrm1csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmutrm1csr`] module"]
#[doc(alias = "SFR_PMUTRM1CSR")]
pub type SfrPmutrm1csr = crate::Reg<sfr_pmutrm1csr::SfrPmutrm1csrSpec>;
#[doc = "See `ao_sysctrl.sv#L384 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L384>`__ (line numbers are approximate)"]
pub mod sfr_pmutrm1csr;
#[doc = "SFR_PMUTRMLP0 (rw) register accessor: See `ao_sysctrl.sv#L381 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L381>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrmlp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrmlp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmutrmlp0`] module"]
#[doc(alias = "SFR_PMUTRMLP0")]
pub type SfrPmutrmlp0 = crate::Reg<sfr_pmutrmlp0::SfrPmutrmlp0Spec>;
#[doc = "See `ao_sysctrl.sv#L381 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L381>`__ (line numbers are approximate)"]
pub mod sfr_pmutrmlp0;
#[doc = "SFR_PMUTRMLP1 (rw) register accessor: See `ao_sysctrl.sv#L382 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L382>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrmlp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrmlp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmutrmlp1`] module"]
#[doc(alias = "SFR_PMUTRMLP1")]
pub type SfrPmutrmlp1 = crate::Reg<sfr_pmutrmlp1::SfrPmutrmlp1Spec>;
#[doc = "See `ao_sysctrl.sv#L382 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L382>`__ (line numbers are approximate)"]
pub mod sfr_pmutrmlp1;
#[doc = "SFR_OSCCR (rw) register accessor: See `ao_sysctrl.sv#L386 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L386>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_osccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_osccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_osccr`] module"]
#[doc(alias = "SFR_OSCCR")]
pub type SfrOsccr = crate::Reg<sfr_osccr::SfrOsccrSpec>;
#[doc = "See `ao_sysctrl.sv#L386 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L386>`__ (line numbers are approximate)"]
pub mod sfr_osccr;
#[doc = "SFR_PMUSR (rw) register accessor: See `ao_sysctrl.sv#L387 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L387>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmusr`] module"]
#[doc(alias = "SFR_PMUSR")]
pub type SfrPmusr = crate::Reg<sfr_pmusr::SfrPmusrSpec>;
#[doc = "See `ao_sysctrl.sv#L387 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L387>`__ (line numbers are approximate)"]
pub mod sfr_pmusr;
#[doc = "SFR_PMUFR (rw) register accessor: See `ao_sysctrl.sv#L388 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L388>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmufr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmufr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmufr`] module"]
#[doc(alias = "SFR_PMUFR")]
pub type SfrPmufr = crate::Reg<sfr_pmufr::SfrPmufrSpec>;
#[doc = "See `ao_sysctrl.sv#L388 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L388>`__ (line numbers are approximate)"]
pub mod sfr_pmufr;
#[doc = "SFR_AOFR (rw) register accessor: See `ao_sysctrl.sv#L390 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L390>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_aofr`] module"]
#[doc(alias = "SFR_AOFR")]
pub type SfrAofr = crate::Reg<sfr_aofr::SfrAofrSpec>;
#[doc = "See `ao_sysctrl.sv#L390 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L390>`__ (line numbers are approximate)"]
pub mod sfr_aofr;
#[doc = "SFR_PMUPDAR (rw) register accessor: See `ao_sysctrl.sv#L391 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L391>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmupdar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmupdar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pmupdar`] module"]
#[doc(alias = "SFR_PMUPDAR")]
pub type SfrPmupdar = crate::Reg<sfr_pmupdar::SfrPmupdarSpec>;
#[doc = "See `ao_sysctrl.sv#L391 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L391>`__ (line numbers are approximate)"]
pub mod sfr_pmupdar;
#[doc = "AR_AOPERI_CLRINT (rw) register accessor: See `ao_sysctrl.sv#L393 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L393>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_aoperi_clrint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_aoperi_clrint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_aoperi_clrint`] module"]
#[doc(alias = "AR_AOPERI_CLRINT")]
pub type ArAoperiClrint = crate::Reg<ar_aoperi_clrint::ArAoperiClrintSpec>;
#[doc = "See `ao_sysctrl.sv#L393 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L393>`__ (line numbers are approximate)"]
pub mod ar_aoperi_clrint;
#[doc = "SFR_IOX (rw) register accessor: See `ao_sysctrl.sv#L400 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L400>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iox::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iox::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_iox`] module"]
#[doc(alias = "SFR_IOX")]
pub type SfrIox = crate::Reg<sfr_iox::SfrIoxSpec>;
#[doc = "See `ao_sysctrl.sv#L400 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L400>`__ (line numbers are approximate)"]
pub mod sfr_iox;
#[doc = "SFR_AOPADPU (rw) register accessor: See `ao_sysctrl.sv#L401 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L401>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aopadpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aopadpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_aopadpu`] module"]
#[doc(alias = "SFR_AOPADPU")]
pub type SfrAopadpu = crate::Reg<sfr_aopadpu::SfrAopadpuSpec>;
#[doc = "See `ao_sysctrl.sv#L401 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L401>`__ (line numbers are approximate)"]
pub mod sfr_aopadpu;
