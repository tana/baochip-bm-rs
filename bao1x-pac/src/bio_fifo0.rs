#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sfr_flevel: SfrFlevel,
    sfr_txf0: SfrTxf0,
    _reserved2: [u8; 0x0c],
    sfr_rxf0: SfrRxf0,
    _reserved3: [u8; 0x14],
    sfr_event_set: SfrEventSet,
    sfr_event_clr: SfrEventClr,
    sfr_event_status: SfrEventStatus,
}
impl RegisterBlock {
    #[doc = "0x0c - See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_flevel(&self) -> &SfrFlevel {
        &self.sfr_flevel
    }
    #[doc = "0x10 - See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_txf0(&self) -> &SfrTxf0 {
        &self.sfr_txf0
    }
    #[doc = "0x20 - See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rxf0(&self) -> &SfrRxf0 {
        &self.sfr_rxf0
    }
    #[doc = "0x38 - See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_set(&self) -> &SfrEventSet {
        &self.sfr_event_set
    }
    #[doc = "0x3c - See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_clr(&self) -> &SfrEventClr {
        &self.sfr_event_clr
    }
    #[doc = "0x40 - See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_event_status(&self) -> &SfrEventStatus {
        &self.sfr_event_status
    }
}
#[doc = "SFR_FLEVEL (rw) register accessor: See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_flevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_flevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_flevel`] module"]
#[doc(alias = "SFR_FLEVEL")]
pub type SfrFlevel = crate::Reg<sfr_flevel::SfrFlevelSpec>;
#[doc = "See `bio_bdma.sv#L492 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L492>`__ (line numbers are approximate)"]
pub mod sfr_flevel;
#[doc = "SFR_TXF0 (rw) register accessor: See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_txf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_txf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_txf0`] module"]
#[doc(alias = "SFR_TXF0")]
pub type SfrTxf0 = crate::Reg<sfr_txf0::SfrTxf0Spec>;
#[doc = "See `bio_bdma.sv#L493 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L493>`__ (line numbers are approximate)"]
pub mod sfr_txf0;
#[doc = "SFR_RXF0 (rw) register accessor: See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rxf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rxf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rxf0`] module"]
#[doc(alias = "SFR_RXF0")]
pub type SfrRxf0 = crate::Reg<sfr_rxf0::SfrRxf0Spec>;
#[doc = "See `bio_bdma.sv#L497 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L497>`__ (line numbers are approximate)"]
pub mod sfr_rxf0;
#[doc = "SFR_EVENT_SET (rw) register accessor: See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_set`] module"]
#[doc(alias = "SFR_EVENT_SET")]
pub type SfrEventSet = crate::Reg<sfr_event_set::SfrEventSetSpec>;
#[doc = "See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)"]
pub mod sfr_event_set;
#[doc = "SFR_EVENT_CLR (rw) register accessor: See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_clr`] module"]
#[doc(alias = "SFR_EVENT_CLR")]
pub type SfrEventClr = crate::Reg<sfr_event_clr::SfrEventClrSpec>;
#[doc = "See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)"]
pub mod sfr_event_clr;
#[doc = "SFR_EVENT_STATUS (rw) register accessor: See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_event_status`] module"]
#[doc(alias = "SFR_EVENT_STATUS")]
pub type SfrEventStatus = crate::Reg<sfr_event_status::SfrEventStatusSpec>;
#[doc = "See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)"]
pub mod sfr_event_status;
