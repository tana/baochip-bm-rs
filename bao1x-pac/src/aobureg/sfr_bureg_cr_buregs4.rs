#[doc = "Register `SFR_BUREG_CR_BUREGS4` reader"]
pub type R = crate::R<SfrBuregCrBuregs4Spec>;
#[doc = "Register `SFR_BUREG_CR_BUREGS4` writer"]
pub type W = crate::W<SfrBuregCrBuregs4Spec>;
#[doc = "Field `cr_buregs4` reader - cr_buregs read/write control register"]
pub type CrBuregs4R = crate::FieldReader<u32>;
#[doc = "Field `cr_buregs4` writer - cr_buregs read/write control register"]
pub type CrBuregs4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_buregs read/write control register"]
    #[inline(always)]
    pub fn cr_buregs4(&self) -> CrBuregs4R {
        CrBuregs4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_buregs read/write control register"]
    #[inline(always)]
    pub fn cr_buregs4(&mut self) -> CrBuregs4W<'_, SfrBuregCrBuregs4Spec> {
        CrBuregs4W::new(self, 0)
    }
}
#[doc = "See `aobureg.sv#L33 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /ao/rtl/aobureg.sv#L33>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_bureg_cr_buregs4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_bureg_cr_buregs4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrBuregCrBuregs4Spec;
impl crate::RegisterSpec for SfrBuregCrBuregs4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_bureg_cr_buregs4::R`](R) reader structure"]
impl crate::Readable for SfrBuregCrBuregs4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_bureg_cr_buregs4::W`](W) writer structure"]
impl crate::Writable for SfrBuregCrBuregs4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_BUREG_CR_BUREGS4 to value 0"]
impl crate::Resettable for SfrBuregCrBuregs4Spec {}
