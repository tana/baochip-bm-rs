#[doc = "Register `SFR_INTFR` reader"]
pub type R = crate::R<SfrIntfrSpec>;
#[doc = "Register `SFR_INTFR` writer"]
pub type W = crate::W<SfrIntfrSpec>;
#[doc = "Field `frint` reader - frint flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrintR = crate::FieldReader;
#[doc = "Field `frint` writer - frint flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type FrintW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - frint flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn frint(&self) -> FrintR {
        FrintR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - frint flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn frint(&mut self) -> FrintW<'_, SfrIntfrSpec> {
        FrintW::new(self, 0)
    }
}
#[doc = "See `iox.sv#L128 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L128>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIntfrSpec;
impl crate::RegisterSpec for SfrIntfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_intfr::R`](R) reader structure"]
impl crate::Readable for SfrIntfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_intfr::W`](W) writer structure"]
impl crate::Writable for SfrIntfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_INTFR to value 0"]
impl crate::Resettable for SfrIntfrSpec {}
