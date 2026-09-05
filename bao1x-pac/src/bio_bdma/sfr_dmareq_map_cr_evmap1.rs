#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP1` reader"]
pub type R = crate::R<SfrDmareqMapCrEvmap1Spec>;
#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP1` writer"]
pub type W = crate::W<SfrDmareqMapCrEvmap1Spec>;
#[doc = "Field `cr_evmap1` reader - cr_evmap read/write control register"]
pub type CrEvmap1R = crate::FieldReader<u32>;
#[doc = "Field `cr_evmap1` writer - cr_evmap read/write control register"]
pub type CrEvmap1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap1(&self) -> CrEvmap1R {
        CrEvmap1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap1(&mut self) -> CrEvmap1W<'_, SfrDmareqMapCrEvmap1Spec> {
        CrEvmap1W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDmareqMapCrEvmap1Spec;
impl crate::RegisterSpec for SfrDmareqMapCrEvmap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dmareq_map_cr_evmap1::R`](R) reader structure"]
impl crate::Readable for SfrDmareqMapCrEvmap1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dmareq_map_cr_evmap1::W`](W) writer structure"]
impl crate::Writable for SfrDmareqMapCrEvmap1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DMAREQ_MAP_CR_EVMAP1 to value 0"]
impl crate::Resettable for SfrDmareqMapCrEvmap1Spec {}
