#[doc = "Register `SFR_CGUFDAO` reader"]
pub type R = crate::R<SfrCgufdaoSpec>;
#[doc = "Register `SFR_CGUFDAO` writer"]
pub type W = crate::W<SfrCgufdaoSpec>;
#[doc = "Field `cfgfdcr` reader - cfgfdcr read/write control register"]
pub type CfgfdcrR = crate::FieldReader<u32>;
#[doc = "Field `cfgfdcr` writer - cfgfdcr read/write control register"]
pub type CfgfdcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cfgfdcr read/write control register"]
    #[inline(always)]
    pub fn cfgfdcr(&self) -> CfgfdcrR {
        CfgfdcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cfgfdcr read/write control register"]
    #[inline(always)]
    pub fn cfgfdcr(&mut self) -> CfgfdcrW<'_, SfrCgufdaoSpec> {
        CfgfdcrW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L778 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L778>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdao::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdao::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufdaoSpec;
impl crate::RegisterSpec for SfrCgufdaoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufdao::R`](R) reader structure"]
impl crate::Readable for SfrCgufdaoSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufdao::W`](W) writer structure"]
impl crate::Writable for SfrCgufdaoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFDAO to value 0"]
impl crate::Resettable for SfrCgufdaoSpec {}
