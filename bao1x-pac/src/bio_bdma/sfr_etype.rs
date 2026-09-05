#[doc = "Register `SFR_ETYPE` reader"]
pub type R = crate::R<SfrEtypeSpec>;
#[doc = "Register `SFR_ETYPE` writer"]
pub type W = crate::W<SfrEtypeSpec>;
#[doc = "Field `fifo_event_lt_mask` reader - fifo_event_lt_mask read/write control register"]
pub type FifoEventLtMaskR = crate::FieldReader;
#[doc = "Field `fifo_event_lt_mask` writer - fifo_event_lt_mask read/write control register"]
pub type FifoEventLtMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `fifo_event_eq_mask` reader - fifo_event_eq_mask read/write control register"]
pub type FifoEventEqMaskR = crate::FieldReader;
#[doc = "Field `fifo_event_eq_mask` writer - fifo_event_eq_mask read/write control register"]
pub type FifoEventEqMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `fifo_event_gt_mask` reader - fifo_event_gt_mask read/write control register"]
pub type FifoEventGtMaskR = crate::FieldReader;
#[doc = "Field `fifo_event_gt_mask` writer - fifo_event_gt_mask read/write control register"]
pub type FifoEventGtMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - fifo_event_lt_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_lt_mask(&self) -> FifoEventLtMaskR {
        FifoEventLtMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fifo_event_eq_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_eq_mask(&self) -> FifoEventEqMaskR {
        FifoEventEqMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fifo_event_gt_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_gt_mask(&self) -> FifoEventGtMaskR {
        FifoEventGtMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - fifo_event_lt_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_lt_mask(&mut self) -> FifoEventLtMaskW<'_, SfrEtypeSpec> {
        FifoEventLtMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fifo_event_eq_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_eq_mask(&mut self) -> FifoEventEqMaskW<'_, SfrEtypeSpec> {
        FifoEventEqMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fifo_event_gt_mask read/write control register"]
    #[inline(always)]
    pub fn fifo_event_gt_mask(&mut self) -> FifoEventGtMaskW<'_, SfrEtypeSpec> {
        FifoEventGtMaskW::new(self, 16)
    }
}
#[doc = "See `bio_bdma.sv#L503 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L503>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_etype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_etype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEtypeSpec;
impl crate::RegisterSpec for SfrEtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_etype::R`](R) reader structure"]
impl crate::Readable for SfrEtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_etype::W`](W) writer structure"]
impl crate::Writable for SfrEtypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ETYPE to value 0"]
impl crate::Resettable for SfrEtypeSpec {}
