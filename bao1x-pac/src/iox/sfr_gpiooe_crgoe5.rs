#[doc = "Register `SFR_GPIOOE_CRGOE5` reader"]
pub type R = crate::R<SfrGpiooeCrgoe5Spec>;
#[doc = "Register `SFR_GPIOOE_CRGOE5` writer"]
pub type W = crate::W<SfrGpiooeCrgoe5Spec>;
#[doc = "Field `crgoe5` reader - crgoe read/write control register"]
pub type Crgoe5R = crate::FieldReader<u16>;
#[doc = "Field `crgoe5` writer - crgoe read/write control register"]
pub type Crgoe5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe5(&self) -> Crgoe5R {
        Crgoe5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgoe read/write control register"]
    #[inline(always)]
    pub fn crgoe5(&mut self) -> Crgoe5W<'_, SfrGpiooeCrgoe5Spec> {
        Crgoe5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpiooe_crgoe5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpiooe_crgoe5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiooeCrgoe5Spec;
impl crate::RegisterSpec for SfrGpiooeCrgoe5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpiooe_crgoe5::R`](R) reader structure"]
impl crate::Readable for SfrGpiooeCrgoe5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpiooe_crgoe5::W`](W) writer structure"]
impl crate::Writable for SfrGpiooeCrgoe5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOOE_CRGOE5 to value 0"]
impl crate::Resettable for SfrGpiooeCrgoe5Spec {}
