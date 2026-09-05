#[doc = "Register `SFR_DBG_PADOE` reader"]
pub type R = crate::R<SfrDbgPadoeSpec>;
#[doc = "Register `SFR_DBG_PADOE` writer"]
pub type W = crate::W<SfrDbgPadoeSpec>;
#[doc = "Field `sfr_dbg_padoe` reader - sfr_dbg_padoe read only status register"]
pub type SfrDbgPadoeR = crate::FieldReader<u32>;
#[doc = "Field `sfr_dbg_padoe` writer - sfr_dbg_padoe read only status register"]
pub type SfrDbgPadoeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_dbg_padoe read only status register"]
    #[inline(always)]
    pub fn sfr_dbg_padoe(&self) -> SfrDbgPadoeR {
        SfrDbgPadoeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_dbg_padoe read only status register"]
    #[inline(always)]
    pub fn sfr_dbg_padoe(&mut self) -> SfrDbgPadoeW<'_, SfrDbgPadoeSpec> {
        SfrDbgPadoeW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L527 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L527>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg_padoe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg_padoe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDbgPadoeSpec;
impl crate::RegisterSpec for SfrDbgPadoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dbg_padoe::R`](R) reader structure"]
impl crate::Readable for SfrDbgPadoeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dbg_padoe::W`](W) writer structure"]
impl crate::Writable for SfrDbgPadoeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DBG_PADOE to value 0"]
impl crate::Resettable for SfrDbgPadoeSpec {}
