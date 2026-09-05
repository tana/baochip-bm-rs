#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP5` reader"]
pub type R = crate::R<SfrDmareqMapCrEvmap5Spec>;
#[doc = "Register `SFR_DMAREQ_MAP_CR_EVMAP5` writer"]
pub type W = crate::W<SfrDmareqMapCrEvmap5Spec>;
#[doc = "Field `cr_evmap5` reader - cr_evmap read/write control register"]
pub type CrEvmap5R = crate::FieldReader<u32>;
#[doc = "Field `cr_evmap5` writer - cr_evmap read/write control register"]
pub type CrEvmap5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap5(&self) -> CrEvmap5R {
        CrEvmap5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_evmap read/write control register"]
    #[inline(always)]
    pub fn cr_evmap5(&mut self) -> CrEvmap5W<'_, SfrDmareqMapCrEvmap5Spec> {
        CrEvmap5W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L571 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L571>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_map_cr_evmap5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_map_cr_evmap5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDmareqMapCrEvmap5Spec;
impl crate::RegisterSpec for SfrDmareqMapCrEvmap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dmareq_map_cr_evmap5::R`](R) reader structure"]
impl crate::Readable for SfrDmareqMapCrEvmap5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dmareq_map_cr_evmap5::W`](W) writer structure"]
impl crate::Writable for SfrDmareqMapCrEvmap5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DMAREQ_MAP_CR_EVMAP5 to value 0"]
impl crate::Resettable for SfrDmareqMapCrEvmap5Spec {}
