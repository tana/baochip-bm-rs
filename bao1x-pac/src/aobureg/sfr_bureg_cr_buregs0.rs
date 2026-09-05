#[doc = "Register `SFR_BUREG_CR_BUREGS0` reader"]
pub type R = crate::R<SfrBuregCrBuregs0Spec>;
#[doc = "Register `SFR_BUREG_CR_BUREGS0` writer"]
pub type W = crate::W<SfrBuregCrBuregs0Spec>;
#[doc = "Field `cr_buregs0` reader - cr_buregs read/write control register"]
pub type CrBuregs0R = crate::FieldReader<u32>;
#[doc = "Field `cr_buregs0` writer - cr_buregs read/write control register"]
pub type CrBuregs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_buregs read/write control register"]
    #[inline(always)]
    pub fn cr_buregs0(&self) -> CrBuregs0R {
        CrBuregs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_buregs read/write control register"]
    #[inline(always)]
    pub fn cr_buregs0(&mut self) -> CrBuregs0W<'_, SfrBuregCrBuregs0Spec> {
        CrBuregs0W::new(self, 0)
    }
}
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrBuregCrBuregs0Spec;
impl crate::RegisterSpec for SfrBuregCrBuregs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_bureg_cr_buregs0::R`](R) reader structure"]
impl crate::Readable for SfrBuregCrBuregs0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_bureg_cr_buregs0::W`](W) writer structure"]
impl crate::Writable for SfrBuregCrBuregs0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_BUREG_CR_BUREGS0 to value 0"]
impl crate::Resettable for SfrBuregCrBuregs0Spec {}
