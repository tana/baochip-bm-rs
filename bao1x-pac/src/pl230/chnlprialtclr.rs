#[doc = "Register `CHNLPRIALTCLR` writer"]
pub type W = crate::W<ChnlprialtclrSpec>;
#[doc = "Field `CHNL_PRI_ALT_CLR` writer - CHNL_PRI_ALT_CLR"]
pub type ChnlPriAltClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_PRI_ALT_CLR"]
    #[inline(always)]
    pub fn chnl_pri_alt_clr(&mut self) -> ChnlPriAltClrW<'_, ChnlprialtclrSpec> {
        ChnlPriAltClrW::new(self, 0)
    }
}
#[doc = "DMA Channel Primary-Alternate Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlprialtclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlprialtclrSpec;
impl crate::RegisterSpec for ChnlprialtclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnlprialtclr::W`](W) writer structure"]
impl crate::Writable for ChnlprialtclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLPRIALTCLR to value 0"]
impl crate::Resettable for ChnlprialtclrSpec {}
