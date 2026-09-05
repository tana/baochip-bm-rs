#[doc = "Register `SFR_TICKCNT` reader"]
pub type R = crate::R<SfrTickcntSpec>;
#[doc = "Register `SFR_TICKCNT` writer"]
pub type W = crate::W<SfrTickcntSpec>;
#[doc = "Field `sfr_tickcnt` reader - sfr_tickcnt read only status register"]
pub type SfrTickcntR = crate::FieldReader<u32>;
#[doc = "Field `sfr_tickcnt` writer - sfr_tickcnt read only status register"]
pub type SfrTickcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_tickcnt read only status register"]
    #[inline(always)]
    pub fn sfr_tickcnt(&self) -> SfrTickcntR {
        SfrTickcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_tickcnt read only status register"]
    #[inline(always)]
    pub fn sfr_tickcnt(&mut self) -> SfrTickcntW<'_, SfrTickcntSpec> {
        SfrTickcntW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTickcntSpec;
impl crate::RegisterSpec for SfrTickcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_tickcnt::R`](R) reader structure"]
impl crate::Readable for SfrTickcntSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_tickcnt::W`](W) writer structure"]
impl crate::Writable for SfrTickcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TICKCNT to value 0"]
impl crate::Resettable for SfrTickcntSpec {}
