#[doc = "Register `CHNLUSEBURSTSET` reader"]
pub type R = crate::R<ChnluseburstsetSpec>;
#[doc = "Register `CHNLUSEBURSTSET` writer"]
pub type W = crate::W<ChnluseburstsetSpec>;
#[doc = "Field `CHNL_USEBURST_SET` reader - CHNL_USEBURST_SET"]
pub type ChnlUseburstSetR = crate::FieldReader;
#[doc = "Field `CHNL_USEBURST_SET` writer - CHNL_USEBURST_SET"]
pub type ChnlUseburstSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CHNL_USEBURST_SET"]
    #[inline(always)]
    pub fn chnl_useburst_set(&self) -> ChnlUseburstSetR {
        ChnlUseburstSetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CHNL_USEBURST_SET"]
    #[inline(always)]
    pub fn chnl_useburst_set(&mut self) -> ChnlUseburstSetW<'_, ChnluseburstsetSpec> {
        ChnlUseburstSetW::new(self, 0)
    }
}
#[doc = "DMA Channel Useburst Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chnluseburstset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnluseburstset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnluseburstsetSpec;
impl crate::RegisterSpec for ChnluseburstsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnluseburstset::R`](R) reader structure"]
impl crate::Readable for ChnluseburstsetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnluseburstset::W`](W) writer structure"]
impl crate::Writable for ChnluseburstsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNLUSEBURSTSET to value 0"]
impl crate::Resettable for ChnluseburstsetSpec {}
