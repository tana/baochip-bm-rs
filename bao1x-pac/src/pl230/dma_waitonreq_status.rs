#[doc = "Register `DMA_WAITONREQ_STATUS` reader"]
pub type R = crate::R<DmaWaitonreqStatusSpec>;
#[doc = "Field `DMA_WAITONREQ_STATUS` reader - Wait on request status, one bit per channel"]
pub type DmaWaitonreqStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Wait on request status, one bit per channel"]
    #[inline(always)]
    pub fn dma_waitonreq_status(&self) -> DmaWaitonreqStatusR {
        DmaWaitonreqStatusR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Channel wait on request status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_waitonreq_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaWaitonreqStatusSpec;
impl crate::RegisterSpec for DmaWaitonreqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_waitonreq_status::R`](R) reader structure"]
impl crate::Readable for DmaWaitonreqStatusSpec {}
#[doc = "`reset()` method sets DMA_WAITONREQ_STATUS to value 0"]
impl crate::Resettable for DmaWaitonreqStatusSpec {}
