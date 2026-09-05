#[doc = "Register `SFR_CM7ERRFR` reader"]
pub type R = crate::R<SfrCm7errfrSpec>;
#[doc = "Register `SFR_CM7ERRFR` writer"]
pub type W = crate::W<SfrCm7errfrSpec>;
#[doc = "Field `errin` reader - errin flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ErrinR = crate::FieldReader<u32>;
#[doc = "Field `errin` writer - errin flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ErrinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - errin flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn errin(&self) -> ErrinR {
        ErrinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - errin flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn errin(&mut self) -> ErrinW<'_, SfrCm7errfrSpec> {
        ErrinW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L150 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L150>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7errfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7errfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7errfrSpec;
impl crate::RegisterSpec for SfrCm7errfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7errfr::R`](R) reader structure"]
impl crate::Readable for SfrCm7errfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7errfr::W`](W) writer structure"]
impl crate::Writable for SfrCm7errfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7ERRFR to value 0"]
impl crate::Resettable for SfrCm7errfrSpec {}
