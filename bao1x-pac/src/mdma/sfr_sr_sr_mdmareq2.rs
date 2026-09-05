#[doc = "Register `SFR_SR_SR_MDMAREQ2` reader"]
pub type R = crate::R<SfrSrSrMdmareq2Spec>;
#[doc = "Register `SFR_SR_SR_MDMAREQ2` writer"]
pub type W = crate::W<SfrSrSrMdmareq2Spec>;
#[doc = "Field `sr_mdmareq2` reader - sr_mdmareq read only status register"]
pub type SrMdmareq2R = crate::FieldReader;
#[doc = "Field `sr_mdmareq2` writer - sr_mdmareq read only status register"]
pub type SrMdmareq2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq2(&self) -> SrMdmareq2R {
        SrMdmareq2R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq2(&mut self) -> SrMdmareq2W<'_, SfrSrSrMdmareq2Spec> {
        SrMdmareq2W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSrMdmareq2Spec;
impl crate::RegisterSpec for SfrSrSrMdmareq2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr_sr_mdmareq2::R`](R) reader structure"]
impl crate::Readable for SfrSrSrMdmareq2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr_sr_mdmareq2::W`](W) writer structure"]
impl crate::Writable for SfrSrSrMdmareq2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR_SR_MDMAREQ2 to value 0"]
impl crate::Resettable for SfrSrSrMdmareq2Spec {}
