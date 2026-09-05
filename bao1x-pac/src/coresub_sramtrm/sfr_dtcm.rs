#[doc = "Register `SFR_DTCM` reader"]
pub type R = crate::R<SfrDtcmSpec>;
#[doc = "Register `SFR_DTCM` writer"]
pub type W = crate::W<SfrDtcmSpec>;
#[doc = "Field `sfr_dtcm` reader - sfr_dtcm read/write control register"]
pub type SfrDtcmR = crate::FieldReader;
#[doc = "Field `sfr_dtcm` writer - sfr_dtcm read/write control register"]
pub type SfrDtcmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_dtcm read/write control register"]
    #[inline(always)]
    pub fn sfr_dtcm(&self) -> SfrDtcmR {
        SfrDtcmR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_dtcm read/write control register"]
    #[inline(always)]
    pub fn sfr_dtcm(&mut self) -> SfrDtcmW<'_, SfrDtcmSpec> {
        SfrDtcmW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L56 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L56>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dtcm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dtcm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDtcmSpec;
impl crate::RegisterSpec for SfrDtcmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dtcm::R`](R) reader structure"]
impl crate::Readable for SfrDtcmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dtcm::W`](W) writer structure"]
impl crate::Writable for SfrDtcmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DTCM to value 0"]
impl crate::Resettable for SfrDtcmSpec {}
