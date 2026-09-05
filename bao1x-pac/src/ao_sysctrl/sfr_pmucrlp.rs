#[doc = "Register `SFR_PMUCRLP` reader"]
pub type R = crate::R<SfrPmucrlpSpec>;
#[doc = "Register `SFR_PMUCRLP` writer"]
pub type W = crate::W<SfrPmucrlpSpec>;
#[doc = "Field `sfrpmucrlp` reader - sfrpmucrlp read/write control register"]
pub type SfrpmucrlpR = crate::FieldReader;
#[doc = "Field `sfrpmucrlp` writer - sfrpmucrlp read/write control register"]
pub type SfrpmucrlpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfrpmucrlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmucrlp(&self) -> SfrpmucrlpR {
        SfrpmucrlpR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfrpmucrlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmucrlp(&mut self) -> SfrpmucrlpW<'_, SfrPmucrlpSpec> {
        SfrpmucrlpW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L373 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L373>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmucrlp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmucrlp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmucrlpSpec;
impl crate::RegisterSpec for SfrPmucrlpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmucrlp::R`](R) reader structure"]
impl crate::Readable for SfrPmucrlpSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmucrlp::W`](W) writer structure"]
impl crate::Writable for SfrPmucrlpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUCRLP to value 0"]
impl crate::Resettable for SfrPmucrlpSpec {}
