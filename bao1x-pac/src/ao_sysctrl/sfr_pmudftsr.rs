#[doc = "Register `SFR_PMUDFTSR` reader"]
pub type R = crate::R<SfrPmudftsrSpec>;
#[doc = "Register `SFR_PMUDFTSR` writer"]
pub type W = crate::W<SfrPmudftsrSpec>;
#[doc = "Field `pmudftreg` reader - pmudftreg read only status register"]
pub type PmudftregR = crate::FieldReader;
#[doc = "Field `pmudftreg` writer - pmudftreg read only status register"]
pub type PmudftregW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - pmudftreg read only status register"]
    #[inline(always)]
    pub fn pmudftreg(&self) -> PmudftregR {
        PmudftregR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - pmudftreg read only status register"]
    #[inline(always)]
    pub fn pmudftreg(&mut self) -> PmudftregW<'_, SfrPmudftsrSpec> {
        PmudftregW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L377 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L377>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmudftsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmudftsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmudftsrSpec;
impl crate::RegisterSpec for SfrPmudftsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmudftsr::R`](R) reader structure"]
impl crate::Readable for SfrPmudftsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmudftsr::W`](W) writer structure"]
impl crate::Writable for SfrPmudftsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUDFTSR to value 0"]
impl crate::Resettable for SfrPmudftsrSpec {}
