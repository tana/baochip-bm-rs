#[doc = "Register `SFR_CACHE` reader"]
pub type R = crate::R<SfrCacheSpec>;
#[doc = "Register `SFR_CACHE` writer"]
pub type W = crate::W<SfrCacheSpec>;
#[doc = "Field `sfr_cache` reader - sfr_cache read/write control register"]
pub type SfrCacheR = crate::FieldReader;
#[doc = "Field `sfr_cache` writer - sfr_cache read/write control register"]
pub type SfrCacheW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - sfr_cache read/write control register"]
    #[inline(always)]
    pub fn sfr_cache(&self) -> SfrCacheR {
        SfrCacheR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - sfr_cache read/write control register"]
    #[inline(always)]
    pub fn sfr_cache(&mut self) -> SfrCacheW<'_, SfrCacheSpec> {
        SfrCacheW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L54 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L54>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cache::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cache::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCacheSpec;
impl crate::RegisterSpec for SfrCacheSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cache::R`](R) reader structure"]
impl crate::Readable for SfrCacheSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cache::W`](W) writer structure"]
impl crate::Writable for SfrCacheSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CACHE to value 0"]
impl crate::Resettable for SfrCacheSpec {}
