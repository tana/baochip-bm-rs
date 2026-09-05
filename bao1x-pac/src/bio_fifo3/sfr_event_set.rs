#[doc = "Register `SFR_EVENT_SET` reader"]
pub type R = crate::R<SfrEventSetSpec>;
#[doc = "Register `SFR_EVENT_SET` writer"]
pub type W = crate::W<SfrEventSetSpec>;
#[doc = "Field `sfr_event_set` reader - sfr_event_set read/write control register"]
pub type SfrEventSetR = crate::FieldReader<u32>;
#[doc = "Field `sfr_event_set` writer - sfr_event_set read/write control register"]
pub type SfrEventSetW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - sfr_event_set read/write control register"]
    #[inline(always)]
    pub fn sfr_event_set(&self) -> SfrEventSetR {
        SfrEventSetR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - sfr_event_set read/write control register"]
    #[inline(always)]
    pub fn sfr_event_set(&mut self) -> SfrEventSetW<'_, SfrEventSetSpec> {
        SfrEventSetW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L504 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L504>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEventSetSpec;
impl crate::RegisterSpec for SfrEventSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_event_set::R`](R) reader structure"]
impl crate::Readable for SfrEventSetSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_event_set::W`](W) writer structure"]
impl crate::Writable for SfrEventSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_EVENT_SET to value 0"]
impl crate::Resettable for SfrEventSetSpec {}
