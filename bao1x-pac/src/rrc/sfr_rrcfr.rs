#[doc = "Register `SFR_RRCFR` reader"]
pub type R = crate::R<SfrRrcfrSpec>;
#[doc = "Register `SFR_RRCFR` writer"]
pub type W = crate::W<SfrRrcfrSpec>;
#[doc = "Field `sfr_rrcfr` reader - sfr_rrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrRrcfrR = crate::FieldReader;
#[doc = "Field `sfr_rrcfr` writer - sfr_rrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrRrcfrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_rrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_rrcfr(&self) -> SfrRrcfrR {
        SfrRrcfrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_rrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_rrcfr(&mut self) -> SfrRrcfrW<'_, SfrRrcfrSpec> {
        SfrRrcfrW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L264 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L264>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcfrSpec;
impl crate::RegisterSpec for SfrRrcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcfr::R`](R) reader structure"]
impl crate::Readable for SfrRrcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcfr::W`](W) writer structure"]
impl crate::Writable for SfrRrcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCFR to value 0"]
impl crate::Resettable for SfrRrcfrSpec {}
