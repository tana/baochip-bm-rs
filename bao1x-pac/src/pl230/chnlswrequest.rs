#[doc = "Register `CHNLSWREQUEST` writer"]
pub type W = crate::W<ChnlswrequestSpec>;
#[doc = "Field `CHNL_SW_REQUEST` writer - CHNL_SW_REQUEST"]
pub type ChnlSwRequestW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - CHNL_SW_REQUEST"]
    #[inline(always)]
    pub fn chnl_sw_request(&mut self) -> ChnlSwRequestW<'_, ChnlswrequestSpec> {
        ChnlSwRequestW::new(self, 0)
    }
}
#[doc = "DMA Channel Software Request Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlswrequest::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlswrequestSpec;
impl crate::RegisterSpec for ChnlswrequestSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnlswrequest::W`](W) writer structure"]
impl crate::Writable for ChnlswrequestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLSWREQUEST to value 0"]
impl crate::Resettable for ChnlswrequestSpec {}
