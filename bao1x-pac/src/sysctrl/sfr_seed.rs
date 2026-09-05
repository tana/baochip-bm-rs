#[doc = "Register `SFR_SEED` reader"]
pub type R = crate::R<SfrSeedSpec>;
#[doc = "Register `SFR_SEED` writer"]
pub type W = crate::W<SfrSeedSpec>;
#[doc = "Field `sfr_seed` reader - sfr_seed read/write control register"]
pub type SfrSeedR = crate::FieldReader<u32>;
#[doc = "Field `sfr_seed` writer - sfr_seed read/write control register"]
pub type SfrSeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_seed read/write control register"]
    #[inline(always)]
    pub fn sfr_seed(&self) -> SfrSeedR {
        SfrSeedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_seed read/write control register"]
    #[inline(always)]
    pub fn sfr_seed(&mut self) -> SfrSeedW<'_, SfrSeedSpec> {
        SfrSeedW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L771 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L771>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_seed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_seed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSeedSpec;
impl crate::RegisterSpec for SfrSeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_seed::R`](R) reader structure"]
impl crate::Readable for SfrSeedSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_seed::W`](W) writer structure"]
impl crate::Writable for SfrSeedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEED to value 0"]
impl crate::Resettable for SfrSeedSpec {}
