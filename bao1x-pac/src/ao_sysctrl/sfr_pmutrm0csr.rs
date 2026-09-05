#[doc = "Register `SFR_PMUTRM0CSR` reader"]
pub type R = crate::R<SfrPmutrm0csrSpec>;
#[doc = "Register `SFR_PMUTRM0CSR` writer"]
pub type W = crate::W<SfrPmutrm0csrSpec>;
#[doc = "Field `pmutrmreg` reader - pmutrmreg read only status register"]
pub type PmutrmregR = crate::FieldReader<u32>;
#[doc = "Field `pmutrmreg` writer - pmutrmreg read only status register"]
pub type PmutrmregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - pmutrmreg read only status register"]
    #[inline(always)]
    pub fn pmutrmreg(&self) -> PmutrmregR {
        PmutrmregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - pmutrmreg read only status register"]
    #[inline(always)]
    pub fn pmutrmreg(&mut self) -> PmutrmregW<'_, SfrPmutrm0csrSpec> {
        PmutrmregW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L383 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L383>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_pmutrm0csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_pmutrm0csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPmutrm0csrSpec;
impl crate::RegisterSpec for SfrPmutrm0csrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_pmutrm0csr::R`](R) reader structure"]
impl crate::Readable for SfrPmutrm0csrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_pmutrm0csr::W`](W) writer structure"]
impl crate::Writable for SfrPmutrm0csrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PMUTRM0CSR to value 0"]
impl crate::Resettable for SfrPmutrm0csrSpec {}
