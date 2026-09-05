#[doc = "Register `SFR_CGUFSVLD` reader"]
pub type R = crate::R<SfrCgufsvldSpec>;
#[doc = "Register `SFR_CGUFSVLD` writer"]
pub type W = crate::W<SfrCgufsvldSpec>;
#[doc = "Field `sfr_cgufsvld` reader - sfr_cgufsvld read only status register"]
pub type SfrCgufsvldR = crate::FieldReader;
#[doc = "Field `sfr_cgufsvld` writer - sfr_cgufsvld read only status register"]
pub type SfrCgufsvldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_cgufsvld read only status register"]
    #[inline(always)]
    pub fn sfr_cgufsvld(&self) -> SfrCgufsvldR {
        SfrCgufsvldR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_cgufsvld read only status register"]
    #[inline(always)]
    pub fn sfr_cgufsvld(&mut self) -> SfrCgufsvldW<'_, SfrCgufsvldSpec> {
        SfrCgufsvldW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L786 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L786>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufsvld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufsvld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufsvldSpec;
impl crate::RegisterSpec for SfrCgufsvldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufsvld::R`](R) reader structure"]
impl crate::Readable for SfrCgufsvldSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufsvld::W`](W) writer structure"]
impl crate::Writable for SfrCgufsvldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFSVLD to value 0"]
impl crate::Resettable for SfrCgufsvldSpec {}
