#[doc = "Register `SFR_GPIOPU_CRGPU4` reader"]
pub type R = crate::R<SfrGpiopuCrgpu4Spec>;
#[doc = "Register `SFR_GPIOPU_CRGPU4` writer"]
pub type W = crate::W<SfrGpiopuCrgpu4Spec>;
#[doc = "Field `crgpu4` reader - crgpu read/write control register"]
pub type Crgpu4R = crate::FieldReader<u16>;
#[doc = "Field `crgpu4` writer - crgpu read/write control register"]
pub type Crgpu4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu4(&self) -> Crgpu4R {
        Crgpu4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgpu read/write control register"]
    #[inline(always)]
    pub fn crgpu4(&mut self) -> Crgpu4W<'_, SfrGpiopuCrgpu4Spec> {
        Crgpu4W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiopu_crgpu4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiopu_crgpu4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiopuCrgpu4Spec;
impl crate::RegisterSpec for SfrGpiopuCrgpu4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiopu_crgpu4::R`](R) reader structure"]
impl crate::Readable for SfrGpiopuCrgpu4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiopu_crgpu4::W`](W) writer structure"]
impl crate::Writable for SfrGpiopuCrgpu4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOPU_CRGPU4 to value 0"]
impl crate::Resettable for SfrGpiopuCrgpu4Spec {}
