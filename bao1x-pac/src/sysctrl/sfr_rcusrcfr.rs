#[doc = "Register `SFR_RCUSRCFR` reader"]
pub type R = crate::R<SfrRcusrcfrSpec>;
#[doc = "Register `SFR_RCUSRCFR` writer"]
pub type W = crate::W<SfrRcusrcfrSpec>;
#[doc = "Field `sfr_rcusrcfr` reader - sfr_rcusrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrRcusrcfrR = crate::FieldReader<u16>;
#[doc = "Field `sfr_rcusrcfr` writer - sfr_rcusrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrRcusrcfrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_rcusrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_rcusrcfr(&self) -> SfrRcusrcfrR {
        SfrRcusrcfrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_rcusrcfr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_rcusrcfr(&mut self) -> SfrRcusrcfrW<'_, SfrRcusrcfrSpec> {
        SfrRcusrcfrW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L806 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L806>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcusrcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcusrcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRcusrcfrSpec;
impl crate::RegisterSpec for SfrRcusrcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rcusrcfr::R`](R) reader structure"]
impl crate::Readable for SfrRcusrcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rcusrcfr::W`](W) writer structure"]
impl crate::Writable for SfrRcusrcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RCUSRCFR to value 0"]
impl crate::Resettable for SfrRcusrcfrSpec {}
