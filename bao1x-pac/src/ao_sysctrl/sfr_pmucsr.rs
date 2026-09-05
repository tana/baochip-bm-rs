#[doc = "Register `SFR_PMUCSR` reader"]
pub type R = crate::R<SfrPmucsrSpec>;
#[doc = "Register `SFR_PMUCSR` writer"]
pub type W = crate::W<SfrPmucsrSpec>;
#[doc = "Field `pmucrreg` reader - pmucrreg read only status register"]
pub type PmucrregR = crate::FieldReader;
#[doc = "Field `pmucrreg` writer - pmucrreg read only status register"]
pub type PmucrregW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - pmucrreg read only status register"]
    #[inline(always)]
    pub fn pmucrreg(&self) -> PmucrregR {
        PmucrregR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - pmucrreg read only status register"]
    #[inline(always)]
    pub fn pmucrreg(&mut self) -> PmucrregW<'_, SfrPmucsrSpec> {
        PmucrregW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L376 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L376>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmucsrSpec;
impl crate::RegisterSpec for SfrPmucsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmucsr::R`](R) reader structure"]
impl crate::Readable for SfrPmucsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmucsr::W`](W) writer structure"]
impl crate::Writable for SfrPmucsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUCSR to value 0"]
impl crate::Resettable for SfrPmucsrSpec {}
