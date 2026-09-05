#[doc = "Register `SFR_TMREVSEL` reader"]
pub type R = crate::R<SfrTmrevselSpec>;
#[doc = "Register `SFR_TMREVSEL` writer"]
pub type W = crate::W<SfrTmrevselSpec>;
#[doc = "Field `tmr_evsel` reader - tmr_evsel read/write control register"]
pub type TmrEvselR = crate::FieldReader<u16>;
#[doc = "Field `tmr_evsel` writer - tmr_evsel read/write control register"]
pub type TmrEvselW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - tmr_evsel read/write control register"]
    #[inline(always)]
    pub fn tmr_evsel(&self) -> TmrEvselR {
        TmrEvselR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - tmr_evsel read/write control register"]
    #[inline(always)]
    pub fn tmr_evsel(&mut self) -> TmrEvselW<'_, SfrTmrevselSpec> {
        TmrEvselW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L144 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L144>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tmrevsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tmrevsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTmrevselSpec;
impl crate::RegisterSpec for SfrTmrevselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_tmrevsel::R`](R) reader structure"]
impl crate::Readable for SfrTmrevselSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_tmrevsel::W`](W) writer structure"]
impl crate::Writable for SfrTmrevselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TMREVSEL to value 0"]
impl crate::Resettable for SfrTmrevselSpec {}
