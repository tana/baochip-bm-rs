#[doc = "Register `SFR_PMUFR` reader"]
pub type R = crate::R<SfrPmufrSpec>;
#[doc = "Register `SFR_PMUFR` writer"]
pub type W = crate::W<SfrPmufrSpec>;
#[doc = "Field `sfr_pmufr` reader - sfr_pmufr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrPmufrR = crate::FieldReader;
#[doc = "Field `sfr_pmufr` writer - sfr_pmufr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrPmufrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_pmufr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_pmufr(&self) -> SfrPmufrR {
        SfrPmufrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_pmufr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_pmufr(&mut self) -> SfrPmufrW<'_, SfrPmufrSpec> {
        SfrPmufrW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L388 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L388>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmufr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmufr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmufrSpec;
impl crate::RegisterSpec for SfrPmufrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmufr::R`](R) reader structure"]
impl crate::Readable for SfrPmufrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmufr::W`](W) writer structure"]
impl crate::Writable for SfrPmufrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUFR to value 0"]
impl crate::Resettable for SfrPmufrSpec {}
