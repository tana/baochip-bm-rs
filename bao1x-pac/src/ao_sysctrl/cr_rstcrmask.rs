#[doc = "Register `CR_RSTCRMASK` reader"]
pub type R = crate::R<CrRstcrmaskSpec>;
#[doc = "Register `CR_RSTCRMASK` writer"]
pub type W = crate::W<CrRstcrmaskSpec>;
#[doc = "Field `cr_rstcrmask` reader - cr_rstcrmask read/write control register"]
pub type CrRstcrmaskR = crate::FieldReader;
#[doc = "Field `cr_rstcrmask` writer - cr_rstcrmask read/write control register"]
pub type CrRstcrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - cr_rstcrmask read/write control register"]
    #[inline(always)]
    pub fn cr_rstcrmask(&self) -> CrRstcrmaskR {
        CrRstcrmaskR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - cr_rstcrmask read/write control register"]
    #[inline(always)]
    pub fn cr_rstcrmask(&mut self) -> CrRstcrmaskW<'_, CrRstcrmaskSpec> {
        CrRstcrmaskW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L370 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L370>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_rstcrmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_rstcrmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRstcrmaskSpec;
impl crate::RegisterSpec for CrRstcrmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_rstcrmask::R`](R) reader structure"]
impl crate::Readable for CrRstcrmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_rstcrmask::W`](W) writer structure"]
impl crate::Writable for CrRstcrmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_RSTCRMASK to value 0"]
impl crate::Resettable for CrRstcrmaskSpec {}
