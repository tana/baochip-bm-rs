#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_cgusec: SfrCgusec,
    sfr_cgulp: SfrCgulp,
    sfr_seed: SfrSeed,
    sfr_seedar: SfrSeedar,
    sfr_cgusel0: SfrCgusel0,
    sfr_cgufd_cfgfdcr_0_4_0: SfrCgufdCfgfdcr0_4_0,
    sfr_cgufd_cfgfdcr_0_4_1: SfrCgufdCfgfdcr0_4_1,
    sfr_cgufd_cfgfdcr_0_4_2: SfrCgufdCfgfdcr0_4_2,
    sfr_cgufd_cfgfdcr_0_4_3: SfrCgufdCfgfdcr0_4_3,
    sfr_cgufd_cfgfdcr_0_4_4: SfrCgufdCfgfdcr0_4_4,
    sfr_cgufdao: SfrCgufdao,
    sfr_cguset: SfrCguset,
    sfr_cgusel1: SfrCgusel1,
    sfr_cgufdpke: SfrCgufdpke,
    sfr_cgufdaoram: SfrCgufdaoram,
    sfr_cgufdper: SfrCgufdper,
    sfr_cgufssr_fsfreq0: SfrCgufssrFsfreq0,
    sfr_cgufssr_fsfreq1: SfrCgufssrFsfreq1,
    sfr_cgufssr_fsfreq2: SfrCgufssrFsfreq2,
    sfr_cgufssr_fsfreq3: SfrCgufssrFsfreq3,
    sfr_cgufsvld: SfrCgufsvld,
    sfr_cgufscr: SfrCgufscr,
    _reserved22: [u8; 0x08],
    sfr_aclkgr: SfrAclkgr,
    sfr_hclkgr: SfrHclkgr,
    sfr_iclkgr: SfrIclkgr,
    sfr_pclkgr: SfrPclkgr,
    _reserved26: [u8; 0x10],
    sfr_rcurst0: SfrRcurst0,
    sfr_rcurst1: SfrRcurst1,
    sfr_rcusrcfr: SfrRcusrcfr,
    _reserved29: [u8; 0x04],
    sfr_ipcaripflow: SfrIpcaripflow,
    sfr_ipcen: SfrIpcen,
    sfr_ipclpen: SfrIpclpen,
    sfr_ipcosc: SfrIpcosc,
    sfr_ipcpllmn: SfrIpcpllmn,
    sfr_ipcpllf: SfrIpcpllf,
    sfr_ipcpllq: SfrIpcpllq,
    sfr_ipccr: SfrIpccr,
}
impl RegisterBlock {
    #[doc = "0x00 - See `sysctrl.sv#L768 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L768>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgusec(&self) -> &SfrCgusec {
        &self.sfr_cgusec
    }
    #[doc = "0x04 - See `sysctrl.sv#L769 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L769>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgulp(&self) -> &SfrCgulp {
        &self.sfr_cgulp
    }
    #[doc = "0x08 - See `sysctrl.sv#L771 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L771>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_seed(&self) -> &SfrSeed {
        &self.sfr_seed
    }
    #[doc = "0x0c - See `sysctrl.sv#L772 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L772>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_seedar(&self) -> &SfrSeedar {
        &self.sfr_seedar
    }
    #[doc = "0x10 - See `sysctrl.sv#L774 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L774>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgusel0(&self) -> &SfrCgusel0 {
        &self.sfr_cgusel0
    }
    #[doc = "0x14 - See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufd_cfgfdcr_0_4_0(&self) -> &SfrCgufdCfgfdcr0_4_0 {
        &self.sfr_cgufd_cfgfdcr_0_4_0
    }
    #[doc = "0x18 - See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufd_cfgfdcr_0_4_1(&self) -> &SfrCgufdCfgfdcr0_4_1 {
        &self.sfr_cgufd_cfgfdcr_0_4_1
    }
    #[doc = "0x1c - See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufd_cfgfdcr_0_4_2(&self) -> &SfrCgufdCfgfdcr0_4_2 {
        &self.sfr_cgufd_cfgfdcr_0_4_2
    }
    #[doc = "0x20 - See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufd_cfgfdcr_0_4_3(&self) -> &SfrCgufdCfgfdcr0_4_3 {
        &self.sfr_cgufd_cfgfdcr_0_4_3
    }
    #[doc = "0x24 - See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufd_cfgfdcr_0_4_4(&self) -> &SfrCgufdCfgfdcr0_4_4 {
        &self.sfr_cgufd_cfgfdcr_0_4_4
    }
    #[doc = "0x28 - See `sysctrl.sv#L778 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L778>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufdao(&self) -> &SfrCgufdao {
        &self.sfr_cgufdao
    }
    #[doc = "0x2c - See `sysctrl.sv#L781 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L781>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cguset(&self) -> &SfrCguset {
        &self.sfr_cguset
    }
    #[doc = "0x30 - See `sysctrl.sv#L782 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L782>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgusel1(&self) -> &SfrCgusel1 {
        &self.sfr_cgusel1
    }
    #[doc = "0x34 - See `sysctrl.sv#L777 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L777>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufdpke(&self) -> &SfrCgufdpke {
        &self.sfr_cgufdpke
    }
    #[doc = "0x38 - See `sysctrl.sv#L779 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L779>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufdaoram(&self) -> &SfrCgufdaoram {
        &self.sfr_cgufdaoram
    }
    #[doc = "0x3c - See `sysctrl.sv#L776 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L776>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufdper(&self) -> &SfrCgufdper {
        &self.sfr_cgufdper
    }
    #[doc = "0x40 - See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufssr_fsfreq0(&self) -> &SfrCgufssrFsfreq0 {
        &self.sfr_cgufssr_fsfreq0
    }
    #[doc = "0x44 - See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufssr_fsfreq1(&self) -> &SfrCgufssrFsfreq1 {
        &self.sfr_cgufssr_fsfreq1
    }
    #[doc = "0x48 - See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufssr_fsfreq2(&self) -> &SfrCgufssrFsfreq2 {
        &self.sfr_cgufssr_fsfreq2
    }
    #[doc = "0x4c - See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufssr_fsfreq3(&self) -> &SfrCgufssrFsfreq3 {
        &self.sfr_cgufssr_fsfreq3
    }
    #[doc = "0x50 - See `sysctrl.sv#L786 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L786>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufsvld(&self) -> &SfrCgufsvld {
        &self.sfr_cgufsvld
    }
    #[doc = "0x54 - See `sysctrl.sv#L787 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L787>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_cgufscr(&self) -> &SfrCgufscr {
        &self.sfr_cgufscr
    }
    #[doc = "0x60 - See `sysctrl.sv#L794 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L794>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_aclkgr(&self) -> &SfrAclkgr {
        &self.sfr_aclkgr
    }
    #[doc = "0x64 - See `sysctrl.sv#L795 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L795>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_hclkgr(&self) -> &SfrHclkgr {
        &self.sfr_hclkgr
    }
    #[doc = "0x68 - See `sysctrl.sv#L796 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L796>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_iclkgr(&self) -> &SfrIclkgr {
        &self.sfr_iclkgr
    }
    #[doc = "0x6c - See `sysctrl.sv#L797 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L797>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_pclkgr(&self) -> &SfrPclkgr {
        &self.sfr_pclkgr
    }
    #[doc = "0x80 - See `sysctrl.sv#L804 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L804>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rcurst0(&self) -> &SfrRcurst0 {
        &self.sfr_rcurst0
    }
    #[doc = "0x84 - See `sysctrl.sv#L805 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L805>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rcurst1(&self) -> &SfrRcurst1 {
        &self.sfr_rcurst1
    }
    #[doc = "0x88 - See `sysctrl.sv#L806 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L806>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rcusrcfr(&self) -> &SfrRcusrcfr {
        &self.sfr_rcusrcfr
    }
    #[doc = "0x90 - See `sysctrl.sv#L810 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L810>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcaripflow(&self) -> &SfrIpcaripflow {
        &self.sfr_ipcaripflow
    }
    #[doc = "0x94 - See `sysctrl.sv#L811 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L811>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcen(&self) -> &SfrIpcen {
        &self.sfr_ipcen
    }
    #[doc = "0x98 - See `sysctrl.sv#L812 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L812>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipclpen(&self) -> &SfrIpclpen {
        &self.sfr_ipclpen
    }
    #[doc = "0x9c - See `sysctrl.sv#L813 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L813>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcosc(&self) -> &SfrIpcosc {
        &self.sfr_ipcosc
    }
    #[doc = "0xa0 - See `sysctrl.sv#L814 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L814>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcpllmn(&self) -> &SfrIpcpllmn {
        &self.sfr_ipcpllmn
    }
    #[doc = "0xa4 - See `sysctrl.sv#L815 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L815>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcpllf(&self) -> &SfrIpcpllf {
        &self.sfr_ipcpllf
    }
    #[doc = "0xa8 - See `sysctrl.sv#L816 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L816>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipcpllq(&self) -> &SfrIpcpllq {
        &self.sfr_ipcpllq
    }
    #[doc = "0xac - See `sysctrl.sv#L817 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L817>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ipccr(&self) -> &SfrIpccr {
        &self.sfr_ipccr
    }
}
#[doc = "SFR_CGUSEC (rw) register accessor: See `sysctrl.sv#L768 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L768>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgusec`] module"]
#[doc(alias = "SFR_CGUSEC")]
pub type SfrCgusec = crate::Reg<sfr_cgusec::SfrCgusecSpec>;
#[doc = "See `sysctrl.sv#L768 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L768>`__ (line numbers are approximate)"]
pub mod sfr_cgusec;
#[doc = "SFR_CGULP (rw) register accessor: See `sysctrl.sv#L769 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L769>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgulp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgulp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgulp`] module"]
#[doc(alias = "SFR_CGULP")]
pub type SfrCgulp = crate::Reg<sfr_cgulp::SfrCgulpSpec>;
#[doc = "See `sysctrl.sv#L769 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L769>`__ (line numbers are approximate)"]
pub mod sfr_cgulp;
#[doc = "SFR_SEED (rw) register accessor: See `sysctrl.sv#L771 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L771>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_seed`] module"]
#[doc(alias = "SFR_SEED")]
pub type SfrSeed = crate::Reg<sfr_seed::SfrSeedSpec>;
#[doc = "See `sysctrl.sv#L771 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L771>`__ (line numbers are approximate)"]
pub mod sfr_seed;
#[doc = "SFR_SEEDAR (rw) register accessor: See `sysctrl.sv#L772 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L772>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_seedar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_seedar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_seedar`] module"]
#[doc(alias = "SFR_SEEDAR")]
pub type SfrSeedar = crate::Reg<sfr_seedar::SfrSeedarSpec>;
#[doc = "See `sysctrl.sv#L772 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L772>`__ (line numbers are approximate)"]
pub mod sfr_seedar;
#[doc = "SFR_CGUSEL0 (rw) register accessor: See `sysctrl.sv#L774 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L774>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgusel0`] module"]
#[doc(alias = "SFR_CGUSEL0")]
pub type SfrCgusel0 = crate::Reg<sfr_cgusel0::SfrCgusel0Spec>;
#[doc = "See `sysctrl.sv#L774 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L774>`__ (line numbers are approximate)"]
pub mod sfr_cgusel0;
#[doc = "SFR_CGUFD_CFGFDCR_0_4_0 (rw) register accessor: See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufd_cfgfdcr_0_4_0`] module"]
#[doc(alias = "SFR_CGUFD_CFGFDCR_0_4_0")]
pub type SfrCgufdCfgfdcr0_4_0 = crate::Reg<sfr_cgufd_cfgfdcr_0_4_0::SfrCgufdCfgfdcr0_4_0Spec>;
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
pub mod sfr_cgufd_cfgfdcr_0_4_0;
#[doc = "SFR_CGUFD_CFGFDCR_0_4_1 (rw) register accessor: See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufd_cfgfdcr_0_4_1`] module"]
#[doc(alias = "SFR_CGUFD_CFGFDCR_0_4_1")]
pub type SfrCgufdCfgfdcr0_4_1 = crate::Reg<sfr_cgufd_cfgfdcr_0_4_1::SfrCgufdCfgfdcr0_4_1Spec>;
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
pub mod sfr_cgufd_cfgfdcr_0_4_1;
#[doc = "SFR_CGUFD_CFGFDCR_0_4_2 (rw) register accessor: See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufd_cfgfdcr_0_4_2`] module"]
#[doc(alias = "SFR_CGUFD_CFGFDCR_0_4_2")]
pub type SfrCgufdCfgfdcr0_4_2 = crate::Reg<sfr_cgufd_cfgfdcr_0_4_2::SfrCgufdCfgfdcr0_4_2Spec>;
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
pub mod sfr_cgufd_cfgfdcr_0_4_2;
#[doc = "SFR_CGUFD_CFGFDCR_0_4_3 (rw) register accessor: See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufd_cfgfdcr_0_4_3`] module"]
#[doc(alias = "SFR_CGUFD_CFGFDCR_0_4_3")]
pub type SfrCgufdCfgfdcr0_4_3 = crate::Reg<sfr_cgufd_cfgfdcr_0_4_3::SfrCgufdCfgfdcr0_4_3Spec>;
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
pub mod sfr_cgufd_cfgfdcr_0_4_3;
#[doc = "SFR_CGUFD_CFGFDCR_0_4_4 (rw) register accessor: See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufd_cfgfdcr_0_4_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufd_cfgfdcr_0_4_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufd_cfgfdcr_0_4_4`] module"]
#[doc(alias = "SFR_CGUFD_CFGFDCR_0_4_4")]
pub type SfrCgufdCfgfdcr0_4_4 = crate::Reg<sfr_cgufd_cfgfdcr_0_4_4::SfrCgufdCfgfdcr0_4_4Spec>;
#[doc = "See `sysctrl.sv#L775 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L775>`__ (line numbers are approximate)"]
pub mod sfr_cgufd_cfgfdcr_0_4_4;
#[doc = "SFR_CGUFDAO (rw) register accessor: See `sysctrl.sv#L778 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L778>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdao::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdao::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufdao`] module"]
#[doc(alias = "SFR_CGUFDAO")]
pub type SfrCgufdao = crate::Reg<sfr_cgufdao::SfrCgufdaoSpec>;
#[doc = "See `sysctrl.sv#L778 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L778>`__ (line numbers are approximate)"]
pub mod sfr_cgufdao;
#[doc = "SFR_CGUSET (rw) register accessor: See `sysctrl.sv#L781 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L781>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cguset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cguset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cguset`] module"]
#[doc(alias = "SFR_CGUSET")]
pub type SfrCguset = crate::Reg<sfr_cguset::SfrCgusetSpec>;
#[doc = "See `sysctrl.sv#L781 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L781>`__ (line numbers are approximate)"]
pub mod sfr_cguset;
#[doc = "SFR_CGUSEL1 (rw) register accessor: See `sysctrl.sv#L782 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L782>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgusel1`] module"]
#[doc(alias = "SFR_CGUSEL1")]
pub type SfrCgusel1 = crate::Reg<sfr_cgusel1::SfrCgusel1Spec>;
#[doc = "See `sysctrl.sv#L782 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L782>`__ (line numbers are approximate)"]
pub mod sfr_cgusel1;
#[doc = "SFR_CGUFDPKE (rw) register accessor: See `sysctrl.sv#L777 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L777>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdpke::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdpke::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufdpke`] module"]
#[doc(alias = "SFR_CGUFDPKE")]
pub type SfrCgufdpke = crate::Reg<sfr_cgufdpke::SfrCgufdpkeSpec>;
#[doc = "See `sysctrl.sv#L777 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L777>`__ (line numbers are approximate)"]
pub mod sfr_cgufdpke;
#[doc = "SFR_CGUFDAORAM (rw) register accessor: See `sysctrl.sv#L779 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L779>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdaoram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdaoram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufdaoram`] module"]
#[doc(alias = "SFR_CGUFDAORAM")]
pub type SfrCgufdaoram = crate::Reg<sfr_cgufdaoram::SfrCgufdaoramSpec>;
#[doc = "See `sysctrl.sv#L779 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L779>`__ (line numbers are approximate)"]
pub mod sfr_cgufdaoram;
#[doc = "SFR_CGUFDPER (rw) register accessor: See `sysctrl.sv#L776 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L776>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufdper`] module"]
#[doc(alias = "SFR_CGUFDPER")]
pub type SfrCgufdper = crate::Reg<sfr_cgufdper::SfrCgufdperSpec>;
#[doc = "See `sysctrl.sv#L776 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L776>`__ (line numbers are approximate)"]
pub mod sfr_cgufdper;
#[doc = "SFR_CGUFSSR_FSFREQ0 (rw) register accessor: See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufssr_fsfreq0`] module"]
#[doc(alias = "SFR_CGUFSSR_FSFREQ0")]
pub type SfrCgufssrFsfreq0 = crate::Reg<sfr_cgufssr_fsfreq0::SfrCgufssrFsfreq0Spec>;
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
pub mod sfr_cgufssr_fsfreq0;
#[doc = "SFR_CGUFSSR_FSFREQ1 (rw) register accessor: See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufssr_fsfreq1`] module"]
#[doc(alias = "SFR_CGUFSSR_FSFREQ1")]
pub type SfrCgufssrFsfreq1 = crate::Reg<sfr_cgufssr_fsfreq1::SfrCgufssrFsfreq1Spec>;
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
pub mod sfr_cgufssr_fsfreq1;
#[doc = "SFR_CGUFSSR_FSFREQ2 (rw) register accessor: See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufssr_fsfreq2`] module"]
#[doc(alias = "SFR_CGUFSSR_FSFREQ2")]
pub type SfrCgufssrFsfreq2 = crate::Reg<sfr_cgufssr_fsfreq2::SfrCgufssrFsfreq2Spec>;
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
pub mod sfr_cgufssr_fsfreq2;
#[doc = "SFR_CGUFSSR_FSFREQ3 (rw) register accessor: See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufssr_fsfreq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufssr_fsfreq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufssr_fsfreq3`] module"]
#[doc(alias = "SFR_CGUFSSR_FSFREQ3")]
pub type SfrCgufssrFsfreq3 = crate::Reg<sfr_cgufssr_fsfreq3::SfrCgufssrFsfreq3Spec>;
#[doc = "See `sysctrl.sv#L785 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L785>`__ (line numbers are approximate)"]
pub mod sfr_cgufssr_fsfreq3;
#[doc = "SFR_CGUFSVLD (rw) register accessor: See `sysctrl.sv#L786 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L786>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufsvld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufsvld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufsvld`] module"]
#[doc(alias = "SFR_CGUFSVLD")]
pub type SfrCgufsvld = crate::Reg<sfr_cgufsvld::SfrCgufsvldSpec>;
#[doc = "See `sysctrl.sv#L786 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L786>`__ (line numbers are approximate)"]
pub mod sfr_cgufsvld;
#[doc = "SFR_CGUFSCR (rw) register accessor: See `sysctrl.sv#L787 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L787>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_cgufscr`] module"]
#[doc(alias = "SFR_CGUFSCR")]
pub type SfrCgufscr = crate::Reg<sfr_cgufscr::SfrCgufscrSpec>;
#[doc = "See `sysctrl.sv#L787 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L787>`__ (line numbers are approximate)"]
pub mod sfr_cgufscr;
#[doc = "SFR_ACLKGR (rw) register accessor: See `sysctrl.sv#L794 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L794>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aclkgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aclkgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_aclkgr`] module"]
#[doc(alias = "SFR_ACLKGR")]
pub type SfrAclkgr = crate::Reg<sfr_aclkgr::SfrAclkgrSpec>;
#[doc = "See `sysctrl.sv#L794 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L794>`__ (line numbers are approximate)"]
pub mod sfr_aclkgr;
#[doc = "SFR_HCLKGR (rw) register accessor: See `sysctrl.sv#L795 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L795>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_hclkgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_hclkgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_hclkgr`] module"]
#[doc(alias = "SFR_HCLKGR")]
pub type SfrHclkgr = crate::Reg<sfr_hclkgr::SfrHclkgrSpec>;
#[doc = "See `sysctrl.sv#L795 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L795>`__ (line numbers are approximate)"]
pub mod sfr_hclkgr;
#[doc = "SFR_ICLKGR (rw) register accessor: See `sysctrl.sv#L796 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L796>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_iclkgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_iclkgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_iclkgr`] module"]
#[doc(alias = "SFR_ICLKGR")]
pub type SfrIclkgr = crate::Reg<sfr_iclkgr::SfrIclkgrSpec>;
#[doc = "See `sysctrl.sv#L796 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L796>`__ (line numbers are approximate)"]
pub mod sfr_iclkgr;
#[doc = "SFR_PCLKGR (rw) register accessor: See `sysctrl.sv#L797 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L797>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pclkgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pclkgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_pclkgr`] module"]
#[doc(alias = "SFR_PCLKGR")]
pub type SfrPclkgr = crate::Reg<sfr_pclkgr::SfrPclkgrSpec>;
#[doc = "See `sysctrl.sv#L797 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L797>`__ (line numbers are approximate)"]
pub mod sfr_pclkgr;
#[doc = "SFR_RCURST0 (rw) register accessor: See `sysctrl.sv#L804 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L804>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcurst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcurst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rcurst0`] module"]
#[doc(alias = "SFR_RCURST0")]
pub type SfrRcurst0 = crate::Reg<sfr_rcurst0::SfrRcurst0Spec>;
#[doc = "See `sysctrl.sv#L804 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L804>`__ (line numbers are approximate)"]
pub mod sfr_rcurst0;
#[doc = "SFR_RCURST1 (rw) register accessor: See `sysctrl.sv#L805 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L805>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcurst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcurst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rcurst1`] module"]
#[doc(alias = "SFR_RCURST1")]
pub type SfrRcurst1 = crate::Reg<sfr_rcurst1::SfrRcurst1Spec>;
#[doc = "See `sysctrl.sv#L805 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L805>`__ (line numbers are approximate)"]
pub mod sfr_rcurst1;
#[doc = "SFR_RCUSRCFR (rw) register accessor: See `sysctrl.sv#L806 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L806>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcusrcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcusrcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rcusrcfr`] module"]
#[doc(alias = "SFR_RCUSRCFR")]
pub type SfrRcusrcfr = crate::Reg<sfr_rcusrcfr::SfrRcusrcfrSpec>;
#[doc = "See `sysctrl.sv#L806 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L806>`__ (line numbers are approximate)"]
pub mod sfr_rcusrcfr;
#[doc = "SFR_IPCARIPFLOW (rw) register accessor: See `sysctrl.sv#L810 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L810>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcaripflow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcaripflow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcaripflow`] module"]
#[doc(alias = "SFR_IPCARIPFLOW")]
pub type SfrIpcaripflow = crate::Reg<sfr_ipcaripflow::SfrIpcaripflowSpec>;
#[doc = "See `sysctrl.sv#L810 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L810>`__ (line numbers are approximate)"]
pub mod sfr_ipcaripflow;
#[doc = "SFR_IPCEN (rw) register accessor: See `sysctrl.sv#L811 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L811>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcen`] module"]
#[doc(alias = "SFR_IPCEN")]
pub type SfrIpcen = crate::Reg<sfr_ipcen::SfrIpcenSpec>;
#[doc = "See `sysctrl.sv#L811 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L811>`__ (line numbers are approximate)"]
pub mod sfr_ipcen;
#[doc = "SFR_IPCLPEN (rw) register accessor: See `sysctrl.sv#L812 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L812>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipclpen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipclpen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipclpen`] module"]
#[doc(alias = "SFR_IPCLPEN")]
pub type SfrIpclpen = crate::Reg<sfr_ipclpen::SfrIpclpenSpec>;
#[doc = "See `sysctrl.sv#L812 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L812>`__ (line numbers are approximate)"]
pub mod sfr_ipclpen;
#[doc = "SFR_IPCOSC (rw) register accessor: See `sysctrl.sv#L813 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L813>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcosc`] module"]
#[doc(alias = "SFR_IPCOSC")]
pub type SfrIpcosc = crate::Reg<sfr_ipcosc::SfrIpcoscSpec>;
#[doc = "See `sysctrl.sv#L813 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L813>`__ (line numbers are approximate)"]
pub mod sfr_ipcosc;
#[doc = "SFR_IPCPLLMN (rw) register accessor: See `sysctrl.sv#L814 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L814>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllmn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllmn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcpllmn`] module"]
#[doc(alias = "SFR_IPCPLLMN")]
pub type SfrIpcpllmn = crate::Reg<sfr_ipcpllmn::SfrIpcpllmnSpec>;
#[doc = "See `sysctrl.sv#L814 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L814>`__ (line numbers are approximate)"]
pub mod sfr_ipcpllmn;
#[doc = "SFR_IPCPLLF (rw) register accessor: See `sysctrl.sv#L815 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L815>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcpllf`] module"]
#[doc(alias = "SFR_IPCPLLF")]
pub type SfrIpcpllf = crate::Reg<sfr_ipcpllf::SfrIpcpllfSpec>;
#[doc = "See `sysctrl.sv#L815 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L815>`__ (line numbers are approximate)"]
pub mod sfr_ipcpllf;
#[doc = "SFR_IPCPLLQ (rw) register accessor: See `sysctrl.sv#L816 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L816>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipcpllq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipcpllq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipcpllq`] module"]
#[doc(alias = "SFR_IPCPLLQ")]
pub type SfrIpcpllq = crate::Reg<sfr_ipcpllq::SfrIpcpllqSpec>;
#[doc = "See `sysctrl.sv#L816 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L816>`__ (line numbers are approximate)"]
pub mod sfr_ipcpllq;
#[doc = "SFR_IPCCR (rw) register accessor: See `sysctrl.sv#L817 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L817>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ipccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ipccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ipccr`] module"]
#[doc(alias = "SFR_IPCCR")]
pub type SfrIpccr = crate::Reg<sfr_ipccr::SfrIpccrSpec>;
#[doc = "See `sysctrl.sv#L817 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L817>`__ (line numbers are approximate)"]
pub mod sfr_ipccr;
