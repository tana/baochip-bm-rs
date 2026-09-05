#[doc = "Register `SFR_CM7EVSEL_CM7EVSEL3` reader"]
pub type R = crate::R<SfrCm7evselCm7evsel3Spec>;
#[doc = "Register `SFR_CM7EVSEL_CM7EVSEL3` writer"]
pub type W = crate::W<SfrCm7evselCm7evsel3Spec>;
#[doc = "Field `cm7evsel3` reader - cm7evsel read/write control register"]
pub type Cm7evsel3R = crate::FieldReader;
#[doc = "Field `cm7evsel3` writer - cm7evsel read/write control register"]
pub type Cm7evsel3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cm7evsel read/write control register"]
    #[inline(always)]
    pub fn cm7evsel3(&self) -> Cm7evsel3R {
        Cm7evsel3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cm7evsel read/write control register"]
    #[inline(always)]
    pub fn cm7evsel3(&mut self) -> Cm7evsel3W<'_, SfrCm7evselCm7evsel3Spec> {
        Cm7evsel3W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L140 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L140>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evsel_cm7evsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evsel_cm7evsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7evselCm7evsel3Spec;
impl crate::RegisterSpec for SfrCm7evselCm7evsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7evsel_cm7evsel3::R`](R) reader structure"]
impl crate::Readable for SfrCm7evselCm7evsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7evsel_cm7evsel3::W`](W) writer structure"]
impl crate::Writable for SfrCm7evselCm7evsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7EVSEL_CM7EVSEL3 to value 0"]
impl crate::Resettable for SfrCm7evselCm7evsel3Spec {}
