#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ev_soft: EvSoft,
    ev_edge_triggered: EvEdgeTriggered,
    ev_polarity: EvPolarity,
    ev_status: EvStatus,
    ev_pending: EvPending,
    ev_enable: EvEnable,
}
impl RegisterBlock {
    #[doc = "0x00 - Software interrupt trigger register. ) Bits set to `1` will trigger an interrupt. Interrupts trigger on write, but the value will persist in the register, allowing software to determine if a software interrupt was triggered by reading back the register. Software is responsible for clearing the register to 0. Repeated `1` writes without clearing will still trigger an interrupt."]
    #[inline(always)]
    pub const fn ev_soft(&self) -> &EvSoft {
        &self.ev_soft
    }
    #[doc = "0x04 - If a bit is set to 1, then the hardware trigger is edge-triggered"]
    #[inline(always)]
    pub const fn ev_edge_triggered(&self) -> &EvEdgeTriggered {
        &self.ev_edge_triggered
    }
    #[doc = "0x08 - If a bit is set to 1, then the polarity is rising edge triggered; 0 is falling edge triggered. Bit is ignored if `edge_triggered` is 0."]
    #[inline(always)]
    pub const fn ev_polarity(&self) -> &EvPolarity {
        &self.ev_polarity
    }
    #[doc = "0x0c - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub const fn ev_status(&self) -> &EvStatus {
        &self.ev_status
    }
    #[doc = "0x10 - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub const fn ev_pending(&self) -> &EvPending {
        &self.ev_pending
    }
    #[doc = "0x14 - `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub const fn ev_enable(&self) -> &EvEnable {
        &self.ev_enable
    }
}
#[doc = "EV_SOFT (rw) register accessor: Software interrupt trigger register. ) Bits set to `1` will trigger an interrupt. Interrupts trigger on write, but the value will persist in the register, allowing software to determine if a software interrupt was triggered by reading back the register. Software is responsible for clearing the register to 0. Repeated `1` writes without clearing will still trigger an interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_soft::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_soft::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_soft`] module"]
#[doc(alias = "EV_SOFT")]
pub type EvSoft = crate::Reg<ev_soft::EvSoftSpec>;
#[doc = "Software interrupt trigger register. ) Bits set to `1` will trigger an interrupt. Interrupts trigger on write, but the value will persist in the register, allowing software to determine if a software interrupt was triggered by reading back the register. Software is responsible for clearing the register to 0. Repeated `1` writes without clearing will still trigger an interrupt."]
pub mod ev_soft;
#[doc = "EV_EDGE_TRIGGERED (rw) register accessor: If a bit is set to 1, then the hardware trigger is edge-triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_edge_triggered::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_edge_triggered::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_edge_triggered`] module"]
#[doc(alias = "EV_EDGE_TRIGGERED")]
pub type EvEdgeTriggered = crate::Reg<ev_edge_triggered::EvEdgeTriggeredSpec>;
#[doc = "If a bit is set to 1, then the hardware trigger is edge-triggered"]
pub mod ev_edge_triggered;
#[doc = "EV_POLARITY (rw) register accessor: If a bit is set to 1, then the polarity is rising edge triggered; 0 is falling edge triggered. Bit is ignored if `edge_triggered` is 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_polarity::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_polarity::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_polarity`] module"]
#[doc(alias = "EV_POLARITY")]
pub type EvPolarity = crate::Reg<ev_polarity::EvPolaritySpec>;
#[doc = "If a bit is set to 1, then the polarity is rising edge triggered; 0 is falling edge triggered. Bit is ignored if `edge_triggered` is 0."]
pub mod ev_polarity;
#[doc = "EV_STATUS (rw) register accessor: `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_status`] module"]
#[doc(alias = "EV_STATUS")]
pub type EvStatus = crate::Reg<ev_status::EvStatusSpec>;
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub mod ev_status;
#[doc = "EV_PENDING (rw) register accessor: `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_pending`] module"]
#[doc(alias = "EV_PENDING")]
pub type EvPending = crate::Reg<ev_pending::EvPendingSpec>;
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub mod ev_pending;
#[doc = "EV_ENABLE (rw) register accessor: `1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ev_enable`] module"]
#[doc(alias = "EV_ENABLE")]
pub type EvEnable = crate::Reg<ev_enable::EvEnableSpec>;
#[doc = "`1` when a \"pwm3_ev\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub mod ev_enable;
