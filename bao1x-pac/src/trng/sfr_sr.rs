#[doc = "Register `SFR_SR` reader"]
pub type R = crate::R<SfrSrSpec>;
#[doc = "Register `SFR_SR` writer"]
pub type W = crate::W<SfrSrSpec>;
#[doc = "Field `sr_rng` reader - sr_rng read only status register"]
pub type SrRngR = crate::FieldReader<u32>;
#[doc = "Field `sr_rng` writer - sr_rng read only status register"]
pub type SrRngW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_rng read only status register"]
    #[inline(always)]
    pub fn sr_rng(&self) -> SrRngR {
        SrRngR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_rng read only status register"]
    #[inline(always)]
    pub fn sr_rng(&mut self) -> SrRngW<'_, SfrSrSpec> {
        SrRngW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L114 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L114>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSpec;
impl crate::RegisterSpec for SfrSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr::R`](R) reader structure"]
impl crate::Readable for SfrSrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr::W`](W) writer structure"]
impl crate::Writable for SfrSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR to value 0"]
impl crate::Resettable for SfrSrSpec {}
