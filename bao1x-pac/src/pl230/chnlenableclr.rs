#[doc = "Register `CHNLENABLECLR` writer"]
pub type W = crate::W<ChnlenableclrSpec>;
#[doc = "Field `CHNL_ENABLE_CLR` writer - CHNL_ENABLE_CLR"]
pub type ChnlEnableClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_ENABLE_CLR"]
    #[inline(always)]
    pub fn chnl_enable_clr(&mut self) -> ChnlEnableClrW<'_, ChnlenableclrSpec> {
        ChnlEnableClrW::new(self, 0)
    }
}
#[doc = "DMA Channel Enable Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlenableclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlenableclrSpec;
impl crate::RegisterSpec for ChnlenableclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnlenableclr::W`](W) writer structure"]
impl crate::Writable for ChnlenableclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLENABLECLR to value 0"]
impl crate::Resettable for ChnlenableclrSpec {}
