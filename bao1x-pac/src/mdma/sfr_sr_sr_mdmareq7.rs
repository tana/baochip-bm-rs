#[doc = "Register `SFR_SR_SR_MDMAREQ7` reader"]
pub type R = crate::R<SfrSrSrMdmareq7Spec>;
#[doc = "Register `SFR_SR_SR_MDMAREQ7` writer"]
pub type W = crate::W<SfrSrSrMdmareq7Spec>;
#[doc = "Field `sr_mdmareq7` reader - sr_mdmareq read only status register"]
pub type SrMdmareq7R = crate::FieldReader;
#[doc = "Field `sr_mdmareq7` writer - sr_mdmareq read only status register"]
pub type SrMdmareq7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq7(&self) -> SrMdmareq7R {
        SrMdmareq7R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq7(&mut self) -> SrMdmareq7W<'_, SfrSrSrMdmareq7Spec> {
        SrMdmareq7W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSrMdmareq7Spec;
impl crate::RegisterSpec for SfrSrSrMdmareq7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr_sr_mdmareq7::R`](R) reader structure"]
impl crate::Readable for SfrSrSrMdmareq7Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr_sr_mdmareq7::W`](W) writer structure"]
impl crate::Writable for SfrSrSrMdmareq7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR_SR_MDMAREQ7 to value 0"]
impl crate::Resettable for SfrSrSrMdmareq7Spec {}
