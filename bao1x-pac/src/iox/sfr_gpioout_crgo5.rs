#[doc = "Register `SFR_GPIOOUT_CRGO5` reader"]
pub type R = crate::R<SfrGpiooutCrgo5Spec>;
#[doc = "Register `SFR_GPIOOUT_CRGO5` writer"]
pub type W = crate::W<SfrGpiooutCrgo5Spec>;
#[doc = "Field `crgo5` reader - crgo read/write control register"]
pub type Crgo5R = crate::FieldReader<u16>;
#[doc = "Field `crgo5` writer - crgo read/write control register"]
pub type Crgo5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crgo read/write control register"]
    #[inline(always)]
    pub fn crgo5(&self) -> Crgo5R {
        Crgo5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crgo read/write control register"]
    #[inline(always)]
    pub fn crgo5(&mut self) -> Crgo5W<'_, SfrGpiooutCrgo5Spec> {
        Crgo5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioout_crgo5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioout_crgo5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpiooutCrgo5Spec;
impl crate::RegisterSpec for SfrGpiooutCrgo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpioout_crgo5::R`](R) reader structure"]
impl crate::Readable for SfrGpiooutCrgo5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpioout_crgo5::W`](W) writer structure"]
impl crate::Writable for SfrGpiooutCrgo5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOOUT_CRGO5 to value 0"]
impl crate::Resettable for SfrGpiooutCrgo5Spec {}
