#[doc = "Register `SFR_GPIOIN_SRGI0` reader"]
pub type R = crate::R<SfrGpioinSrgi0Spec>;
#[doc = "Register `SFR_GPIOIN_SRGI0` writer"]
pub type W = crate::W<SfrGpioinSrgi0Spec>;
#[doc = "Field `srgi0` reader - srgi read only status register"]
pub type Srgi0R = crate::FieldReader<u16>;
#[doc = "Field `srgi0` writer - srgi read only status register"]
pub type Srgi0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi0(&self) -> Srgi0R {
        Srgi0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi0(&mut self) -> Srgi0W<'_, SfrGpioinSrgi0Spec> {
        Srgi0W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpioinSrgi0Spec;
impl crate::RegisterSpec for SfrGpioinSrgi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpioin_srgi0::R`](R) reader structure"]
impl crate::Readable for SfrGpioinSrgi0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpioin_srgi0::W`](W) writer structure"]
impl crate::Writable for SfrGpioinSrgi0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOIN_SRGI0 to value 0"]
impl crate::Resettable for SfrGpioinSrgi0Spec {}
