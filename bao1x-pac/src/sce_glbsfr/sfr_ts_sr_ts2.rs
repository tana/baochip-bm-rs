#[doc = "Register `SFR_TS_SR_TS2` reader"]
pub type R = crate::R<SfrTsSrTs2Spec>;
#[doc = "Register `SFR_TS_SR_TS2` writer"]
pub type W = crate::W<SfrTsSrTs2Spec>;
#[doc = "Field `sr_ts2` reader - sr_ts read only status register"]
pub type SrTs2R = crate::FieldReader<u32>;
#[doc = "Field `sr_ts2` writer - sr_ts read only status register"]
pub type SrTs2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts2(&self) -> SrTs2R {
        SrTs2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts2(&mut self) -> SrTs2W<'_, SfrTsSrTs2Spec> {
        SrTs2W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTsSrTs2Spec;
impl crate::RegisterSpec for SfrTsSrTs2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ts_sr_ts2::R`](R) reader structure"]
impl crate::Readable for SfrTsSrTs2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ts_sr_ts2::W`](W) writer structure"]
impl crate::Writable for SfrTsSrTs2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TS_SR_TS2 to value 0"]
impl crate::Resettable for SfrTsSrTs2Spec {}
