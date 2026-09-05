#[doc = "Register `SFR_VDIP_TEST` reader"]
pub type R = crate::R<SfrVdipTestSpec>;
#[doc = "Register `SFR_VDIP_TEST` writer"]
pub type W = crate::W<SfrVdipTestSpec>;
#[doc = "Field `vdtst` reader - vdtst read/write control register"]
pub type VdtstR = crate::FieldReader;
#[doc = "Field `vdtst` writer - vdtst read/write control register"]
pub type VdtstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - vdtst read/write control register"]
    #[inline(always)]
    pub fn vdtst(&self) -> VdtstR {
        VdtstR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - vdtst read/write control register"]
    #[inline(always)]
    pub fn vdtst(&mut self) -> VdtstW<'_, SfrVdipTestSpec> {
        VdtstW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L75 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L75>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdip_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdip_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdipTestSpec;
impl crate::RegisterSpec for SfrVdipTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdip_test::R`](R) reader structure"]
impl crate::Readable for SfrVdipTestSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdip_test::W`](W) writer structure"]
impl crate::Writable for SfrVdipTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDIP_TEST to value 0"]
impl crate::Resettable for SfrVdipTestSpec {}
