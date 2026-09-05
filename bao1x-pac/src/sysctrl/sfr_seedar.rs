#[doc = "Register `SFR_SEEDAR` reader"]
pub type R = crate::R<SfrSeedarSpec>;
#[doc = "Register `SFR_SEEDAR` writer"]
pub type W = crate::W<SfrSeedarSpec>;
#[doc = "Field `sfr_seedar` reader - sfr_seedar performs action on write of value: 0x5a"]
pub type SfrSeedarR = crate::FieldReader<u32>;
#[doc = "Field `sfr_seedar` writer - sfr_seedar performs action on write of value: 0x5a"]
pub type SfrSeedarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_seedar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_seedar(&self) -> SfrSeedarR {
        SfrSeedarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_seedar performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfr_seedar(&mut self) -> SfrSeedarW<'_, SfrSeedarSpec> {
        SfrSeedarW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L772 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L772>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_seedar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_seedar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSeedarSpec;
impl crate::RegisterSpec for SfrSeedarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_seedar::R`](R) reader structure"]
impl crate::Readable for SfrSeedarSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_seedar::W`](W) writer structure"]
impl crate::Writable for SfrSeedarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEEDAR to value 0"]
impl crate::Resettable for SfrSeedarSpec {}
