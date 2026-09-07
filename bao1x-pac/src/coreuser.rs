#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    status: Status,
    map_lo: MapLo,
    map_hi: MapHi,
    uservalue: Uservalue,
    protect: Protect,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn map_lo(&self) -> &MapLo {
        &self.map_lo
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn map_hi(&self) -> &MapHi {
        &self.map_hi
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn uservalue(&self) -> &Uservalue {
        &self.uservalue
    }
    #[doc = "0x14 - Writing `1` to this bit prevents any further updates to CoreUser configuration status. Can only be reversed with a system reset."]
    #[inline(always)]
    pub const fn protect(&self) -> &Protect {
        &self.protect
    }
}
#[doc = "CONTROL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = ""]
pub mod control;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = ""]
pub mod status;
#[doc = "MAP_LO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`map_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`map_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@map_lo`] module"]
#[doc(alias = "MAP_LO")]
pub type MapLo = crate::Reg<map_lo::MapLoSpec>;
#[doc = ""]
pub mod map_lo;
#[doc = "MAP_HI (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`map_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`map_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@map_hi`] module"]
#[doc(alias = "MAP_HI")]
pub type MapHi = crate::Reg<map_hi::MapHiSpec>;
#[doc = ""]
pub mod map_hi;
#[doc = "USERVALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`uservalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uservalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uservalue`] module"]
#[doc(alias = "USERVALUE")]
pub type Uservalue = crate::Reg<uservalue::UservalueSpec>;
#[doc = ""]
pub mod uservalue;
#[doc = "PROTECT (rw) register accessor: Writing `1` to this bit prevents any further updates to CoreUser configuration status. Can only be reversed with a system reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`protect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protect`] module"]
#[doc(alias = "PROTECT")]
pub type Protect = crate::Reg<protect::ProtectSpec>;
#[doc = "Writing `1` to this bit prevents any further updates to CoreUser configuration status. Can only be reversed with a system reset."]
pub mod protect;
