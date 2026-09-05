#[doc = "Register `SFR_CR_CR_MDMAREQ0` reader"]
pub type R = crate::R<SfrCrCrMdmareq0Spec>;
#[doc = "Register `SFR_CR_CR_MDMAREQ0` writer"]
pub type W = crate::W<SfrCrCrMdmareq0Spec>;
#[doc = "Field `cr_mdmareq0` reader - cr_mdmareq read/write control register"]
pub type CrMdmareq0R = crate::FieldReader;
#[doc = "Field `cr_mdmareq0` writer - cr_mdmareq read/write control register"]
pub type CrMdmareq0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq0(&self) -> CrMdmareq0R {
        CrMdmareq0R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - cr_mdmareq read/write control register"]
    #[inline(always)]
    pub fn cr_mdmareq0(&mut self) -> CrMdmareq0W<'_, SfrCrCrMdmareq0Spec> {
        CrMdmareq0W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L103 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L103>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cr_cr_mdmareq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cr_cr_mdmareq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrCrMdmareq0Spec;
impl crate::RegisterSpec for SfrCrCrMdmareq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cr_cr_mdmareq0::R`](R) reader structure"]
impl crate::Readable for SfrCrCrMdmareq0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cr_cr_mdmareq0::W`](W) writer structure"]
impl crate::Writable for SfrCrCrMdmareq0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CR_CR_MDMAREQ0 to value 0"]
impl crate::Resettable for SfrCrCrMdmareq0Spec {}
