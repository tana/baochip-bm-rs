#[doc = "Register `SFR_PMUTRMLP0` reader"]
pub type R = crate::R<SfrPmutrmlp0Spec>;
#[doc = "Register `SFR_PMUTRMLP0` writer"]
pub type W = crate::W<SfrPmutrmlp0Spec>;
#[doc = "Field `sfrpmutrmlp` reader - sfrpmutrmlp read/write control register"]
pub type SfrpmutrmlpR = crate::FieldReader<u32>;
#[doc = "Field `sfrpmutrmlp` writer - sfrpmutrmlp read/write control register"]
pub type SfrpmutrmlpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfrpmutrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmutrmlp(&self) -> SfrpmutrmlpR {
        SfrpmutrmlpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfrpmutrmlp read/write control register"]
    #[inline(always)]
    pub fn sfrpmutrmlp(&mut self) -> SfrpmutrmlpW<'_, SfrPmutrmlp0Spec> {
        SfrpmutrmlpW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L381 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L381>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrmlp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrmlp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmutrmlp0Spec;
impl crate::RegisterSpec for SfrPmutrmlp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmutrmlp0::R`](R) reader structure"]
impl crate::Readable for SfrPmutrmlp0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmutrmlp0::W`](W) writer structure"]
impl crate::Writable for SfrPmutrmlp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUTRMLP0 to value 0"]
impl crate::Resettable for SfrPmutrmlp0Spec {}
