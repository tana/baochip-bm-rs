#[doc = "Register `SFR_GPIOPU_CRGPU3` reader"]
pub type R = crate::R<SfrGpiopuCrgpu3Spec>;
#[doc = "Register `SFR_GPIOPU_CRGPU3` writer"]
pub type W = crate::W<SfrGpiopuCrgpu3Spec>;
#[doc = "Field `crgpu3` reader - crgpu read/write control register"]
pub type Crgpu3R = crate::FieldReader<u16>;
#[doc = "Field `crgpu3` writer - crgpu read/write control register"]
pub type Crgpu3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu3(&self) -> Crgpu3R {
        Crgpu3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu3(&mut self) -> Crgpu3W<'_, SfrGpiopuCrgpu3Spec> {
        Crgpu3W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiopuCrgpu3Spec;
impl crate::RegisterSpec for SfrGpiopuCrgpu3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiopu_crgpu3::R`](R) reader structure"]
impl crate::Readable for SfrGpiopuCrgpu3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiopu_crgpu3::W`](W) writer structure"]
impl crate::Writable for SfrGpiopuCrgpu3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOPU_CRGPU3 to value 0"]
impl crate::Resettable for SfrGpiopuCrgpu3Spec {}
