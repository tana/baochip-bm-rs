#[doc = "Register `CHNLPRIALTSET` reader"]
pub type R = crate::R<ChnlprialtsetSpec>;
#[doc = "Register `CHNLPRIALTSET` writer"]
pub type W = crate::W<ChnlprialtsetSpec>;
#[doc = "Field `CHNL_PRI_ALT_SET` reader - CHNL_PRI_ALT_SET"]
pub type ChnlPriAltSetR = crate::FieldReader;
#[doc = "Field `CHNL_PRI_ALT_SET` writer - CHNL_PRI_ALT_SET"]
pub type ChnlPriAltSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CHNL_PRI_ALT_SET"]
    #[inline(always)]
    pub fn chnl_pri_alt_set(&self) -> ChnlPriAltSetR {
        ChnlPriAltSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CHNL_PRI_ALT_SET"]
    #[inline(always)]
    pub fn chnl_pri_alt_set(&mut self) -> ChnlPriAltSetW<'_, ChnlprialtsetSpec> {
        ChnlPriAltSetW::new(self, 0)
    }
}
#[doc = "DMA Channel Primary-Alternate Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlprialtset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlprialtset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlprialtsetSpec;
impl crate::RegisterSpec for ChnlprialtsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlprialtset::R`](R) reader structure"]
impl crate::Readable for ChnlprialtsetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnlprialtset::W`](W) writer structure"]
impl crate::Writable for ChnlprialtsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLPRIALTSET to value 0"]
impl crate::Resettable for ChnlprialtsetSpec {}
