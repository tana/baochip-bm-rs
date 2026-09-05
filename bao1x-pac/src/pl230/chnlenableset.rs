#[doc = "Register `CHNLENABLESET` reader"]
pub type R = crate::R<ChnlenablesetSpec>;
#[doc = "Register `CHNLENABLESET` writer"]
pub type W = crate::W<ChnlenablesetSpec>;
#[doc = "Field `CHNL_ENABLE_SET` reader - CHNL_ENABLE_SET"]
pub type ChnlEnableSetR = crate::FieldReader;
#[doc = "Field `CHNL_ENABLE_SET` writer - CHNL_ENABLE_SET"]
pub type ChnlEnableSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CHNL_ENABLE_SET"]
    #[inline(always)]
    pub fn chnl_enable_set(&self) -> ChnlEnableSetR {
        ChnlEnableSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CHNL_ENABLE_SET"]
    #[inline(always)]
    pub fn chnl_enable_set(&mut self) -> ChnlEnableSetW<'_, ChnlenablesetSpec> {
        ChnlEnableSetW::new(self, 0)
    }
}
#[doc = "DMA Channel Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnlenableset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnlenableset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlenablesetSpec;
impl crate::RegisterSpec for ChnlenablesetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnlenableset::R`](R) reader structure"]
impl crate::Readable for ChnlenablesetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnlenableset::W`](W) writer structure"]
impl crate::Writable for ChnlenablesetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLENABLESET to value 0"]
impl crate::Resettable for ChnlenablesetSpec {}
