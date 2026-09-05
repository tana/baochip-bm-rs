#[doc = "Register `SFR_DMAREQ_STAT_SR_EVSTAT1` reader"]
pub type R = crate::R<SfrDmareqStatSrEvstat1Spec>;
#[doc = "Register `SFR_DMAREQ_STAT_SR_EVSTAT1` writer"]
pub type W = crate::W<SfrDmareqStatSrEvstat1Spec>;
#[doc = "Field `sr_evstat1` reader - sr_evstat read only status register"]
pub type SrEvstat1R = crate::FieldReader<u32>;
#[doc = "Field `sr_evstat1` writer - sr_evstat read only status register"]
pub type SrEvstat1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_evstat read only status register"]
    #[inline(always)]
    pub fn sr_evstat1(&self) -> SrEvstat1R {
        SrEvstat1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_evstat read only status register"]
    #[inline(always)]
    pub fn sr_evstat1(&mut self) -> SrEvstat1W<'_, SfrDmareqStatSrEvstat1Spec> {
        SrEvstat1W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L572 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L572>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dmareq_stat_sr_evstat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dmareq_stat_sr_evstat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDmareqStatSrEvstat1Spec;
impl crate::RegisterSpec for SfrDmareqStatSrEvstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dmareq_stat_sr_evstat1::R`](R) reader structure"]
impl crate::Readable for SfrDmareqStatSrEvstat1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dmareq_stat_sr_evstat1::W`](W) writer structure"]
impl crate::Writable for SfrDmareqStatSrEvstat1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DMAREQ_STAT_SR_EVSTAT1 to value 0"]
impl crate::Resettable for SfrDmareqStatSrEvstat1Spec {}
