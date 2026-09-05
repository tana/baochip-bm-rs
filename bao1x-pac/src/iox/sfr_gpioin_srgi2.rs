#[doc = "Register `SFR_GPIOIN_SRGI2` reader"]
pub type R = crate::R<SfrGpioinSrgi2Spec>;
#[doc = "Register `SFR_GPIOIN_SRGI2` writer"]
pub type W = crate::W<SfrGpioinSrgi2Spec>;
#[doc = "Field `srgi2` reader - srgi read only status register"]
pub type Srgi2R = crate::FieldReader<u16>;
#[doc = "Field `srgi2` writer - srgi read only status register"]
pub type Srgi2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi2(&self) -> Srgi2R {
        Srgi2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - srgi read only status register"]
    #[inline(always)]
    pub fn srgi2(&mut self) -> Srgi2W<'_, SfrGpioinSrgi2Spec> {
        Srgi2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L149 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L149>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gpioin_srgi2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gpioin_srgi2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGpioinSrgi2Spec;
impl crate::RegisterSpec for SfrGpioinSrgi2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gpioin_srgi2::R`](R) reader structure"]
impl crate::Readable for SfrGpioinSrgi2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gpioin_srgi2::W`](W) writer structure"]
impl crate::Writable for SfrGpioinSrgi2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GPIOIN_SRGI2 to value 0"]
impl crate::Resettable for SfrGpioinSrgi2Spec {}
