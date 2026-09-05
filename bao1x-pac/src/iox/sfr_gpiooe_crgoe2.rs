#[doc = "Register `SFR_GPIOOE_CRGOE2` reader"]
pub type R = crate::R<SfrGpiooeCrgoe2Spec>;
#[doc = "Register `SFR_GPIOOE_CRGOE2` writer"]
pub type W = crate::W<SfrGpiooeCrgoe2Spec>;
#[doc = "Field `crgoe2` reader - crgoe read/write control register"]
pub type Crgoe2R = crate::FieldReader<u16>;
#[doc = "Field `crgoe2` writer - crgoe read/write control register"]
pub type Crgoe2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe2(&self) -> Crgoe2R {
        Crgoe2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe2(&mut self) -> Crgoe2W<'_, SfrGpiooeCrgoe2Spec> {
        Crgoe2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiooeCrgoe2Spec;
impl crate::RegisterSpec for SfrGpiooeCrgoe2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiooe_crgoe2::R`](R) reader structure"]
impl crate::Readable for SfrGpiooeCrgoe2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiooe_crgoe2::W`](W) writer structure"]
impl crate::Writable for SfrGpiooeCrgoe2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOOE_CRGOE2 to value 0"]
impl crate::Resettable for SfrGpiooeCrgoe2Spec {}
