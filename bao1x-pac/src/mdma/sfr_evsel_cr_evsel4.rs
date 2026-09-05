#[doc = "Register `SFR_EVSEL_CR_EVSEL4` reader"]
pub type R = crate::R<SfrEvselCrEvsel4Spec>;
#[doc = "Register `SFR_EVSEL_CR_EVSEL4` writer"]
pub type W = crate::W<SfrEvselCrEvsel4Spec>;
#[doc = "Field `cr_evsel4` reader - cr_evsel read/write control register"]
pub type CrEvsel4R = crate::FieldReader;
#[doc = "Field `cr_evsel4` writer - cr_evsel read/write control register"]
pub type CrEvsel4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cr_evsel read/write control register"]
    #[inline(always)]
    pub fn cr_evsel4(&self) -> CrEvsel4R {
        CrEvsel4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cr_evsel read/write control register"]
    #[inline(always)]
    pub fn cr_evsel4(&mut self) -> CrEvsel4W<'_, SfrEvselCrEvsel4Spec> {
        CrEvsel4W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_evsel_cr_evsel4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_evsel_cr_evsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEvselCrEvsel4Spec;
impl crate::RegisterSpec for SfrEvselCrEvsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_evsel_cr_evsel4::R`](R) reader structure"]
impl crate::Readable for SfrEvselCrEvsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_evsel_cr_evsel4::W`](W) writer structure"]
impl crate::Writable for SfrEvselCrEvsel4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_EVSEL_CR_EVSEL4 to value 0"]
impl crate::Resettable for SfrEvselCrEvsel4Spec {}
