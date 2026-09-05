#[doc = "Register `SFR_CHAIN_RNGCHAINEN2` reader"]
pub type R = crate::R<SfrChainRngchainen2Spec>;
#[doc = "Register `SFR_CHAIN_RNGCHAINEN2` writer"]
pub type W = crate::W<SfrChainRngchainen2Spec>;
#[doc = "Field `rngchainen2` reader - rngchainen read/write control register"]
pub type Rngchainen2R = crate::FieldReader<u32>;
#[doc = "Field `rngchainen2` writer - rngchainen read/write control register"]
pub type Rngchainen2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rngchainen read/write control register"]
    #[inline(always)]
    pub fn rngchainen2(&self) -> Rngchainen2R {
        Rngchainen2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rngchainen read/write control register"]
    #[inline(always)]
    pub fn rngchainen2(&mut self) -> Rngchainen2W<'_, SfrChainRngchainen2Spec> {
        Rngchainen2W::new(self, 0)
    }
}
#[doc = "See `trng.sv#L117 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L117>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_chain_rngchainen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_chain_rngchainen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrChainRngchainen2Spec;
impl crate::RegisterSpec for SfrChainRngchainen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_chain_rngchainen2::R`](R) reader structure"]
impl crate::Readable for SfrChainRngchainen2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_chain_rngchainen2::W`](W) writer structure"]
impl crate::Writable for SfrChainRngchainen2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CHAIN_RNGCHAINEN2 to value 0"]
impl crate::Resettable for SfrChainRngchainen2Spec {}
