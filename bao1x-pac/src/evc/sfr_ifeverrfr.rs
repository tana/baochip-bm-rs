#[doc = "Register `SFR_IFEVERRFR` reader"]
pub type R = crate::R<SfrIfeverrfrSpec>;
#[doc = "Register `SFR_IFEVERRFR` writer"]
pub type W = crate::W<SfrIfeverrfrSpec>;
#[doc = "Field `ifev_errs` reader - ifev_errs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type IfevErrsR = crate::FieldReader<u32>;
#[doc = "Field `ifev_errs` writer - ifev_errs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type IfevErrsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifev_errs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn ifev_errs(&self) -> IfevErrsR {
        IfevErrsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifev_errs flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn ifev_errs(&mut self) -> IfevErrsW<'_, SfrIfeverrfrSpec> {
        IfevErrsW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L148 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L148>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeverrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeverrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfeverrfrSpec;
impl crate::RegisterSpec for SfrIfeverrfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeverrfr::R`](R) reader structure"]
impl crate::Readable for SfrIfeverrfrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeverrfr::W`](W) writer structure"]
impl crate::Writable for SfrIfeverrfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVERRFR to value 0"]
impl crate::Resettable for SfrIfeverrfrSpec {}
