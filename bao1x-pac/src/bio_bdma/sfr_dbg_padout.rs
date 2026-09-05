#[doc = "Register `SFR_DBG_PADOUT` reader"]
pub type R = crate::R<SfrDbgPadoutSpec>;
#[doc = "Register `SFR_DBG_PADOUT` writer"]
pub type W = crate::W<SfrDbgPadoutSpec>;
#[doc = "Field `sfr_dbg_padout` reader - sfr_dbg_padout read only status register"]
pub type SfrDbgPadoutR = crate::FieldReader<u32>;
#[doc = "Field `sfr_dbg_padout` writer - sfr_dbg_padout read only status register"]
pub type SfrDbgPadoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_dbg_padout read only status register"]
    #[inline(always)]
    pub fn sfr_dbg_padout(&self) -> SfrDbgPadoutR {
        SfrDbgPadoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_dbg_padout read only status register"]
    #[inline(always)]
    pub fn sfr_dbg_padout(&mut self) -> SfrDbgPadoutW<'_, SfrDbgPadoutSpec> {
        SfrDbgPadoutW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L526 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L526>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_dbg_padout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_dbg_padout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrDbgPadoutSpec;
impl crate::RegisterSpec for SfrDbgPadoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_dbg_padout::R`](R) reader structure"]
impl crate::Readable for SfrDbgPadoutSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_dbg_padout::W`](W) writer structure"]
impl crate::Writable for SfrDbgPadoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_DBG_PADOUT to value 0"]
impl crate::Resettable for SfrDbgPadoutSpec {}
