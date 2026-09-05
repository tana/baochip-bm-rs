#[doc = "Register `SFR_CR_CR_MDMAREQ1` reader"]
pub type R = crate::R<SfrCrCrMdmareq1Spec>;
#[doc = "Register `SFR_CR_CR_MDMAREQ1` writer"]
pub type W = crate::W<SfrCrCrMdmareq1Spec>;
#[doc = "Field `cr_mdmareq1` reader - cr_mdmareq read/write control register"]
pub type CrMdmareq1R = crate::FieldReader;
#[doc = "Field `cr_mdmareq1` writer - cr_mdmareq read/write control register"]
pub type CrMdmareq1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq1(&self) -> CrMdmareq1R {
        CrMdmareq1R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq1(&mut self) -> CrMdmareq1W<'_, SfrCrCrMdmareq1Spec> {
        CrMdmareq1W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrCrMdmareq1Spec;
impl crate::RegisterSpec for SfrCrCrMdmareq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cr_cr_mdmareq1::R`](R) reader structure"]
impl crate::Readable for SfrCrCrMdmareq1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cr_cr_mdmareq1::W`](W) writer structure"]
impl crate::Writable for SfrCrCrMdmareq1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CR_CR_MDMAREQ1 to value 0"]
impl crate::Resettable for SfrCrCrMdmareq1Spec {}
