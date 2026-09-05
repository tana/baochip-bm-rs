#[doc = "Register `SFR_SYNC_BYPASS` reader"]
pub type R = crate::R<SfrSyncBypassSpec>;
#[doc = "Register `SFR_SYNC_BYPASS` writer"]
pub type W = crate::W<SfrSyncBypassSpec>;
#[doc = "Field `sfr_sync_bypass` reader - sfr_sync_bypass read/write control register"]
pub type SfrSyncBypassR = crate::FieldReader<u32>;
#[doc = "Field `sfr_sync_bypass` writer - sfr_sync_bypass read/write control register"]
pub type SfrSyncBypassW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_sync_bypass read/write control register"]
    #[inline(always)]
    pub fn sfr_sync_bypass(&self) -> SfrSyncBypassR {
        SfrSyncBypassR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_sync_bypass read/write control register"]
    #[inline(always)]
    pub fn sfr_sync_bypass(&mut self) -> SfrSyncBypassW<'_, SfrSyncBypassSpec> {
        SfrSyncBypassW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L516 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L516>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sync_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sync_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSyncBypassSpec;
impl crate::RegisterSpec for SfrSyncBypassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sync_bypass::R`](R) reader structure"]
impl crate::Readable for SfrSyncBypassSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sync_bypass::W`](W) writer structure"]
impl crate::Writable for SfrSyncBypassSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SYNC_BYPASS to value 0"]
impl crate::Resettable for SfrSyncBypassSpec {}
