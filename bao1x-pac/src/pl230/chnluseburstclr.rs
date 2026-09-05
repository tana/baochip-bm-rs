#[doc = "Register `CHNLUSEBURSTCLR` writer"]
pub type W = crate::W<ChnluseburstclrSpec>;
#[doc = "Field `CHNL_USEBURST_CLR` writer - CHNL_USEBURST_CLR"]
pub type ChnlUseburstClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_USEBURST_CLR"]
    #[inline(always)]
    pub fn chnl_useburst_clr(&mut self) -> ChnlUseburstClrW<'_, ChnluseburstclrSpec> {
        ChnlUseburstClrW::new(self, 0)
    }
}
#[doc = "DMA Channel Useburst Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnluseburstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnluseburstclrSpec;
impl crate::RegisterSpec for ChnluseburstclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnluseburstclr::W`](W) writer structure"]
impl crate::Writable for ChnluseburstclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLUSEBURSTCLR to value 0"]
impl crate::Resettable for ChnluseburstclrSpec {}
