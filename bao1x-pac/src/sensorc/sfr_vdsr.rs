#[doc = "Register `SFR_VDSR` reader"]
pub type R = crate::R<SfrVdsrSpec>;
#[doc = "Register `SFR_VDSR` writer"]
pub type W = crate::W<SfrVdsrSpec>;
#[doc = "Field `vdflag` reader - vdflag read only status register"]
pub type VdflagR = crate::FieldReader;
#[doc = "Field `vdflag` writer - vdflag read only status register"]
pub type VdflagW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - vdflag read only status register"]
    #[inline(always)]
    pub fn vdflag(&self) -> VdflagR {
        VdflagR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - vdflag read only status register"]
    #[inline(always)]
    pub fn vdflag(&mut self) -> VdflagW<'_, SfrVdsrSpec> {
        VdflagW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L65 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L65>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdsrSpec;
impl crate::RegisterSpec for SfrVdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdsr::R`](R) reader structure"]
impl crate::Readable for SfrVdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdsr::W`](W) writer structure"]
impl crate::Writable for SfrVdsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDSR to value 0"]
impl crate::Resettable for SfrVdsrSpec {}
