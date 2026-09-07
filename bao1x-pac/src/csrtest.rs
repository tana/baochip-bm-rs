#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wtest: Wtest,
    rtest: Rtest,
}
impl RegisterBlock {
    #[doc = "0x00 - Write test data here"]
    #[inline(always)]
    pub const fn wtest(&self) -> &Wtest {
        &self.wtest
    }
    #[doc = "0x04 - Read test data here"]
    #[inline(always)]
    pub const fn rtest(&self) -> &Rtest {
        &self.rtest
    }
}
#[doc = "WTEST (rw) register accessor: Write test data here\n\nYou can [`read`](crate::Reg::read) this register and get [`wtest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtest`] module"]
#[doc(alias = "WTEST")]
pub type Wtest = crate::Reg<wtest::WtestSpec>;
#[doc = "Write test data here"]
pub mod wtest;
#[doc = "RTEST (rw) register accessor: Read test data here\n\nYou can [`read`](crate::Reg::read) this register and get [`rtest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtest`] module"]
#[doc(alias = "RTEST")]
pub type Rtest = crate::Reg<rtest::RtestSpec>;
#[doc = "Read test data here"]
pub mod rtest;
