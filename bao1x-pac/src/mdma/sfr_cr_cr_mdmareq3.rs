#[doc = "Register `SFR_CR_CR_MDMAREQ3` reader"]
pub type R = crate::R<SfrCrCrMdmareq3Spec>;
#[doc = "Register `SFR_CR_CR_MDMAREQ3` writer"]
pub type W = crate::W<SfrCrCrMdmareq3Spec>;
#[doc = "Field `cr_mdmareq3` reader - cr_mdmareq read/write control register"]
pub type CrMdmareq3R = crate::FieldReader;
#[doc = "Field `cr_mdmareq3` writer - cr_mdmareq read/write control register"]
pub type CrMdmareq3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq3(&self) -> CrMdmareq3R {
        CrMdmareq3R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq3(&mut self) -> CrMdmareq3W<'_, SfrCrCrMdmareq3Spec> {
        CrMdmareq3W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrCrMdmareq3Spec;
impl crate::RegisterSpec for SfrCrCrMdmareq3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cr_cr_mdmareq3::R`](R) reader structure"]
impl crate::Readable for SfrCrCrMdmareq3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cr_cr_mdmareq3::W`](W) writer structure"]
impl crate::Writable for SfrCrCrMdmareq3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CR_CR_MDMAREQ3 to value 0"]
impl crate::Resettable for SfrCrCrMdmareq3Spec {}
