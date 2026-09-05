#[doc = "Register `SFR_VDFR` reader"]
pub type R = crate::R<SfrVdfrSpec>;
#[doc = "Register `SFR_VDFR` writer"]
pub type W = crate::W<SfrVdfrSpec>;
#[doc = "Field `vdflag` reader - vdflag flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type VdflagR = crate::FieldReader;
#[doc = "Field `vdflag` writer - vdflag flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type VdflagW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - vdflag flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn vdflag(&self) -> VdflagR {
        VdflagR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - vdflag flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn vdflag(&mut self) -> VdflagW<'_, SfrVdfrSpec> {
        VdflagW::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L66 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L66>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdfrSpec;
impl crate::RegisterSpec for SfrVdfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdfr::R`](R) reader structure"]
impl crate::Readable for SfrVdfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdfr::W`](W) writer structure"]
impl crate::Writable for SfrVdfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDFR to value 0"]
impl crate::Resettable for SfrVdfrSpec {}
