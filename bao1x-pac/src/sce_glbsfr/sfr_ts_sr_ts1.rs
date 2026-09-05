#[doc = "Register `SFR_TS_SR_TS1` reader"]
pub type R = crate::R<SfrTsSrTs1Spec>;
#[doc = "Register `SFR_TS_SR_TS1` writer"]
pub type W = crate::W<SfrTsSrTs1Spec>;
#[doc = "Field `sr_ts1` reader - sr_ts read only status register"]
pub type SrTs1R = crate::FieldReader<u32>;
#[doc = "Field `sr_ts1` writer - sr_ts read only status register"]
pub type SrTs1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts1(&self) -> SrTs1R {
        SrTs1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts1(&mut self) -> SrTs1W<'_, SfrTsSrTs1Spec> {
        SrTs1W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTsSrTs1Spec;
impl crate::RegisterSpec for SfrTsSrTs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ts_sr_ts1::R`](R) reader structure"]
impl crate::Readable for SfrTsSrTs1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ts_sr_ts1::W`](W) writer structure"]
impl crate::Writable for SfrTsSrTs1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TS_SR_TS1 to value 0"]
impl crate::Resettable for SfrTsSrTs1Spec {}
