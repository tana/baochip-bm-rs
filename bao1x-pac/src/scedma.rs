#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_schstart_ar: SfrSchstartAr,
    _reserved1: [u8; 0x0c],
    sfr_xch_func: SfrXchFunc,
    sfr_xch_opt: SfrXchOpt,
    sfr_xch_axstart: SfrXchAxstart,
    sfr_xch_segid: SfrXchSegid,
    sfr_xch_segstart: SfrXchSegstart,
    sfr_xch_transize: SfrXchTransize,
    _reserved7: [u8; 0x08],
    sfr_sch_func: SfrSchFunc,
    sfr_sch_opt: SfrSchOpt,
    sfr_sch_axstart: SfrSchAxstart,
    sfr_sch_segid: SfrSchSegid,
    sfr_sch_segstart: SfrSchSegstart,
    sfr_sch_transize: SfrSchTransize,
    _reserved13: [u8; 0x08],
    sfr_ich_opt: SfrIchOpt,
    sfr_ich_segid: SfrIchSegid,
    sfr_ich_rpstart: SfrIchRpstart,
    sfr_ich_wpstart: SfrIchWpstart,
    sfr_ich_transize: SfrIchTransize,
    _reserved18: [u8; 0x0c],
    sfr_wdatabypass_mode: SfrWdatabypassMode,
    sfr_wdatabypass_data: SfrWdatabypassData,
}
impl RegisterBlock {
    #[doc = "0x00 - See `scedma.sv#L95 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L95>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_schstart_ar(&self) -> &SfrSchstartAr {
        &self.sfr_schstart_ar
    }
    #[doc = "0x10 - See `scedma.sv#L97 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L97>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_func(&self) -> &SfrXchFunc {
        &self.sfr_xch_func
    }
    #[doc = "0x14 - See `scedma.sv#L98 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L98>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_opt(&self) -> &SfrXchOpt {
        &self.sfr_xch_opt
    }
    #[doc = "0x18 - See `scedma.sv#L99 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L99>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_axstart(&self) -> &SfrXchAxstart {
        &self.sfr_xch_axstart
    }
    #[doc = "0x1c - See `scedma.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L100>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_segid(&self) -> &SfrXchSegid {
        &self.sfr_xch_segid
    }
    #[doc = "0x20 - See `scedma.sv#L101 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L101>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_segstart(&self) -> &SfrXchSegstart {
        &self.sfr_xch_segstart
    }
    #[doc = "0x24 - See `scedma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L102>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_xch_transize(&self) -> &SfrXchTransize {
        &self.sfr_xch_transize
    }
    #[doc = "0x30 - See `scedma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L104>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_func(&self) -> &SfrSchFunc {
        &self.sfr_sch_func
    }
    #[doc = "0x34 - See `scedma.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L105>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_opt(&self) -> &SfrSchOpt {
        &self.sfr_sch_opt
    }
    #[doc = "0x38 - See `scedma.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L106>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_axstart(&self) -> &SfrSchAxstart {
        &self.sfr_sch_axstart
    }
    #[doc = "0x3c - See `scedma.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L107>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_segid(&self) -> &SfrSchSegid {
        &self.sfr_sch_segid
    }
    #[doc = "0x40 - See `scedma.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L108>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_segstart(&self) -> &SfrSchSegstart {
        &self.sfr_sch_segstart
    }
    #[doc = "0x44 - See `scedma.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L109>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_sch_transize(&self) -> &SfrSchTransize {
        &self.sfr_sch_transize
    }
    #[doc = "0x50 - See `scedma.sv#L111 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L111>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ich_opt(&self) -> &SfrIchOpt {
        &self.sfr_ich_opt
    }
    #[doc = "0x54 - See `scedma.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L112>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ich_segid(&self) -> &SfrIchSegid {
        &self.sfr_ich_segid
    }
    #[doc = "0x58 - See `scedma.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L113>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ich_rpstart(&self) -> &SfrIchRpstart {
        &self.sfr_ich_rpstart
    }
    #[doc = "0x5c - See `scedma.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L114>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ich_wpstart(&self) -> &SfrIchWpstart {
        &self.sfr_ich_wpstart
    }
    #[doc = "0x60 - See `scedma.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L115>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_ich_transize(&self) -> &SfrIchTransize {
        &self.sfr_ich_transize
    }
    #[doc = "0x70 - See `scedma.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L117>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_wdatabypass_mode(&self) -> &SfrWdatabypassMode {
        &self.sfr_wdatabypass_mode
    }
    #[doc = "0x74 - See `scedma.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L118>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_wdatabypass_data(&self) -> &SfrWdatabypassData {
        &self.sfr_wdatabypass_data
    }
}
#[doc = "SFR_SCHSTART_AR (rw) register accessor: See `scedma.sv#L95 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L95>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_schstart_ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_schstart_ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_schstart_ar`] module"]
#[doc(alias = "SFR_SCHSTART_AR")]
pub type SfrSchstartAr = crate::Reg<sfr_schstart_ar::SfrSchstartArSpec>;
#[doc = "See `scedma.sv#L95 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L95>`__ (line numbers are approximate)"]
pub mod sfr_schstart_ar;
#[doc = "SFR_XCH_FUNC (rw) register accessor: See `scedma.sv#L97 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L97>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_func`] module"]
#[doc(alias = "SFR_XCH_FUNC")]
pub type SfrXchFunc = crate::Reg<sfr_xch_func::SfrXchFuncSpec>;
#[doc = "See `scedma.sv#L97 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L97>`__ (line numbers are approximate)"]
pub mod sfr_xch_func;
#[doc = "SFR_XCH_OPT (rw) register accessor: See `scedma.sv#L98 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L98>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_opt`] module"]
#[doc(alias = "SFR_XCH_OPT")]
pub type SfrXchOpt = crate::Reg<sfr_xch_opt::SfrXchOptSpec>;
#[doc = "See `scedma.sv#L98 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L98>`__ (line numbers are approximate)"]
pub mod sfr_xch_opt;
#[doc = "SFR_XCH_AXSTART (rw) register accessor: See `scedma.sv#L99 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L99>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_axstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_axstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_axstart`] module"]
#[doc(alias = "SFR_XCH_AXSTART")]
pub type SfrXchAxstart = crate::Reg<sfr_xch_axstart::SfrXchAxstartSpec>;
#[doc = "See `scedma.sv#L99 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L99>`__ (line numbers are approximate)"]
pub mod sfr_xch_axstart;
#[doc = "SFR_XCH_SEGID (rw) register accessor: See `scedma.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_segid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_segid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_segid`] module"]
#[doc(alias = "SFR_XCH_SEGID")]
pub type SfrXchSegid = crate::Reg<sfr_xch_segid::SfrXchSegidSpec>;
#[doc = "See `scedma.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L100>`__ (line numbers are approximate)"]
pub mod sfr_xch_segid;
#[doc = "SFR_XCH_SEGSTART (rw) register accessor: See `scedma.sv#L101 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L101>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_segstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_segstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_segstart`] module"]
#[doc(alias = "SFR_XCH_SEGSTART")]
pub type SfrXchSegstart = crate::Reg<sfr_xch_segstart::SfrXchSegstartSpec>;
#[doc = "See `scedma.sv#L101 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L101>`__ (line numbers are approximate)"]
pub mod sfr_xch_segstart;
#[doc = "SFR_XCH_TRANSIZE (rw) register accessor: See `scedma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_transize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_transize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_xch_transize`] module"]
#[doc(alias = "SFR_XCH_TRANSIZE")]
pub type SfrXchTransize = crate::Reg<sfr_xch_transize::SfrXchTransizeSpec>;
#[doc = "See `scedma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L102>`__ (line numbers are approximate)"]
pub mod sfr_xch_transize;
#[doc = "SFR_SCH_FUNC (rw) register accessor: See `scedma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_func`] module"]
#[doc(alias = "SFR_SCH_FUNC")]
pub type SfrSchFunc = crate::Reg<sfr_sch_func::SfrSchFuncSpec>;
#[doc = "See `scedma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L104>`__ (line numbers are approximate)"]
pub mod sfr_sch_func;
#[doc = "SFR_SCH_OPT (rw) register accessor: See `scedma.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_opt`] module"]
#[doc(alias = "SFR_SCH_OPT")]
pub type SfrSchOpt = crate::Reg<sfr_sch_opt::SfrSchOptSpec>;
#[doc = "See `scedma.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L105>`__ (line numbers are approximate)"]
pub mod sfr_sch_opt;
#[doc = "SFR_SCH_AXSTART (rw) register accessor: See `scedma.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_axstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_axstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_axstart`] module"]
#[doc(alias = "SFR_SCH_AXSTART")]
pub type SfrSchAxstart = crate::Reg<sfr_sch_axstart::SfrSchAxstartSpec>;
#[doc = "See `scedma.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L106>`__ (line numbers are approximate)"]
pub mod sfr_sch_axstart;
#[doc = "SFR_SCH_SEGID (rw) register accessor: See `scedma.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_segid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_segid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_segid`] module"]
#[doc(alias = "SFR_SCH_SEGID")]
pub type SfrSchSegid = crate::Reg<sfr_sch_segid::SfrSchSegidSpec>;
#[doc = "See `scedma.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L107>`__ (line numbers are approximate)"]
pub mod sfr_sch_segid;
#[doc = "SFR_SCH_SEGSTART (rw) register accessor: See `scedma.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L108>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_segstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_segstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_segstart`] module"]
#[doc(alias = "SFR_SCH_SEGSTART")]
pub type SfrSchSegstart = crate::Reg<sfr_sch_segstart::SfrSchSegstartSpec>;
#[doc = "See `scedma.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L108>`__ (line numbers are approximate)"]
pub mod sfr_sch_segstart;
#[doc = "SFR_SCH_TRANSIZE (rw) register accessor: See `scedma.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L109>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sch_transize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sch_transize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_sch_transize`] module"]
#[doc(alias = "SFR_SCH_TRANSIZE")]
pub type SfrSchTransize = crate::Reg<sfr_sch_transize::SfrSchTransizeSpec>;
#[doc = "See `scedma.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L109>`__ (line numbers are approximate)"]
pub mod sfr_sch_transize;
#[doc = "SFR_ICH_OPT (rw) register accessor: See `scedma.sv#L111 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L111>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_opt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_opt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ich_opt`] module"]
#[doc(alias = "SFR_ICH_OPT")]
pub type SfrIchOpt = crate::Reg<sfr_ich_opt::SfrIchOptSpec>;
#[doc = "See `scedma.sv#L111 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L111>`__ (line numbers are approximate)"]
pub mod sfr_ich_opt;
#[doc = "SFR_ICH_SEGID (rw) register accessor: See `scedma.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L112>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_segid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_segid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ich_segid`] module"]
#[doc(alias = "SFR_ICH_SEGID")]
pub type SfrIchSegid = crate::Reg<sfr_ich_segid::SfrIchSegidSpec>;
#[doc = "See `scedma.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L112>`__ (line numbers are approximate)"]
pub mod sfr_ich_segid;
#[doc = "SFR_ICH_RPSTART (rw) register accessor: See `scedma.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L113>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_rpstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_rpstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ich_rpstart`] module"]
#[doc(alias = "SFR_ICH_RPSTART")]
pub type SfrIchRpstart = crate::Reg<sfr_ich_rpstart::SfrIchRpstartSpec>;
#[doc = "See `scedma.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L113>`__ (line numbers are approximate)"]
pub mod sfr_ich_rpstart;
#[doc = "SFR_ICH_WPSTART (rw) register accessor: See `scedma.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L114>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_wpstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_wpstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ich_wpstart`] module"]
#[doc(alias = "SFR_ICH_WPSTART")]
pub type SfrIchWpstart = crate::Reg<sfr_ich_wpstart::SfrIchWpstartSpec>;
#[doc = "See `scedma.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L114>`__ (line numbers are approximate)"]
pub mod sfr_ich_wpstart;
#[doc = "SFR_ICH_TRANSIZE (rw) register accessor: See `scedma.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L115>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_transize::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_transize::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_ich_transize`] module"]
#[doc(alias = "SFR_ICH_TRANSIZE")]
pub type SfrIchTransize = crate::Reg<sfr_ich_transize::SfrIchTransizeSpec>;
#[doc = "See `scedma.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L115>`__ (line numbers are approximate)"]
pub mod sfr_ich_transize;
#[doc = "SFR_WDATABYPASS_MODE (rw) register accessor: See `scedma.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_wdatabypass_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_wdatabypass_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_wdatabypass_mode`] module"]
#[doc(alias = "SFR_WDATABYPASS_MODE")]
pub type SfrWdatabypassMode = crate::Reg<sfr_wdatabypass_mode::SfrWdatabypassModeSpec>;
#[doc = "See `scedma.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L117>`__ (line numbers are approximate)"]
pub mod sfr_wdatabypass_mode;
#[doc = "SFR_WDATABYPASS_DATA (rw) register accessor: See `scedma.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L118>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_wdatabypass_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_wdatabypass_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_wdatabypass_data`] module"]
#[doc(alias = "SFR_WDATABYPASS_DATA")]
pub type SfrWdatabypassData = crate::Reg<sfr_wdatabypass_data::SfrWdatabypassDataSpec>;
#[doc = "See `scedma.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L118>`__ (line numbers are approximate)"]
pub mod sfr_wdatabypass_data;
