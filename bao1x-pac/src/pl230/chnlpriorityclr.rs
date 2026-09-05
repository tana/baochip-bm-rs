#[doc = "Register `CHNLPRIORITYCLR` writer"]
pub type W = crate::W<ChnlpriorityclrSpec>;
#[doc = "Field `CHNL_PRIORITY_CLR` writer - CHNL_PRIORITY_CLR"]
pub type ChnlPriorityClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_PRIORITY_CLR"]
    #[inline(always)]
    pub fn chnl_priority_clr(&mut self) -> ChnlPriorityClrW<'_, ChnlpriorityclrSpec> {
        ChnlPriorityClrW::new(self, 0)
    }
}
#[doc = "DMA Channel Priority Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlpriorityclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlpriorityclrSpec;
impl crate::RegisterSpec for ChnlpriorityclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnlpriorityclr::W`](W) writer structure"]
impl crate::Writable for ChnlpriorityclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLPRIORITYCLR to value 0"]
impl crate::Resettable for ChnlpriorityclrSpec {}
