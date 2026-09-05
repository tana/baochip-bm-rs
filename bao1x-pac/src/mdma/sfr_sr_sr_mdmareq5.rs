#[doc = "Register `SFR_SR_SR_MDMAREQ5` reader"]
pub type R = crate::R<SfrSrSrMdmareq5Spec>;
#[doc = "Register `SFR_SR_SR_MDMAREQ5` writer"]
pub type W = crate::W<SfrSrSrMdmareq5Spec>;
#[doc = "Field `sr_mdmareq5` reader - sr_mdmareq read only status register"]
pub type SrMdmareq5R = crate::FieldReader;
#[doc = "Field `sr_mdmareq5` writer - sr_mdmareq read only status register"]
pub type SrMdmareq5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq5(&self) -> SrMdmareq5R {
        SrMdmareq5R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq5(&mut self) -> SrMdmareq5W<'_, SfrSrSrMdmareq5Spec> {
        SrMdmareq5W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSrMdmareq5Spec;
impl crate::RegisterSpec for SfrSrSrMdmareq5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr_sr_mdmareq5::R`](R) reader structure"]
impl crate::Readable for SfrSrSrMdmareq5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr_sr_mdmareq5::W`](W) writer structure"]
impl crate::Writable for SfrSrSrMdmareq5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR_SR_MDMAREQ5 to value 0"]
impl crate::Resettable for SfrSrSrMdmareq5Spec {}
