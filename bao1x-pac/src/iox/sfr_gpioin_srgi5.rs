#[doc = "Register `SFR_GPIOIN_SRGI5` reader"]
pub type R = crate::R<SfrGpioinSrgi5Spec>;
#[doc = "Register `SFR_GPIOIN_SRGI5` writer"]
pub type W = crate::W<SfrGpioinSrgi5Spec>;
#[doc = "Field `srgi5` reader - srgi read only status register"]
pub type Srgi5R = crate::FieldReader<u16>;
#[doc = "Field `srgi5` writer - srgi read only status register"]
pub type Srgi5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi5(&self) -> Srgi5R {
        Srgi5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi5(&mut self) -> Srgi5W<'_, SfrGpioinSrgi5Spec> {
        Srgi5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpioinSrgi5Spec;
impl crate::RegisterSpec for SfrGpioinSrgi5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpioin_srgi5::R`](R) reader structure"]
impl crate::Readable for SfrGpioinSrgi5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpioin_srgi5::W`](W) writer structure"]
impl crate::Writable for SfrGpioinSrgi5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOIN_SRGI5 to value 0"]
impl crate::Resettable for SfrGpioinSrgi5Spec {}
