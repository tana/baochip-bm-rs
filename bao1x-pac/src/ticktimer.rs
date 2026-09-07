#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    time1: Time1,
    time0: Time0,
    msleep_target1: MsleepTarget1,
    msleep_target0: MsleepTarget0,
    ev_status: EvStatus,
    ev_pending: EvPending,
    ev_enable: EvEnable,
    clocks_per_tick: ClocksPerTick,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Bits 32-63 of `TICKTIMER_TIME`. Elapsed time in systicks"]
    #[inline(always)]
    pub const fn time1(&self) -> &Time1 {
        &self.time1
    }
    #[doc = "0x08 - Bits 0-31 of `TICKTIMER_TIME`."]
    #[inline(always)]
    pub const fn time0(&self) -> &Time0 {
        &self.time0
    }
    #[doc = "0x0c - Bits 32-63 of `TICKTIMER_MSLEEP_TARGET`. Target time in 1.0ms ticks"]
    #[inline(always)]
    pub const fn msleep_target1(&self) -> &MsleepTarget1 {
        &self.msleep_target1
    }
    #[doc = "0x10 - Bits 0-31 of `TICKTIMER_MSLEEP_TARGET`."]
    #[inline(always)]
    pub const fn msleep_target0(&self) -> &MsleepTarget0 {
        &self.msleep_target0
    }
    #[doc = "0x14 - This register contains the current raw level of the alarm event trigger. Writes to this register have no effect."]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EvStatus {
        &self.ev_status
    }
    #[doc = "0x18 - When a alarm event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EvPending {
        &self.ev_pending
    }
    #[doc = "0x1c - This register enables the corresponding alarm events. Write a ``0`` to this register to disable individual events."]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EvEnable {
        &self.ev_enable
    }
    #[doc = "0x20 - Clocks per tick, defaults to 800000"]
    #[inline(always)]
    pub const fn clocks_per_tick(&self) -> &ClocksPerTick {
        &self.clocks_per_tick
    }
}
#[doc = "CONTROL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = ""]
pub mod control;
#[doc = "TIME1 (rw) register accessor: Bits 32-63 of `TICKTIMER_TIME`. Elapsed time in systicks\n\nYou can [`read`](crate::Reg::read) this register and get [`time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time1`] module"]
#[doc(alias = "TIME1")]
pub type Time1 = crate::Reg<time1::Time1Spec>;
#[doc = "Bits 32-63 of `TICKTIMER_TIME`. Elapsed time in systicks"]
pub mod time1;
#[doc = "TIME0 (rw) register accessor: Bits 0-31 of `TICKTIMER_TIME`.\n\nYou can [`read`](crate::Reg::read) this register and get [`time0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time0`] module"]
#[doc(alias = "TIME0")]
pub type Time0 = crate::Reg<time0::Time0Spec>;
#[doc = "Bits 0-31 of `TICKTIMER_TIME`."]
pub mod time0;
#[doc = "MSLEEP_TARGET1 (rw) register accessor: Bits 32-63 of `TICKTIMER_MSLEEP_TARGET`. Target time in 1.0ms ticks\n\nYou can [`read`](crate::Reg::read) this register and get [`msleep_target1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep_target1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msleep_target1`] module"]
#[doc(alias = "MSLEEP_TARGET1")]
pub type MsleepTarget1 = crate::Reg<msleep_target1::MsleepTarget1Spec>;
#[doc = "Bits 32-63 of `TICKTIMER_MSLEEP_TARGET`. Target time in 1.0ms ticks"]
pub mod msleep_target1;
#[doc = "MSLEEP_TARGET0 (rw) register accessor: Bits 0-31 of `TICKTIMER_MSLEEP_TARGET`.\n\nYou can [`read`](crate::Reg::read) this register and get [`msleep_target0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msleep_target0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msleep_target0`] module"]
#[doc(alias = "MSLEEP_TARGET0")]
pub type MsleepTarget0 = crate::Reg<msleep_target0::MsleepTarget0Spec>;
#[doc = "Bits 0-31 of `TICKTIMER_MSLEEP_TARGET`."]
pub mod msleep_target0;
#[doc = "EV_STATUS (rw) register accessor: This register contains the current raw level of the alarm event trigger. Writes to this register have no effect.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`] module"]
#[doc(alias = "EV_STATUS")]
pub type EvStatus = crate::Reg<ev_status::EvStatusSpec>;
#[doc = "This register contains the current raw level of the alarm event trigger. Writes to this register have no effect."]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: When a alarm event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`] module"]
#[doc(alias = "EV_PENDING")]
pub type EvPending = crate::Reg<ev_pending::EvPendingSpec>;
#[doc = "When a alarm event occurs, the corresponding bit will be set in this register. To clear the Event, set the corresponding bit in this register."]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: This register enables the corresponding alarm events. Write a ``0`` to this register to disable individual events.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`] module"]
#[doc(alias = "EV_ENABLE")]
pub type EvEnable = crate::Reg<ev_enable::EvEnableSpec>;
#[doc = "This register enables the corresponding alarm events. Write a ``0`` to this register to disable individual events."]
pub mod ev_enable;
#[doc = "CLOCKS_PER_TICK (rw) register accessor: Clocks per tick, defaults to 800000\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_per_tick::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_per_tick::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clocks_per_tick`] module"]
#[doc(alias = "CLOCKS_PER_TICK")]
pub type ClocksPerTick = crate::Reg<clocks_per_tick::ClocksPerTickSpec>;
#[doc = "Clocks per tick, defaults to 800000"]
pub mod clocks_per_tick;
