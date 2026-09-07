#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    cfg: Cfg,
    ctrlbaseptr: Ctrlbaseptr,
    altctrlbaseptr: Altctrlbaseptr,
    dma_waitonreq_status: DmaWaitonreqStatus,
    chnlswrequest: Chnlswrequest,
    chnluseburstset: Chnluseburstset,
    chnluseburstclr: Chnluseburstclr,
    chnlreqmaskset: Chnlreqmaskset,
    chnlreqmaskclr: Chnlreqmaskclr,
    chnlenableset: Chnlenableset,
    chnlenableclr: Chnlenableclr,
    chnlprialtset: Chnlprialtset,
    chnlprialtclr: Chnlprialtclr,
    chnlpriorityset: Chnlpriorityset,
    chnlpriorityclr: Chnlpriorityclr,
    _reserved16: [u8; 0x0c],
    errclr: Errclr,
    _reserved17: [u8; 0x0f90],
    periph_id_0: PeriphId0,
    periph_id_1: PeriphId1,
    periph_id_2: PeriphId2,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - DMA Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn ctrlbaseptr(&self) -> &Ctrlbaseptr {
        &self.ctrlbaseptr
    }
    #[doc = "0x0c - DMA Channel Alternate Control Data Base Pointer Register"]
    #[inline(always)]
    pub const fn altctrlbaseptr(&self) -> &Altctrlbaseptr {
        &self.altctrlbaseptr
    }
    #[doc = "0x10 - Channel wait on request status"]
    #[inline(always)]
    pub const fn dma_waitonreq_status(&self) -> &DmaWaitonreqStatus {
        &self.dma_waitonreq_status
    }
    #[doc = "0x14 - DMA Channel Software Request Register"]
    #[inline(always)]
    pub const fn chnlswrequest(&self) -> &Chnlswrequest {
        &self.chnlswrequest
    }
    #[doc = "0x18 - DMA Channel Useburst Set Register"]
    #[inline(always)]
    pub const fn chnluseburstset(&self) -> &Chnluseburstset {
        &self.chnluseburstset
    }
    #[doc = "0x1c - DMA Channel Useburst Clear Register"]
    #[inline(always)]
    pub const fn chnluseburstclr(&self) -> &Chnluseburstclr {
        &self.chnluseburstclr
    }
    #[doc = "0x20 - DMA Channel Request Mask Set Register"]
    #[inline(always)]
    pub const fn chnlreqmaskset(&self) -> &Chnlreqmaskset {
        &self.chnlreqmaskset
    }
    #[doc = "0x24 - DMA Channel Request Mask Clear Register"]
    #[inline(always)]
    pub const fn chnlreqmaskclr(&self) -> &Chnlreqmaskclr {
        &self.chnlreqmaskclr
    }
    #[doc = "0x28 - DMA Channel Enable Set Register"]
    #[inline(always)]
    pub const fn chnlenableset(&self) -> &Chnlenableset {
        &self.chnlenableset
    }
    #[doc = "0x2c - DMA Channel Enable Clear Register"]
    #[inline(always)]
    pub const fn chnlenableclr(&self) -> &Chnlenableclr {
        &self.chnlenableclr
    }
    #[doc = "0x30 - DMA Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub const fn chnlprialtset(&self) -> &Chnlprialtset {
        &self.chnlprialtset
    }
    #[doc = "0x34 - DMA Channel Primary-Alternate Clear Register"]
    #[inline(always)]
    pub const fn chnlprialtclr(&self) -> &Chnlprialtclr {
        &self.chnlprialtclr
    }
    #[doc = "0x38 - DMA Channel Priority Set Register"]
    #[inline(always)]
    pub const fn chnlpriorityset(&self) -> &Chnlpriorityset {
        &self.chnlpriorityset
    }
    #[doc = "0x3c - DMA Channel Priority Clear Register"]
    #[inline(always)]
    pub const fn chnlpriorityclr(&self) -> &Chnlpriorityclr {
        &self.chnlpriorityclr
    }
    #[doc = "0x4c - DMA Bus Error Clear Register"]
    #[inline(always)]
    pub const fn errclr(&self) -> &Errclr {
        &self.errclr
    }
    #[doc = "0xfe0 - Peripheral ID byte 0"]
    #[inline(always)]
    pub const fn periph_id_0(&self) -> &PeriphId0 {
        &self.periph_id_0
    }
    #[doc = "0xfe4 - Peripheral ID byte 1"]
    #[inline(always)]
    pub const fn periph_id_1(&self) -> &PeriphId1 {
        &self.periph_id_1
    }
    #[doc = "0xfe8 - Peripheral ID byte 2"]
    #[inline(always)]
    pub const fn periph_id_2(&self) -> &PeriphId2 {
        &self.periph_id_2
    }
}
#[doc = "STATUS (r) register accessor: DMA Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "CFG (w) register accessor: DMA Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMA Configuration Register"]
pub mod cfg;
#[doc = "CTRLBASEPTR (rw) register accessor: DMA Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbaseptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbaseptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbaseptr`] module"]
#[doc(alias = "CTRLBASEPTR")]
pub type Ctrlbaseptr = crate::Reg<ctrlbaseptr::CtrlbaseptrSpec>;
#[doc = "DMA Control Data Base Pointer Register"]
pub mod ctrlbaseptr;
#[doc = "ALTCTRLBASEPTR (r) register accessor: DMA Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altctrlbaseptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altctrlbaseptr`] module"]
#[doc(alias = "ALTCTRLBASEPTR")]
pub type Altctrlbaseptr = crate::Reg<altctrlbaseptr::AltctrlbaseptrSpec>;
#[doc = "DMA Channel Alternate Control Data Base Pointer Register"]
pub mod altctrlbaseptr;
#[doc = "DMA_WAITONREQ_STATUS (r) register accessor: Channel wait on request status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_waitonreq_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_waitonreq_status`] module"]
#[doc(alias = "DMA_WAITONREQ_STATUS")]
pub type DmaWaitonreqStatus = crate::Reg<dma_waitonreq_status::DmaWaitonreqStatusSpec>;
#[doc = "Channel wait on request status"]
pub mod dma_waitonreq_status;
#[doc = "CHNLSWREQUEST (w) register accessor: DMA Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlswrequest::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlswrequest`] module"]
#[doc(alias = "CHNLSWREQUEST")]
pub type Chnlswrequest = crate::Reg<chnlswrequest::ChnlswrequestSpec>;
#[doc = "DMA Channel Software Request Register"]
pub mod chnlswrequest;
#[doc = "CHNLUSEBURSTSET (rw) register accessor: DMA Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnluseburstset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnluseburstset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnluseburstset`] module"]
#[doc(alias = "CHNLUSEBURSTSET")]
pub type Chnluseburstset = crate::Reg<chnluseburstset::ChnluseburstsetSpec>;
#[doc = "DMA Channel Useburst Set Register"]
pub mod chnluseburstset;
#[doc = "CHNLUSEBURSTCLR (w) register accessor: DMA Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnluseburstclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnluseburstclr`] module"]
#[doc(alias = "CHNLUSEBURSTCLR")]
pub type Chnluseburstclr = crate::Reg<chnluseburstclr::ChnluseburstclrSpec>;
#[doc = "DMA Channel Useburst Clear Register"]
pub mod chnluseburstclr;
#[doc = "CHNLREQMASKSET (rw) register accessor: DMA Channel Request Mask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlreqmaskset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlreqmaskset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlreqmaskset`] module"]
#[doc(alias = "CHNLREQMASKSET")]
pub type Chnlreqmaskset = crate::Reg<chnlreqmaskset::ChnlreqmasksetSpec>;
#[doc = "DMA Channel Request Mask Set Register"]
pub mod chnlreqmaskset;
#[doc = "CHNLREQMASKCLR (w) register accessor: DMA Channel Request Mask Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlreqmaskclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlreqmaskclr`] module"]
#[doc(alias = "CHNLREQMASKCLR")]
pub type Chnlreqmaskclr = crate::Reg<chnlreqmaskclr::ChnlreqmaskclrSpec>;
#[doc = "DMA Channel Request Mask Clear Register"]
pub mod chnlreqmaskclr;
#[doc = "CHNLENABLESET (rw) register accessor: DMA Channel Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlenableset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlenableset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlenableset`] module"]
#[doc(alias = "CHNLENABLESET")]
pub type Chnlenableset = crate::Reg<chnlenableset::ChnlenablesetSpec>;
#[doc = "DMA Channel Enable Set Register"]
pub mod chnlenableset;
#[doc = "CHNLENABLECLR (w) register accessor: DMA Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlenableclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlenableclr`] module"]
#[doc(alias = "CHNLENABLECLR")]
pub type Chnlenableclr = crate::Reg<chnlenableclr::ChnlenableclrSpec>;
#[doc = "DMA Channel Enable Clear Register"]
pub mod chnlenableclr;
#[doc = "CHNLPRIALTSET (rw) register accessor: DMA Channel Primary-Alternate Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlprialtset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlprialtset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlprialtset`] module"]
#[doc(alias = "CHNLPRIALTSET")]
pub type Chnlprialtset = crate::Reg<chnlprialtset::ChnlprialtsetSpec>;
#[doc = "DMA Channel Primary-Alternate Set Register"]
pub mod chnlprialtset;
#[doc = "CHNLPRIALTCLR (w) register accessor: DMA Channel Primary-Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlprialtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlprialtclr`] module"]
#[doc(alias = "CHNLPRIALTCLR")]
pub type Chnlprialtclr = crate::Reg<chnlprialtclr::ChnlprialtclrSpec>;
#[doc = "DMA Channel Primary-Alternate Clear Register"]
pub mod chnlprialtclr;
#[doc = "CHNLPRIORITYSET (rw) register accessor: DMA Channel Priority Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlpriorityset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlpriorityset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlpriorityset`] module"]
#[doc(alias = "CHNLPRIORITYSET")]
pub type Chnlpriorityset = crate::Reg<chnlpriorityset::ChnlprioritysetSpec>;
#[doc = "DMA Channel Priority Set Register"]
pub mod chnlpriorityset;
#[doc = "CHNLPRIORITYCLR (w) register accessor: DMA Channel Priority Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlpriorityclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chnlpriorityclr`] module"]
#[doc(alias = "CHNLPRIORITYCLR")]
pub type Chnlpriorityclr = crate::Reg<chnlpriorityclr::ChnlpriorityclrSpec>;
#[doc = "DMA Channel Priority Clear Register"]
pub mod chnlpriorityclr;
#[doc = "ERRCLR (rw) register accessor: DMA Bus Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errclr`] module"]
#[doc(alias = "ERRCLR")]
pub type Errclr = crate::Reg<errclr::ErrclrSpec>;
#[doc = "DMA Bus Error Clear Register"]
pub mod errclr;
#[doc = "PERIPH_ID_0 (rw) register accessor: Peripheral ID byte 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_0`] module"]
#[doc(alias = "PERIPH_ID_0")]
pub type PeriphId0 = crate::Reg<periph_id_0::PeriphId0Spec>;
#[doc = "Peripheral ID byte 0"]
pub mod periph_id_0;
#[doc = "PERIPH_ID_1 (rw) register accessor: Peripheral ID byte 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_1`] module"]
#[doc(alias = "PERIPH_ID_1")]
pub type PeriphId1 = crate::Reg<periph_id_1::PeriphId1Spec>;
#[doc = "Peripheral ID byte 1"]
pub mod periph_id_1;
#[doc = "PERIPH_ID_2 (rw) register accessor: Peripheral ID byte 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_id_2`] module"]
#[doc(alias = "PERIPH_ID_2")]
pub type PeriphId2 = crate::Reg<periph_id_2::PeriphId2Spec>;
#[doc = "Peripheral ID byte 2"]
pub mod periph_id_2;
