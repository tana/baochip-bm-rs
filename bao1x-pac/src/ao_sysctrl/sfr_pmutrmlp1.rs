#[doc = "Register `SFR_PMUTRMLP1` reader"]
pub type R = crate::R<SfrPmutrmlp1Spec>;
#[doc = "Register `SFR_PMUTRMLP1` writer"]
pub type W = crate::W<SfrPmutrmlp1Spec>;
#[doc = "Field `sfrpmutrmlp` reader - sfrpmutrmlp read/write control register"]
pub type SfrpmutrmlpR = crate::FieldReader;
#[doc = "Field `sfrpmutrmlp` writer - sfrpmutrmlp read/write control register"]
pub type SfrpmutrmlpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - sfrpmutrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmutrmlp(&self) -> SfrpmutrmlpR {
        SfrpmutrmlpR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sfrpmutrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmutrmlp(&mut self) -> SfrpmutrmlpW<'_, SfrPmutrmlp1Spec> {
        SfrpmutrmlpW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L382 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L382>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrmlp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrmlp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmutrmlp1Spec;
impl crate::RegisterSpec for SfrPmutrmlp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmutrmlp1::R`](R) reader structure"]
impl crate::Readable for SfrPmutrmlp1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmutrmlp1::W`](W) writer structure"]
impl crate::Writable for SfrPmutrmlp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUTRMLP1 to value 0"]
impl crate::Resettable for SfrPmutrmlp1Spec {}
