#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    heartbeat: Heartbeat,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn heartbeat(&self) -> &Heartbeat {
        &self.heartbeat
    }
}
#[doc = "CONTROL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = ""]
pub mod control;
#[doc = "HEARTBEAT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`heartbeat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`heartbeat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@heartbeat`] module"]
#[doc(alias = "HEARTBEAT")]
pub type Heartbeat = crate::Reg<heartbeat::HeartbeatSpec>;
#[doc = ""]
pub mod heartbeat;
