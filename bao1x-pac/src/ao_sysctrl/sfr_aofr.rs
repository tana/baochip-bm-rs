#[doc = "Register `SFR_AOFR` reader"]
pub type R = crate::R<SfrAofrSpec>;
#[doc = "Register `SFR_AOFR` writer"]
pub type W = crate::W<SfrAofrSpec>;
#[doc = "Field `sfr_aofr` reader - sfr_aofr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrAofrR = crate::FieldReader<u16>;
#[doc = "Field `sfr_aofr` writer - sfr_aofr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrAofrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sfr_aofr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_aofr(&self) -> SfrAofrR {
        SfrAofrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sfr_aofr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_aofr(&mut self) -> SfrAofrW<'_, SfrAofrSpec> {
        SfrAofrW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L390 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L390>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_aofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_aofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAofrSpec;
impl crate::RegisterSpec for SfrAofrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_aofr::R`](R) reader structure"]
impl crate::Readable for SfrAofrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_aofr::W`](W) writer structure"]
impl crate::Writable for SfrAofrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AOFR to value 0"]
impl crate::Resettable for SfrAofrSpec {}
