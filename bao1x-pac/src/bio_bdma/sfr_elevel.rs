#[doc = "Register `SFR_ELEVEL` reader"]
pub type R = crate::R<SfrElevelSpec>;
#[doc = "Register `SFR_ELEVEL` writer"]
pub type W = crate::W<SfrElevelSpec>;
#[doc = "Field `fifo_event_level0` reader - fifo_event_level\\[0\\] read/write control register"]
pub type FifoEventLevel0R = crate::FieldReader;
#[doc = "Field `fifo_event_level0` writer - fifo_event_level\\[0\\] read/write control register"]
pub type FifoEventLevel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level1` reader - fifo_event_level\\[1\\] read/write control register"]
pub type FifoEventLevel1R = crate::FieldReader;
#[doc = "Field `fifo_event_level1` writer - fifo_event_level\\[1\\] read/write control register"]
pub type FifoEventLevel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level2` reader - fifo_event_level\\[2\\] read/write control register"]
pub type FifoEventLevel2R = crate::FieldReader;
#[doc = "Field `fifo_event_level2` writer - fifo_event_level\\[2\\] read/write control register"]
pub type FifoEventLevel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level3` reader - fifo_event_level\\[3\\] read/write control register"]
pub type FifoEventLevel3R = crate::FieldReader;
#[doc = "Field `fifo_event_level3` writer - fifo_event_level\\[3\\] read/write control register"]
pub type FifoEventLevel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level4` reader - fifo_event_level\\[4\\] read/write control register"]
pub type FifoEventLevel4R = crate::FieldReader;
#[doc = "Field `fifo_event_level4` writer - fifo_event_level\\[4\\] read/write control register"]
pub type FifoEventLevel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level5` reader - fifo_event_level\\[5\\] read/write control register"]
pub type FifoEventLevel5R = crate::FieldReader;
#[doc = "Field `fifo_event_level5` writer - fifo_event_level\\[5\\] read/write control register"]
pub type FifoEventLevel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level6` reader - fifo_event_level\\[6\\] read/write control register"]
pub type FifoEventLevel6R = crate::FieldReader;
#[doc = "Field `fifo_event_level6` writer - fifo_event_level\\[6\\] read/write control register"]
pub type FifoEventLevel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fifo_event_level7` reader - fifo_event_level\\[7\\] read/write control register"]
pub type FifoEventLevel7R = crate::FieldReader;
#[doc = "Field `fifo_event_level7` writer - fifo_event_level\\[7\\] read/write control register"]
pub type FifoEventLevel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - fifo_event_level\\[0\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level0(&self) -> FifoEventLevel0R {
        FifoEventLevel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - fifo_event_level\\[1\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level1(&self) -> FifoEventLevel1R {
        FifoEventLevel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - fifo_event_level\\[2\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level2(&self) -> FifoEventLevel2R {
        FifoEventLevel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - fifo_event_level\\[3\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level3(&self) -> FifoEventLevel3R {
        FifoEventLevel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - fifo_event_level\\[4\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level4(&self) -> FifoEventLevel4R {
        FifoEventLevel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - fifo_event_level\\[5\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level5(&self) -> FifoEventLevel5R {
        FifoEventLevel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - fifo_event_level\\[6\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level6(&self) -> FifoEventLevel6R {
        FifoEventLevel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - fifo_event_level\\[7\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level7(&self) -> FifoEventLevel7R {
        FifoEventLevel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - fifo_event_level\\[0\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level0(&mut self) -> FifoEventLevel0W<'_, SfrElevelSpec> {
        FifoEventLevel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - fifo_event_level\\[1\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level1(&mut self) -> FifoEventLevel1W<'_, SfrElevelSpec> {
        FifoEventLevel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - fifo_event_level\\[2\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level2(&mut self) -> FifoEventLevel2W<'_, SfrElevelSpec> {
        FifoEventLevel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - fifo_event_level\\[3\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level3(&mut self) -> FifoEventLevel3W<'_, SfrElevelSpec> {
        FifoEventLevel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - fifo_event_level\\[4\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level4(&mut self) -> FifoEventLevel4W<'_, SfrElevelSpec> {
        FifoEventLevel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - fifo_event_level\\[5\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level5(&mut self) -> FifoEventLevel5W<'_, SfrElevelSpec> {
        FifoEventLevel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - fifo_event_level\\[6\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level6(&mut self) -> FifoEventLevel6W<'_, SfrElevelSpec> {
        FifoEventLevel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - fifo_event_level\\[7\\] read/write control register"]
    #[inline(always)]
    pub fn fifo_event_level7(&mut self) -> FifoEventLevel7W<'_, SfrElevelSpec> {
        FifoEventLevel7W::new(self, 28)
    }
}
#[doc = "See `bio_bdma.sv#L502 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L502>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_elevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_elevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrElevelSpec;
impl crate::RegisterSpec for SfrElevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_elevel::R`](R) reader structure"]
impl crate::Readable for SfrElevelSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_elevel::W`](W) writer structure"]
impl crate::Writable for SfrElevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ELEVEL to value 0"]
impl crate::Resettable for SfrElevelSpec {}
