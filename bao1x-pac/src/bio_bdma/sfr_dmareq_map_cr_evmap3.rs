#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP3` reader"]
pub type R = crate::R<SfrDmareqMapCrEvmap3Spec>;
#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP3` writer"]
pub type W = crate::W<SfrDmareqMapCrEvmap3Spec>;
#[doc = "Field `cr_evmap3` reader - cr_evmap read/write control register"]
pub type CrEvmap3R = crate::FieldReader<u32>;
#[doc = "Field `cr_evmap3` writer - cr_evmap read/write control register"]
pub type CrEvmap3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap3(&self) -> CrEvmap3R {
        CrEvmap3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap3(&mut self) -> CrEvmap3W<'_, SfrDmareqMapCrEvmap3Spec> {
        CrEvmap3W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDmareqMapCrEvmap3Spec;
impl crate::RegisterSpec for SfrDmareqMapCrEvmap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dmareq_map_cr_evmap3::R`](R) reader structure"]
impl crate::Readable for SfrDmareqMapCrEvmap3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dmareq_map_cr_evmap3::W`](W) writer structure"]
impl crate::Writable for SfrDmareqMapCrEvmap3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DMAREQ_MAP_CR_EVMAP3 to value 0"]
impl crate::Resettable for SfrDmareqMapCrEvmap3Spec {}
