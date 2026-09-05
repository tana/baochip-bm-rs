#[doc = "Register `SFR_ITCM` reader"]
pub type R = crate::R<SfrItcmSpec>;
#[doc = "Register `SFR_ITCM` writer"]
pub type W = crate::W<SfrItcmSpec>;
#[doc = "Field `sfr_itcm` reader - sfr_itcm read/write control register"]
pub type SfrItcmR = crate::FieldReader;
#[doc = "Field `sfr_itcm` writer - sfr_itcm read/write control register"]
pub type SfrItcmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_itcm read/write control register"]
    #[inline(always)]
    pub fn sfr_itcm(&self) -> SfrItcmR {
        SfrItcmR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_itcm read/write control register"]
    #[inline(always)]
    pub fn sfr_itcm(&mut self) -> SfrItcmW<'_, SfrItcmSpec> {
        SfrItcmW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L55 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L55>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_itcm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_itcm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrItcmSpec;
impl crate::RegisterSpec for SfrItcmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_itcm::R`](R) reader structure"]
impl crate::Readable for SfrItcmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_itcm::W`](W) writer structure"]
impl crate::Writable for SfrItcmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ITCM to value 0"]
impl crate::Resettable for SfrItcmSpec {}
