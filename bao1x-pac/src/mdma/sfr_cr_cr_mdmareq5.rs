#[doc = "Register `SFR_CR_CR_MDMAREQ5` reader"]
pub type R = crate::R<SfrCrCrMdmareq5Spec>;
#[doc = "Register `SFR_CR_CR_MDMAREQ5` writer"]
pub type W = crate::W<SfrCrCrMdmareq5Spec>;
#[doc = "Field `cr_mdmareq5` reader - cr_mdmareq read/write control register"]
pub type CrMdmareq5R = crate::FieldReader;
#[doc = "Field `cr_mdmareq5` writer - cr_mdmareq read/write control register"]
pub type CrMdmareq5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq5(&self) -> CrMdmareq5R {
        CrMdmareq5R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq5(&mut self) -> CrMdmareq5W<'_, SfrCrCrMdmareq5Spec> {
        CrMdmareq5W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrCrMdmareq5Spec;
impl crate::RegisterSpec for SfrCrCrMdmareq5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cr_cr_mdmareq5::R`](R) reader structure"]
impl crate::Readable for SfrCrCrMdmareq5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cr_cr_mdmareq5::W`](W) writer structure"]
impl crate::Writable for SfrCrCrMdmareq5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CR_CR_MDMAREQ5 to value 0"]
impl crate::Resettable for SfrCrCrMdmareq5Spec {}
