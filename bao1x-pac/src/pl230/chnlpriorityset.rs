#[doc = "Register `CHNLPRIORITYSET` reader"]
pub type R = crate::R<ChnlprioritysetSpec>;
#[doc = "Register `CHNLPRIORITYSET` writer"]
pub type W = crate::W<ChnlprioritysetSpec>;
#[doc = "Field `CHNL_PRIORITY_SET` reader - CHNL_PRIORITY_SET"]
pub type ChnlPrioritySetR = crate::FieldReader;
#[doc = "Field `CHNL_PRIORITY_SET` writer - CHNL_PRIORITY_SET"]
pub type ChnlPrioritySetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CHNL_PRIORITY_SET"]
    #[inline(always)]
    pub fn chnl_priority_set(&self) -> ChnlPrioritySetR {
        ChnlPrioritySetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CHNL_PRIORITY_SET"]
    #[inline(always)]
    pub fn chnl_priority_set(&mut self) -> ChnlPrioritySetW<'_, ChnlprioritysetSpec> {
        ChnlPrioritySetW::new(self, 0)
    }
}
#[doc = "DMA Channel Priority Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlpriorityset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlpriorityset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlprioritysetSpec;
impl crate::RegisterSpec for ChnlprioritysetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlpriorityset::R`](R) reader structure"]
impl crate::Readable for ChnlprioritysetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnlpriorityset::W`](W) writer structure"]
impl crate::Writable for ChnlprioritysetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLPRIORITYSET to value 0"]
impl crate::Resettable for ChnlprioritysetSpec {}
