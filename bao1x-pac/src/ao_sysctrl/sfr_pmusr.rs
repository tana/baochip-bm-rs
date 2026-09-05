#[doc = "Register `SFR_PMUSR` reader"]
pub type R = crate::R<SfrPmusrSpec>;
#[doc = "Register `SFR_PMUSR` writer"]
pub type W = crate::W<SfrPmusrSpec>;
#[doc = "Field `sfr_pmusr` reader - sfr_pmusr read only status register"]
pub type SfrPmusrR = crate::FieldReader;
#[doc = "Field `sfr_pmusr` writer - sfr_pmusr read only status register"]
pub type SfrPmusrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_pmusr read only status register"]
    #[inline(always)]
    pub fn sfr_pmusr(&self) -> SfrPmusrR {
        SfrPmusrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_pmusr read only status register"]
    #[inline(always)]
    pub fn sfr_pmusr(&mut self) -> SfrPmusrW<'_, SfrPmusrSpec> {
        SfrPmusrW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L387 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L387>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmusr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmusr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmusrSpec;
impl crate::RegisterSpec for SfrPmusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmusr::R`](R) reader structure"]
impl crate::Readable for SfrPmusrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmusr::W`](W) writer structure"]
impl crate::Writable for SfrPmusrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUSR to value 0"]
impl crate::Resettable for SfrPmusrSpec {}
