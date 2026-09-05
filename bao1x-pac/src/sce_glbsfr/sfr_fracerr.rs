#[doc = "Register `SFR_FRACERR` reader"]
pub type R = crate::R<SfrFracerrSpec>;
#[doc = "Register `SFR_FRACERR` writer"]
pub type W = crate::W<SfrFracerrSpec>;
#[doc = "Field `fr_acerr` reader - fr_acerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrAcerrR = crate::FieldReader;
#[doc = "Field `fr_acerr` writer - fr_acerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrAcerrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - fr_acerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_acerr(&self) -> FrAcerrR {
        FrAcerrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - fr_acerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_acerr(&mut self) -> FrAcerrW<'_, SfrFracerrSpec> {
        FrAcerrW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L86 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L86>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fracerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fracerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFracerrSpec;
impl crate::RegisterSpec for SfrFracerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_fracerr::R`](R) reader structure"]
impl crate::Readable for SfrFracerrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_fracerr::W`](W) writer structure"]
impl crate::Writable for SfrFracerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FRACERR to value 0"]
impl crate::Resettable for SfrFracerrSpec {}
