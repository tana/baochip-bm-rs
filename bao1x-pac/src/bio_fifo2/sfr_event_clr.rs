#[doc = "Register `SFR_EVENT_CLR` reader"]
pub type R = crate::R<SfrEventClrSpec>;
#[doc = "Register `SFR_EVENT_CLR` writer"]
pub type W = crate::W<SfrEventClrSpec>;
#[doc = "Field `sfr_event_clr` reader - sfr_event_clr read/write control register"]
pub type SfrEventClrR = crate::FieldReader<u32>;
#[doc = "Field `sfr_event_clr` writer - sfr_event_clr read/write control register"]
pub type SfrEventClrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - sfr_event_clr read/write control register"]
    #[inline(always)]
    pub fn sfr_event_clr(&self) -> SfrEventClrR {
        SfrEventClrR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - sfr_event_clr read/write control register"]
    #[inline(always)]
    pub fn sfr_event_clr(&mut self) -> SfrEventClrW<'_, SfrEventClrSpec> {
        SfrEventClrW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L505 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L505>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_event_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_event_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEventClrSpec;
impl crate::RegisterSpec for SfrEventClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_event_clr::R`](R) reader structure"]
impl crate::Readable for SfrEventClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_event_clr::W`](W) writer structure"]
impl crate::Writable for SfrEventClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_EVENT_CLR to value 0"]
impl crate::Resettable for SfrEventClrSpec {}
