#[doc = "Register `SFR_VDIP_ENA` reader"]
pub type R = crate::R<SfrVdipEnaSpec>;
#[doc = "Register `SFR_VDIP_ENA` writer"]
pub type W = crate::W<SfrVdipEnaSpec>;
#[doc = "Field `vdena` reader - vdena read/write control register"]
pub type VdenaR = crate::FieldReader;
#[doc = "Field `vdena` writer - vdena read/write control register"]
pub type VdenaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - vdena read/write control register"]
    #[inline(always)]
    pub fn vdena(&self) -> VdenaR {
        VdenaR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - vdena read/write control register"]
    #[inline(always)]
    pub fn vdena(&mut self) -> VdenaW<'_, SfrVdipEnaSpec> {
        VdenaW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L74 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L74>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdip_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdip_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdipEnaSpec;
impl crate::RegisterSpec for SfrVdipEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdip_ena::R`](R) reader structure"]
impl crate::Readable for SfrVdipEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdip_ena::W`](W) writer structure"]
impl crate::Writable for SfrVdipEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDIP_ENA to value 0"]
impl crate::Resettable for SfrVdipEnaSpec {}
