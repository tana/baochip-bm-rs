#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sfr_wdata: SfrWdata,
    sfr_rdata: SfrRdata,
    sfr_status: SfrStatus,
    _reserved3: [u8; 0x0c],
    sfr_abort: SfrAbort,
    sfr_done: SfrDone,
}
impl RegisterBlock {
    #[doc = "0x00 - See `mbox_v0.1.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L105>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_wdata(&self) -> &SfrWdata {
        &self.sfr_wdata
    }
    #[doc = "0x04 - See `mbox_v0.1.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L106>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_rdata(&self) -> &SfrRdata {
        &self.sfr_rdata
    }
    #[doc = "0x08 - See `mbox_v0.1.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L107>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_status(&self) -> &SfrStatus {
        &self.sfr_status
    }
    #[doc = "0x18 - See `mbox_v0.1.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L108>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_abort(&self) -> &SfrAbort {
        &self.sfr_abort
    }
    #[doc = "0x1c - See `mbox_v0.1.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L109>`__ (line numbers are approximate)"]
    #[inline(always)]
    pub const fn sfr_done(&self) -> &SfrDone {
        &self.sfr_done
    }
}
#[doc = "SFR_WDATA (rw) register accessor: See `mbox_v0.1.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_wdata`] module"]
#[doc(alias = "SFR_WDATA")]
pub type SfrWdata = crate::Reg<sfr_wdata::SfrWdataSpec>;
#[doc = "See `mbox_v0.1.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L105>`__ (line numbers are approximate)"]
pub mod sfr_wdata;
#[doc = "SFR_RDATA (rw) register accessor: See `mbox_v0.1.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_rdata`] module"]
#[doc(alias = "SFR_RDATA")]
pub type SfrRdata = crate::Reg<sfr_rdata::SfrRdataSpec>;
#[doc = "See `mbox_v0.1.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L106>`__ (line numbers are approximate)"]
pub mod sfr_rdata;
#[doc = "SFR_STATUS (rw) register accessor: See `mbox_v0.1.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_status`] module"]
#[doc(alias = "SFR_STATUS")]
pub type SfrStatus = crate::Reg<sfr_status::SfrStatusSpec>;
#[doc = "See `mbox_v0.1.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L107>`__ (line numbers are approximate)"]
pub mod sfr_status;
#[doc = "SFR_ABORT (rw) register accessor: See `mbox_v0.1.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L108>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_abort`] module"]
#[doc(alias = "SFR_ABORT")]
pub type SfrAbort = crate::Reg<sfr_abort::SfrAbortSpec>;
#[doc = "See `mbox_v0.1.sv#L108 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L108>`__ (line numbers are approximate)"]
pub mod sfr_abort;
#[doc = "SFR_DONE (rw) register accessor: See `mbox_v0.1.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L109>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr_done`] module"]
#[doc(alias = "SFR_DONE")]
pub type SfrDone = crate::Reg<sfr_done::SfrDoneSpec>;
#[doc = "See `mbox_v0.1.sv#L109 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L109>`__ (line numbers are approximate)"]
pub mod sfr_done;
