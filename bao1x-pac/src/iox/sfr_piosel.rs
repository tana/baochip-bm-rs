#[doc = "Register `SFR_PIOSEL` reader"]
pub type R = crate::R<SfrPioselSpec>;
#[doc = "Register `SFR_PIOSEL` writer"]
pub type W = crate::W<SfrPioselSpec>;
#[doc = "Field `piosel` reader - piosel read/write control register"]
pub type PioselR = crate::FieldReader<u32>;
#[doc = "Field `piosel` writer - piosel read/write control register"]
pub type PioselW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - piosel read/write control register"]
    #[inline(always)]
    pub fn piosel(&self) -> PioselR {
        PioselR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - piosel read/write control register"]
    #[inline(always)]
    pub fn piosel(&mut self) -> PioselW<'_, SfrPioselSpec> {
        PioselW::new(self, 0)
    }
}
#[doc = "See `iox.sv#L249 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L249>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_piosel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_piosel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrPioselSpec;
impl crate::RegisterSpec for SfrPioselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_piosel::R`](R) reader structure"]
impl crate::Readable for SfrPioselSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_piosel::W`](W) writer structure"]
impl crate::Writable for SfrPioselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_PIOSEL to value 0"]
impl crate::Resettable for SfrPioselSpec {}
