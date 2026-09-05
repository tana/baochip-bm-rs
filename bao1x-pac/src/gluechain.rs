#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_gcmask_cr_gcmask0: SfrGcmaskCrGcmask0,
    sfr_gcsr_gluereg0: SfrGcsrGluereg0,
    sfr_gcrst_gluerst0: SfrGcrstGluerst0,
    sfr_gctest_gluetest0: SfrGctestGluetest0,
}
impl RegisterBlock {
    #[doc = "0x00 - See `gluechain.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L44>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gcmask_cr_gcmask0(&self) -> &SfrGcmaskCrGcmask0 {
        &self.sfr_gcmask_cr_gcmask0
    }
    #[doc = "0x04 - See `gluechain.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L45>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gcsr_gluereg0(&self) -> &SfrGcsrGluereg0 {
        &self.sfr_gcsr_gluereg0
    }
    #[doc = "0x08 - See `gluechain.sv#L46 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L46>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gcrst_gluerst0(&self) -> &SfrGcrstGluerst0 {
        &self.sfr_gcrst_gluerst0
    }
    #[doc = "0x0c - See `gluechain.sv#L47 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L47>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_gctest_gluetest0(&self) -> &SfrGctestGluetest0 {
        &self.sfr_gctest_gluetest0
    }
}
#[doc = "SFR_GCMASK_CR_GCMASK0 (rw) register accessor: See `gluechain.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L44>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcmask_cr_gcmask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcmask_cr_gcmask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gcmask_cr_gcmask0`] module"]
#[doc(alias = "SFR_GCMASK_CR_GCMASK0")]
pub type SfrGcmaskCrGcmask0 = crate::Reg<sfr_gcmask_cr_gcmask0::SfrGcmaskCrGcmask0Spec>;
#[doc = "See `gluechain.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L44>`__ (line numbers are approximate)"]
pub mod sfr_gcmask_cr_gcmask0;
#[doc = "SFR_GCSR_GLUEREG0 (rw) register accessor: See `gluechain.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L45>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcsr_gluereg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcsr_gluereg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gcsr_gluereg0`] module"]
#[doc(alias = "SFR_GCSR_GLUEREG0")]
pub type SfrGcsrGluereg0 = crate::Reg<sfr_gcsr_gluereg0::SfrGcsrGluereg0Spec>;
#[doc = "See `gluechain.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L45>`__ (line numbers are approximate)"]
pub mod sfr_gcsr_gluereg0;
#[doc = "SFR_GCRST_GLUERST0 (rw) register accessor: See `gluechain.sv#L46 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L46>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcrst_gluerst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcrst_gluerst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gcrst_gluerst0`] module"]
#[doc(alias = "SFR_GCRST_GLUERST0")]
pub type SfrGcrstGluerst0 = crate::Reg<sfr_gcrst_gluerst0::SfrGcrstGluerst0Spec>;
#[doc = "See `gluechain.sv#L46 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L46>`__ (line numbers are approximate)"]
pub mod sfr_gcrst_gluerst0;
#[doc = "SFR_GCTEST_GLUETEST0 (rw) register accessor: See `gluechain.sv#L47 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L47>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gctest_gluetest0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gctest_gluetest0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_gctest_gluetest0`] module"]
#[doc(alias = "SFR_GCTEST_GLUETEST0")]
pub type SfrGctestGluetest0 = crate::Reg<sfr_gctest_gluetest0::SfrGctestGluetest0Spec>;
#[doc = "See `gluechain.sv#L47 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L47>`__ (line numbers are approximate)"]
pub mod sfr_gctest_gluetest0;
