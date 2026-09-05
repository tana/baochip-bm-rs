#[doc = "Register `SFR_PMUTRM1CSR` reader"]
pub type R = crate::R<SfrPmutrm1csrSpec>;
#[doc = "Register `SFR_PMUTRM1CSR` writer"]
pub type W = crate::W<SfrPmutrm1csrSpec>;
#[doc = "Field `pmutrmreg` reader - pmutrmreg read only status register"]
pub type PmutrmregR = crate::FieldReader;
#[doc = "Field `pmutrmreg` writer - pmutrmreg read only status register"]
pub type PmutrmregW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - pmutrmreg read only status register"]
    #[inline(always)]
    pub fn pmutrmreg(&self) -> PmutrmregR {
        PmutrmregR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - pmutrmreg read only status register"]
    #[inline(always)]
    pub fn pmutrmreg(&mut self) -> PmutrmregW<'_, SfrPmutrm1csrSpec> {
        PmutrmregW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L384 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L384>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrm1csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrm1csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmutrm1csrSpec;
impl crate::RegisterSpec for SfrPmutrm1csrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmutrm1csr::R`](R) reader structure"]
impl crate::Readable for SfrPmutrm1csrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmutrm1csr::W`](W) writer structure"]
impl crate::Writable for SfrPmutrm1csrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUTRM1CSR to value 0"]
impl crate::Resettable for SfrPmutrm1csrSpec {}
