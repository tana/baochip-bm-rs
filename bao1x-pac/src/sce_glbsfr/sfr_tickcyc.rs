#[doc = "Register `SFR_TICKCYC` reader"]
pub type R = crate::R<SfrTickcycSpec>;
#[doc = "Register `SFR_TICKCYC` writer"]
pub type W = crate::W<SfrTickcycSpec>;
#[doc = "Field `sfr_tickcyc` reader - sfr_tickcyc read/write control register"]
pub type SfrTickcycR = crate::FieldReader;
#[doc = "Field `sfr_tickcyc` writer - sfr_tickcyc read/write control register"]
pub type SfrTickcycW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_tickcyc read/write control register"]
    #[inline(always)]
    pub fn sfr_tickcyc(&self) -> SfrTickcycR {
        SfrTickcycR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_tickcyc read/write control register"]
    #[inline(always)]
    pub fn sfr_tickcyc(&mut self) -> SfrTickcycW<'_, SfrTickcycSpec> {
        SfrTickcycW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L116>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tickcyc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tickcyc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTickcycSpec;
impl crate::RegisterSpec for SfrTickcycSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_tickcyc::R`](R) reader structure"]
impl crate::Readable for SfrTickcycSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_tickcyc::W`](W) writer structure"]
impl crate::Writable for SfrTickcycSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TICKCYC to value 0"]
impl crate::Resettable for SfrTickcycSpec {}
