#[doc = "Register `CHNLREQMASKCLR` writer"]
pub type W = crate::W<ChnlreqmaskclrSpec>;
#[doc = "Field `CHNL_REQ_MASK_CLR` writer - CHNL_REQ_MASK_CLR"]
pub type ChnlReqMaskClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_REQ_MASK_CLR"]
    #[inline(always)]
    pub fn chnl_req_mask_clr(&mut self) -> ChnlReqMaskClrW<'_, ChnlreqmaskclrSpec> {
        ChnlReqMaskClrW::new(self, 0)
    }
}
#[doc = "DMA Channel Request Mask Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlreqmaskclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlreqmaskclrSpec;
impl crate::RegisterSpec for ChnlreqmaskclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnlreqmaskclr::W`](W) writer structure"]
impl crate::Writable for ChnlreqmaskclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLREQMASKCLR to value 0"]
impl crate::Resettable for ChnlreqmaskclrSpec {}
