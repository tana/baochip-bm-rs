#[doc = "Register `SFR_EVENT_STATUS` reader"]
pub type R = crate::R<SfrEventStatusSpec>;
#[doc = "Register `SFR_EVENT_STATUS` writer"]
pub type W = crate::W<SfrEventStatusSpec>;
#[doc = "Field `sfr_event_status` reader - sfr_event_status read only status register"]
pub type SfrEventStatusR = crate::FieldReader<u32>;
#[doc = "Field `sfr_event_status` writer - sfr_event_status read only status register"]
pub type SfrEventStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_event_status read only status register"]
    #[inline(always)]
    pub fn sfr_event_status(&self) -> SfrEventStatusR {
        SfrEventStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_event_status read only status register"]
    #[inline(always)]
    pub fn sfr_event_status(&mut self) -> SfrEventStatusW<'_, SfrEventStatusSpec> {
        SfrEventStatusW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L506 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L506>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEventStatusSpec;
impl crate::RegisterSpec for SfrEventStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_event_status::R`](R) reader structure"]
impl crate::Readable for SfrEventStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_event_status::W`](W) writer structure"]
impl crate::Writable for SfrEventStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_EVENT_STATUS to value 0"]
impl crate::Resettable for SfrEventStatusSpec {}
