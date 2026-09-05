#[doc = "Register `SFR_SR_SR_MDMAREQ4` reader"]
pub type R = crate::R<SfrSrSrMdmareq4Spec>;
#[doc = "Register `SFR_SR_SR_MDMAREQ4` writer"]
pub type W = crate::W<SfrSrSrMdmareq4Spec>;
#[doc = "Field `sr_mdmareq4` reader - sr_mdmareq read only status register"]
pub type SrMdmareq4R = crate::FieldReader;
#[doc = "Field `sr_mdmareq4` writer - sr_mdmareq read only status register"]
pub type SrMdmareq4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq4(&self) -> SrMdmareq4R {
        SrMdmareq4R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sr_mdmareq read only status register"]
    #[inline(always)]
    pub fn sr_mdmareq4(&mut self) -> SrMdmareq4W<'_, SfrSrSrMdmareq4Spec> {
        SrMdmareq4W::new(self, 0)
    }
}
#[doc = "See `mdma.sv#L104 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/mdma.sv#L104>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr_sr_mdmareq4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr_sr_mdmareq4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSrMdmareq4Spec;
impl crate::RegisterSpec for SfrSrSrMdmareq4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr_sr_mdmareq4::R`](R) reader structure"]
impl crate::Readable for SfrSrSrMdmareq4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr_sr_mdmareq4::W`](W) writer structure"]
impl crate::Writable for SfrSrSrMdmareq4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR_SR_MDMAREQ4 to value 0"]
impl crate::Resettable for SfrSrSrMdmareq4Spec {}
