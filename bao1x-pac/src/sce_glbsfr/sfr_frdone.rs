#[doc = "Register `SFR_FRDONE` reader"]
pub type R = crate::R<SfrFrdoneSpec>;
#[doc = "Register `SFR_FRDONE` writer"]
pub type W = crate::W<SfrFrdoneSpec>;
#[doc = "Field `fr_done` reader - fr_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrDoneR = crate::FieldReader<u16>;
#[doc = "Field `fr_done` writer - fr_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrDoneW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - fr_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_done(&self) -> FrDoneR {
        FrDoneR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - fr_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn fr_done(&mut self) -> FrDoneW<'_, SfrFrdoneSpec> {
        FrDoneW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L80 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L80>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_frdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_frdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFrdoneSpec;
impl crate::RegisterSpec for SfrFrdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_frdone::R`](R) reader structure"]
impl crate::Readable for SfrFrdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_frdone::W`](W) writer structure"]
impl crate::Writable for SfrFrdoneSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FRDONE to value 0"]
impl crate::Resettable for SfrFrdoneSpec {}
