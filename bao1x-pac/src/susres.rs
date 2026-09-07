#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    resume_time1: ResumeTime1,
    resume_time0: ResumeTime0,
    time1: Time1,
    time0: Time0,
    status: Status,
    state: State,
    interrupt: Interrupt,
    ev_status: EvStatus,
    ev_pending: EvPending,
    ev_enable: EvEnable,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Bits 32-63 of `SUSRES_RESUME_TIME`. Elapsed time to load. Loaded upon writing `1` to the load bit in the control register. This will immediately affect the msleep extension."]
    #[inline(always)]
    pub const fn resume_time1(&self) -> &ResumeTime1 {
        &self.resume_time1
    }
    #[doc = "0x08 - Bits 0-31 of `SUSRES_RESUME_TIME`."]
    #[inline(always)]
    pub const fn resume_time0(&self) -> &ResumeTime0 {
        &self.resume_time0
    }
    #[doc = "0x0c - Bits 32-63 of `SUSRES_TIME`. Cycle-accurate mirror copy of time in systicks, from the TickTimer"]
    #[inline(always)]
    pub const fn time1(&self) -> &Time1 {
        &self.time1
    }
    #[doc = "0x10 - Bits 0-31 of `SUSRES_TIME`."]
    #[inline(always)]
    pub const fn time0(&self) -> &Time0 {
        &self.time0
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x20 - This register contains the current raw level of the soft_int event trigger. Writes to this register have no effect."]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EvStatus {
        &self.ev_status
    }
    #[doc = "0x24 - When a soft_int event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EvPending {
        &self.ev_pending
    }
    #[doc = "0x28 - This register enables the corresponding soft_int events. Write a ``0`` to this register to disable individual events."]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EvEnable {
        &self.ev_enable
    }
}
#[doc = "CONTROL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = ""]
pub mod control;
#[doc = "RESUME_TIME1 (rw) register accessor: Bits 32-63 of `SUSRES_RESUME_TIME`. Elapsed time to load. Loaded upon writing `1` to the load bit in the control register. This will immediately affect the msleep extension.\n\nYou can [`read`](crate::Reg::read) this register and get [`resume_time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resume_time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resume_time1`] module"]
#[doc(alias = "RESUME_TIME1")]
pub type ResumeTime1 = crate::Reg<resume_time1::ResumeTime1Spec>;
#[doc = "Bits 32-63 of `SUSRES_RESUME_TIME`. Elapsed time to load. Loaded upon writing `1` to the load bit in the control register. This will immediately affect the msleep extension."]
pub mod resume_time1;
#[doc = "RESUME_TIME0 (rw) register accessor: Bits 0-31 of `SUSRES_RESUME_TIME`.\n\nYou can [`read`](crate::Reg::read) this register and get [`resume_time0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resume_time0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resume_time0`] module"]
#[doc(alias = "RESUME_TIME0")]
pub type ResumeTime0 = crate::Reg<resume_time0::ResumeTime0Spec>;
#[doc = "Bits 0-31 of `SUSRES_RESUME_TIME`."]
pub mod resume_time0;
#[doc = "TIME1 (rw) register accessor: Bits 32-63 of `SUSRES_TIME`. Cycle-accurate mirror copy of time in systicks, from the TickTimer\n\nYou can [`read`](crate::Reg::read) this register and get [`time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time1`] module"]
#[doc(alias = "TIME1")]
pub type Time1 = crate::Reg<time1::Time1Spec>;
#[doc = "Bits 32-63 of `SUSRES_TIME`. Cycle-accurate mirror copy of time in systicks, from the TickTimer"]
pub mod time1;
#[doc = "TIME0 (rw) register accessor: Bits 0-31 of `SUSRES_TIME`.\n\nYou can [`read`](crate::Reg::read) this register and get [`time0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time0`] module"]
#[doc(alias = "TIME0")]
pub type Time0 = crate::Reg<time0::Time0Spec>;
#[doc = "Bits 0-31 of `SUSRES_TIME`."]
pub mod time0;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = ""]
pub mod status;
#[doc = "STATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = ""]
pub mod state;
#[doc = "INTERRUPT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`] module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = ""]
pub mod interrupt;
#[doc = "EV_STATUS (rw) register accessor: This register contains the current raw level of the soft_int event trigger. Writes to this register have no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`] module"]
#[doc(alias = "EV_STATUS")]
pub type EvStatus = crate::Reg<ev_status::EvStatusSpec>;
#[doc = "This register contains the current raw level of the soft_int event trigger. Writes to this register have no effect."]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: When a soft_int event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`] module"]
#[doc(alias = "EV_PENDING")]
pub type EvPending = crate::Reg<ev_pending::EvPendingSpec>;
#[doc = "When a soft_int event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: This register enables the corresponding soft_int events. Write a ``0`` to this register to disable individual events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`] module"]
#[doc(alias = "EV_ENABLE")]
pub type EvEnable = crate::Reg<ev_enable::EvEnableSpec>;
#[doc = "This register enables the corresponding soft_int events. Write a ``0`` to this register to disable individual events."]
pub mod ev_enable;
