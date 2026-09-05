#[doc = "Register `SFR_FRERR` reader"]
pub type R = crate::R<SfrFrerrSpec>;
#[doc = "Register `SFR_FRERR` writer"]
pub type W = crate::W<SfrFrerrSpec>;
#[doc = "Field `fr_err` reader - fr_err flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrErrR = crate::FieldReader<u16>;
#[doc = "Field `fr_err` writer - fr_err flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrErrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - fr_err flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_err(&self) -> FrErrR {
        FrErrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - fr_err flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_err(&mut self) -> FrErrW<'_, SfrFrerrSpec> {
        FrErrW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L81 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L81>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_frerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_frerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFrerrSpec;
impl crate::RegisterSpec for SfrFrerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_frerr::R`](R) reader structure"]
impl crate::Readable for SfrFrerrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_frerr::W`](W) writer structure"]
impl crate::Writable for SfrFrerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FRERR to value 0"]
impl crate::Resettable for SfrFrerrSpec {}
