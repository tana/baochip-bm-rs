#[doc = "Register `SFR_CRSRC` reader"]
pub type R = crate::R<SfrCrsrcSpec>;
#[doc = "Register `SFR_CRSRC` writer"]
pub type W = crate::W<SfrCrsrcSpec>;
#[doc = "Field `sfr_crsrc` reader - sfr_crsrc read/write control register"]
pub type SfrCrsrcR = crate::FieldReader<u16>;
#[doc = "Field `sfr_crsrc` writer - sfr_crsrc read/write control register"]
pub type SfrCrsrcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - sfr_crsrc read/write control register"]
    #[inline(always)]
    pub fn sfr_crsrc(&self) -> SfrCrsrcR {
        SfrCrsrcR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - sfr_crsrc read/write control register"]
    #[inline(always)]
    pub fn sfr_crsrc(&mut self) -> SfrCrsrcW<'_, SfrCrsrcSpec> {
        SfrCrsrcW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L105 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L105>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crsrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crsrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrsrcSpec;
impl crate::RegisterSpec for SfrCrsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_crsrc::R`](R) reader structure"]
impl crate::Readable for SfrCrsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_crsrc::W`](W) writer structure"]
impl crate::Writable for SfrCrsrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CRSRC to value 0"]
impl crate::Resettable for SfrCrsrcSpec {}
