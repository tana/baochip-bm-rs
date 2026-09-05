#[doc = "Register `SFR_CGUFDAORAM` reader"]
pub type R = crate::R<SfrCgufdaoramSpec>;
#[doc = "Register `SFR_CGUFDAORAM` writer"]
pub type W = crate::W<SfrCgufdaoramSpec>;
#[doc = "Field `sfr_cgufdaoram` reader - sfr_cgufdaoram read/write control register"]
pub type SfrCgufdaoramR = crate::FieldReader<u16>;
#[doc = "Field `sfr_cgufdaoram` writer - sfr_cgufdaoram read/write control register"]
pub type SfrCgufdaoramW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_cgufdaoram read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdaoram(&self) -> SfrCgufdaoramR {
        SfrCgufdaoramR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_cgufdaoram read/write control register"]
    #[inline(always)]
    pub fn sfr_cgufdaoram(&mut self) -> SfrCgufdaoramW<'_, SfrCgufdaoramSpec> {
        SfrCgufdaoramW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L779 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L779>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgufdaoram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgufdaoram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgufdaoramSpec;
impl crate::RegisterSpec for SfrCgufdaoramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgufdaoram::R`](R) reader structure"]
impl crate::Readable for SfrCgufdaoramSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgufdaoram::W`](W) writer structure"]
impl crate::Writable for SfrCgufdaoramSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUFDAORAM to value 0"]
impl crate::Resettable for SfrCgufdaoramSpec {}
