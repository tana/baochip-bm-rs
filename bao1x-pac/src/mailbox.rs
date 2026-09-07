#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdata: Wdata,
    rdata: Rdata,
    ev_status: EvStatus,
    ev_pending: EvPending,
    ev_enable: EvEnable,
    status: Status,
    control: Control,
    done: Done,
    loopback: Loopback,
}
impl RegisterBlock {
    #[doc = "0x00 - Write data to outgoing FIFO."]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x04 - Read data from incoming FIFO."]
    #[inline(always)]
    pub const fn rdata(&self) -> &Rdata {
        &self.rdata
    }
    #[doc = "0x08 - Triggers if either `tx_err` or `rx_err` are asserted"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EvStatus {
        &self.ev_status
    }
    #[doc = "0x0c - Triggers if either `tx_err` or `rx_err` are asserted"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EvPending {
        &self.ev_pending
    }
    #[doc = "0x10 - Triggers if either `tx_err` or `rx_err` are asserted"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EvEnable {
        &self.ev_enable
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn done(&self) -> &Done {
        &self.done
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn loopback(&self) -> &Loopback {
        &self.loopback
    }
}
#[doc = "WDATA (rw) register accessor: Write data to outgoing FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`] module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "Write data to outgoing FIFO."]
pub mod wdata;
#[doc = "RDATA (rw) register accessor: Read data from incoming FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`] module"]
#[doc(alias = "RDATA")]
pub type Rdata = crate::Reg<rdata::RdataSpec>;
#[doc = "Read data from incoming FIFO."]
pub mod rdata;
#[doc = "EV_STATUS (rw) register accessor: Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`] module"]
#[doc(alias = "EV_STATUS")]
pub type EvStatus = crate::Reg<ev_status::EvStatusSpec>;
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted"]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`] module"]
#[doc(alias = "EV_PENDING")]
pub type EvPending = crate::Reg<ev_pending::EvPendingSpec>;
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted"]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`] module"]
#[doc(alias = "EV_ENABLE")]
pub type EvEnable = crate::Reg<ev_enable::EvEnableSpec>;
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted"]
pub mod ev_enable;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = ""]
pub mod status;
#[doc = "CONTROL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = ""]
pub mod control;
#[doc = "DONE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`done::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`done::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@done`] module"]
#[doc(alias = "DONE")]
pub type Done = crate::Reg<done::DoneSpec>;
#[doc = ""]
pub mod done;
#[doc = "LOOPBACK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`loopback::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loopback::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loopback`] module"]
#[doc(alias = "LOOPBACK")]
pub type Loopback = crate::Reg<loopback::LoopbackSpec>;
#[doc = ""]
pub mod loopback;
