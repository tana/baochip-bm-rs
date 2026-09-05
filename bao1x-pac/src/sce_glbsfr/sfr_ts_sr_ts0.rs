#[doc = "Register `SFR_TS_SR_TS0` reader"]
pub type R = crate::R<SfrTsSrTs0Spec>;
#[doc = "Register `SFR_TS_SR_TS0` writer"]
pub type W = crate::W<SfrTsSrTs0Spec>;
#[doc = "Field `sr_ts0` reader - sr_ts read only status register"]
pub type SrTs0R = crate::FieldReader<u32>;
#[doc = "Field `sr_ts0` writer - sr_ts read only status register"]
pub type SrTs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts0(&self) -> SrTs0R {
        SrTs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_ts read only status register"]
    #[inline(always)]
    pub fn sr_ts0(&mut self) -> SrTs0W<'_, SfrTsSrTs0Spec> {
        SrTs0W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/mo dules/crypto_top/rtl/sce_glbsfra.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ts_sr_ts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ts_sr_ts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTsSrTs0Spec;
impl crate::RegisterSpec for SfrTsSrTs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ts_sr_ts0::R`](R) reader structure"]
impl crate::Readable for SfrTsSrTs0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ts_sr_ts0::W`](W) writer structure"]
impl crate::Writable for SfrTsSrTs0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TS_SR_TS0 to value 0"]
impl crate::Resettable for SfrTsSrTs0Spec {}
