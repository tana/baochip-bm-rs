#[doc = "Register `SFR_GPIOPU_CRGPU5` reader"]
pub type R = crate::R<SfrGpiopuCrgpu5Spec>;
#[doc = "Register `SFR_GPIOPU_CRGPU5` writer"]
pub type W = crate::W<SfrGpiopuCrgpu5Spec>;
#[doc = "Field `crgpu5` reader - crgpu read/write control register"]
pub type Crgpu5R = crate::FieldReader<u16>;
#[doc = "Field `crgpu5` writer - crgpu read/write control register"]
pub type Crgpu5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu5(&self) -> Crgpu5R {
        Crgpu5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu5(&mut self) -> Crgpu5W<'_, SfrGpiopuCrgpu5Spec> {
        Crgpu5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiopuCrgpu5Spec;
impl crate::RegisterSpec for SfrGpiopuCrgpu5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiopu_crgpu5::R`](R) reader structure"]
impl crate::Readable for SfrGpiopuCrgpu5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiopu_crgpu5::W`](W) writer structure"]
impl crate::Writable for SfrGpiopuCrgpu5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOPU_CRGPU5 to value 0"]
impl crate::Resettable for SfrGpiopuCrgpu5Spec {}
