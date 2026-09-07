#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `i2s_rx_dupe` reader - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
pub type I2sRxDupeR = crate::BitReader;
#[doc = "Field `i2s_rx_dupe` writer - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
pub type I2sRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx_dupe` reader - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
pub type I2sTxDupeR = crate::BitReader;
#[doc = "Field `i2s_tx_dupe` writer - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
pub type I2sTxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s2` reader - Write a ``1`` to enable the ``nc_b11s2`` Event"]
pub type NcB11s2R = crate::BitReader;
#[doc = "Field `nc_b11s2` writer - Write a ``1`` to enable the ``nc_b11s2`` Event"]
pub type NcB11s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s3` reader - Write a ``1`` to enable the ``nc_b11s3`` Event"]
pub type NcB11s3R = crate::BitReader;
#[doc = "Field `nc_b11s3` writer - Write a ``1`` to enable the ``nc_b11s3`` Event"]
pub type NcB11s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s4` reader - Write a ``1`` to enable the ``nc_b11s4`` Event"]
pub type NcB11s4R = crate::BitReader;
#[doc = "Field `nc_b11s4` writer - Write a ``1`` to enable the ``nc_b11s4`` Event"]
pub type NcB11s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s5` reader - Write a ``1`` to enable the ``nc_b11s5`` Event"]
pub type NcB11s5R = crate::BitReader;
#[doc = "Field `nc_b11s5` writer - Write a ``1`` to enable the ``nc_b11s5`` Event"]
pub type NcB11s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s6` reader - Write a ``1`` to enable the ``nc_b11s6`` Event"]
pub type NcB11s6R = crate::BitReader;
#[doc = "Field `nc_b11s6` writer - Write a ``1`` to enable the ``nc_b11s6`` Event"]
pub type NcB11s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s7` reader - Write a ``1`` to enable the ``nc_b11s7`` Event"]
pub type NcB11s7R = crate::BitReader;
#[doc = "Field `nc_b11s7` writer - Write a ``1`` to enable the ``nc_b11s7`` Event"]
pub type NcB11s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s8` reader - Write a ``1`` to enable the ``nc_b11s8`` Event"]
pub type NcB11s8R = crate::BitReader;
#[doc = "Field `nc_b11s8` writer - Write a ``1`` to enable the ``nc_b11s8`` Event"]
pub type NcB11s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s9` reader - Write a ``1`` to enable the ``nc_b11s9`` Event"]
pub type NcB11s9R = crate::BitReader;
#[doc = "Field `nc_b11s9` writer - Write a ``1`` to enable the ``nc_b11s9`` Event"]
pub type NcB11s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s10` reader - Write a ``1`` to enable the ``nc_b11s10`` Event"]
pub type NcB11s10R = crate::BitReader;
#[doc = "Field `nc_b11s10` writer - Write a ``1`` to enable the ``nc_b11s10`` Event"]
pub type NcB11s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s11` reader - Write a ``1`` to enable the ``nc_b11s11`` Event"]
pub type NcB11s11R = crate::BitReader;
#[doc = "Field `nc_b11s11` writer - Write a ``1`` to enable the ``nc_b11s11`` Event"]
pub type NcB11s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s12` reader - Write a ``1`` to enable the ``nc_b11s12`` Event"]
pub type NcB11s12R = crate::BitReader;
#[doc = "Field `nc_b11s12` writer - Write a ``1`` to enable the ``nc_b11s12`` Event"]
pub type NcB11s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s13` reader - Write a ``1`` to enable the ``nc_b11s13`` Event"]
pub type NcB11s13R = crate::BitReader;
#[doc = "Field `nc_b11s13` writer - Write a ``1`` to enable the ``nc_b11s13`` Event"]
pub type NcB11s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s14` reader - Write a ``1`` to enable the ``nc_b11s14`` Event"]
pub type NcB11s14R = crate::BitReader;
#[doc = "Field `nc_b11s14` writer - Write a ``1`` to enable the ``nc_b11s14`` Event"]
pub type NcB11s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b11s15` reader - Write a ``1`` to enable the ``nc_b11s15`` Event"]
pub type NcB11s15R = crate::BitReader;
#[doc = "Field `nc_b11s15` writer - Write a ``1`` to enable the ``nc_b11s15`` Event"]
pub type NcB11s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&self) -> I2sRxDupeR {
        I2sRxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&self) -> I2sTxDupeR {
        I2sTxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b11s2`` Event"]
    #[inline(always)]
    pub fn nc_b11s2(&self) -> NcB11s2R {
        NcB11s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b11s3`` Event"]
    #[inline(always)]
    pub fn nc_b11s3(&self) -> NcB11s3R {
        NcB11s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b11s4`` Event"]
    #[inline(always)]
    pub fn nc_b11s4(&self) -> NcB11s4R {
        NcB11s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b11s5`` Event"]
    #[inline(always)]
    pub fn nc_b11s5(&self) -> NcB11s5R {
        NcB11s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b11s6`` Event"]
    #[inline(always)]
    pub fn nc_b11s6(&self) -> NcB11s6R {
        NcB11s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b11s7`` Event"]
    #[inline(always)]
    pub fn nc_b11s7(&self) -> NcB11s7R {
        NcB11s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b11s8`` Event"]
    #[inline(always)]
    pub fn nc_b11s8(&self) -> NcB11s8R {
        NcB11s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b11s9`` Event"]
    #[inline(always)]
    pub fn nc_b11s9(&self) -> NcB11s9R {
        NcB11s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b11s10`` Event"]
    #[inline(always)]
    pub fn nc_b11s10(&self) -> NcB11s10R {
        NcB11s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b11s11`` Event"]
    #[inline(always)]
    pub fn nc_b11s11(&self) -> NcB11s11R {
        NcB11s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b11s12`` Event"]
    #[inline(always)]
    pub fn nc_b11s12(&self) -> NcB11s12R {
        NcB11s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b11s13`` Event"]
    #[inline(always)]
    pub fn nc_b11s13(&self) -> NcB11s13R {
        NcB11s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b11s14`` Event"]
    #[inline(always)]
    pub fn nc_b11s14(&self) -> NcB11s14R {
        NcB11s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b11s15`` Event"]
    #[inline(always)]
    pub fn nc_b11s15(&self) -> NcB11s15R {
        NcB11s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&mut self) -> I2sRxDupeW<'_, EvEnableSpec> {
        I2sRxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&mut self) -> I2sTxDupeW<'_, EvEnableSpec> {
        I2sTxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b11s2`` Event"]
    #[inline(always)]
    pub fn nc_b11s2(&mut self) -> NcB11s2W<'_, EvEnableSpec> {
        NcB11s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b11s3`` Event"]
    #[inline(always)]
    pub fn nc_b11s3(&mut self) -> NcB11s3W<'_, EvEnableSpec> {
        NcB11s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b11s4`` Event"]
    #[inline(always)]
    pub fn nc_b11s4(&mut self) -> NcB11s4W<'_, EvEnableSpec> {
        NcB11s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b11s5`` Event"]
    #[inline(always)]
    pub fn nc_b11s5(&mut self) -> NcB11s5W<'_, EvEnableSpec> {
        NcB11s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b11s6`` Event"]
    #[inline(always)]
    pub fn nc_b11s6(&mut self) -> NcB11s6W<'_, EvEnableSpec> {
        NcB11s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b11s7`` Event"]
    #[inline(always)]
    pub fn nc_b11s7(&mut self) -> NcB11s7W<'_, EvEnableSpec> {
        NcB11s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b11s8`` Event"]
    #[inline(always)]
    pub fn nc_b11s8(&mut self) -> NcB11s8W<'_, EvEnableSpec> {
        NcB11s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b11s9`` Event"]
    #[inline(always)]
    pub fn nc_b11s9(&mut self) -> NcB11s9W<'_, EvEnableSpec> {
        NcB11s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b11s10`` Event"]
    #[inline(always)]
    pub fn nc_b11s10(&mut self) -> NcB11s10W<'_, EvEnableSpec> {
        NcB11s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b11s11`` Event"]
    #[inline(always)]
    pub fn nc_b11s11(&mut self) -> NcB11s11W<'_, EvEnableSpec> {
        NcB11s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b11s12`` Event"]
    #[inline(always)]
    pub fn nc_b11s12(&mut self) -> NcB11s12W<'_, EvEnableSpec> {
        NcB11s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b11s13`` Event"]
    #[inline(always)]
    pub fn nc_b11s13(&mut self) -> NcB11s13W<'_, EvEnableSpec> {
        NcB11s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b11s14`` Event"]
    #[inline(always)]
    pub fn nc_b11s14(&mut self) -> NcB11s14W<'_, EvEnableSpec> {
        NcB11s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b11s15`` Event"]
    #[inline(always)]
    pub fn nc_b11s15(&mut self) -> NcB11s15W<'_, EvEnableSpec> {
        NcB11s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b11s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
