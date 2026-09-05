#[doc = "Register `SFR_GPIOOE_CRGOE4` reader"]
pub type R = crate::R<SfrGpiooeCrgoe4Spec>;
#[doc = "Register `SFR_GPIOOE_CRGOE4` writer"]
pub type W = crate::W<SfrGpiooeCrgoe4Spec>;
#[doc = "Field `crgoe4` reader - crgoe read/write control register"]
pub type Crgoe4R = crate::FieldReader<u16>;
#[doc = "Field `crgoe4` writer - crgoe read/write control register"]
pub type Crgoe4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe4(&self) -> Crgoe4R {
        Crgoe4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe4(&mut self) -> Crgoe4W<'_, SfrGpiooeCrgoe4Spec> {
        Crgoe4W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiooeCrgoe4Spec;
impl crate::RegisterSpec for SfrGpiooeCrgoe4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiooe_crgoe4::R`](R) reader structure"]
impl crate::Readable for SfrGpiooeCrgoe4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiooe_crgoe4::W`](W) writer structure"]
impl crate::Writable for SfrGpiooeCrgoe4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOOE_CRGOE4 to value 0"]
impl crate::Resettable for SfrGpiooeCrgoe4Spec {}
