#[doc = "Register `SFR_GCMASK_CR_GCMASK0` reader"]
pub type R = crate::R<SfrGcmaskCrGcmask0Spec>;
#[doc = "Register `SFR_GCMASK_CR_GCMASK0` writer"]
pub type W = crate::W<SfrGcmaskCrGcmask0Spec>;
#[doc = "Field `cr_gcmask0` reader - cr_gcmask read/write control register"]
pub type CrGcmask0R = crate::FieldReader<u32>;
#[doc = "Field `cr_gcmask0` writer - cr_gcmask read/write control register"]
pub type CrGcmask0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_gcmask read/write control register"]
    #[inline(always)]
    pub fn cr_gcmask0(&self) -> CrGcmask0R {
        CrGcmask0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_gcmask read/write control register"]
    #[inline(always)]
    pub fn cr_gcmask0(&mut self) -> CrGcmask0W<'_, SfrGcmaskCrGcmask0Spec> {
        CrGcmask0W::new(self, 0)
    }
}
#[doc = "See `gluechain.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L44>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcmask_cr_gcmask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcmask_cr_gcmask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGcmaskCrGcmask0Spec;
impl crate::RegisterSpec for SfrGcmaskCrGcmask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gcmask_cr_gcmask0::R`](R) reader structure"]
impl crate::Readable for SfrGcmaskCrGcmask0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gcmask_cr_gcmask0::W`](W) writer structure"]
impl crate::Writable for SfrGcmaskCrGcmask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GCMASK_CR_GCMASK0 to value 0"]
impl crate::Resettable for SfrGcmaskCrGcmask0Spec {}
