#[doc = "Register `SFR_PMUPDAR` reader"]
pub type R = crate::R<SfrPmupdarSpec>;
#[doc = "Register `SFR_PMUPDAR` writer"]
pub type W = crate::W<SfrPmupdarSpec>;
#[doc = "Field `sfr_pmupdar` reader - sfr_pmupdar performs action on write of value: 0x5a"]
pub type SfrPmupdarR = crate::FieldReader<u32>;
#[doc = "Field `sfr_pmupdar` writer - sfr_pmupdar performs action on write of value: 0x5a"]
pub type SfrPmupdarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_pmupdar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_pmupdar(&self) -> SfrPmupdarR {
        SfrPmupdarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_pmupdar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_pmupdar(&mut self) -> SfrPmupdarW<'_, SfrPmupdarSpec> {
        SfrPmupdarW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L391 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L391>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmupdar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmupdar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmupdarSpec;
impl crate::RegisterSpec for SfrPmupdarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmupdar::R`](R) reader structure"]
impl crate::Readable for SfrPmupdarSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmupdar::W`](W) writer structure"]
impl crate::Writable for SfrPmupdarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUPDAR to value 0"]
impl crate::Resettable for SfrPmupdarSpec {}
