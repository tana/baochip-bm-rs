#[doc = "Register `SFR_CM7EVFR` reader"]
pub type R = crate::R<SfrCm7evfrSpec>;
#[doc = "Register `SFR_CM7EVFR` writer"]
pub type W = crate::W<SfrCm7evfrSpec>;
#[doc = "Field `cm7evs` reader - cm7evs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type Cm7evsR = crate::FieldReader;
#[doc = "Field `cm7evs` writer - cm7evs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type Cm7evsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cm7evs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn cm7evs(&self) -> Cm7evsR {
        Cm7evsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cm7evs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn cm7evs(&mut self) -> Cm7evsW<'_, SfrCm7evfrSpec> {
        Cm7evsW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L142 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L142>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7evfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7evfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7evfrSpec;
impl crate::RegisterSpec for SfrCm7evfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7evfr::R`](R) reader structure"]
impl crate::Readable for SfrCm7evfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7evfr::W`](W) writer structure"]
impl crate::Writable for SfrCm7evfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7EVFR to value 0"]
impl crate::Resettable for SfrCm7evfrSpec {}
