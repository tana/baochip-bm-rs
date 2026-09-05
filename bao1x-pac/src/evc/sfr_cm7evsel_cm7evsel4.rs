#[doc = "Register `SFR_CM7EVSEL_CM7EVSEL4` reader"]
pub type R = crate::R<SfrCm7evselCm7evsel4Spec>;
#[doc = "Register `SFR_CM7EVSEL_CM7EVSEL4` writer"]
pub type W = crate::W<SfrCm7evselCm7evsel4Spec>;
#[doc = "Field `cm7evsel4` reader - cm7evsel read/write control register"]
pub type Cm7evsel4R = crate::FieldReader;
#[doc = "Field `cm7evsel4` writer - cm7evsel read/write control register"]
pub type Cm7evsel4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cm7evsel read/write control register"]
    #[inline(always)]
    pub fn cm7evsel4(&self) -> Cm7evsel4R {
        Cm7evsel4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cm7evsel read/write control register"]
    #[inline(always)]
    pub fn cm7evsel4(&mut self) -> Cm7evsel4W<'_, SfrCm7evselCm7evsel4Spec> {
        Cm7evsel4W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7evselCm7evsel4Spec;
impl crate::RegisterSpec for SfrCm7evselCm7evsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7evsel_cm7evsel4::R`](R) reader structure"]
impl crate::Readable for SfrCm7evselCm7evsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7evsel_cm7evsel4::W`](W) writer structure"]
impl crate::Writable for SfrCm7evselCm7evsel4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7EVSEL_CM7EVSEL4 to value 0"]
impl crate::Resettable for SfrCm7evselCm7evsel4Spec {}
