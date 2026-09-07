#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `usbc_dupe` reader - Write a ``1`` to enable the ``usbc_dupe`` Event"]
pub type UsbcDupeR = crate::BitReader;
#[doc = "Field `usbc_dupe` writer - Write a ``1`` to enable the ``usbc_dupe`` Event"]
pub type UsbcDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s1` reader - Write a ``1`` to enable the ``nc_b1s1`` Event"]
pub type NcB1s1R = crate::BitReader;
#[doc = "Field `nc_b1s1` writer - Write a ``1`` to enable the ``nc_b1s1`` Event"]
pub type NcB1s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s2` reader - Write a ``1`` to enable the ``nc_b1s2`` Event"]
pub type NcB1s2R = crate::BitReader;
#[doc = "Field `nc_b1s2` writer - Write a ``1`` to enable the ``nc_b1s2`` Event"]
pub type NcB1s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s3` reader - Write a ``1`` to enable the ``nc_b1s3`` Event"]
pub type NcB1s3R = crate::BitReader;
#[doc = "Field `nc_b1s3` writer - Write a ``1`` to enable the ``nc_b1s3`` Event"]
pub type NcB1s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s4` reader - Write a ``1`` to enable the ``nc_b1s4`` Event"]
pub type NcB1s4R = crate::BitReader;
#[doc = "Field `nc_b1s4` writer - Write a ``1`` to enable the ``nc_b1s4`` Event"]
pub type NcB1s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s5` reader - Write a ``1`` to enable the ``nc_b1s5`` Event"]
pub type NcB1s5R = crate::BitReader;
#[doc = "Field `nc_b1s5` writer - Write a ``1`` to enable the ``nc_b1s5`` Event"]
pub type NcB1s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s6` reader - Write a ``1`` to enable the ``nc_b1s6`` Event"]
pub type NcB1s6R = crate::BitReader;
#[doc = "Field `nc_b1s6` writer - Write a ``1`` to enable the ``nc_b1s6`` Event"]
pub type NcB1s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s7` reader - Write a ``1`` to enable the ``nc_b1s7`` Event"]
pub type NcB1s7R = crate::BitReader;
#[doc = "Field `nc_b1s7` writer - Write a ``1`` to enable the ``nc_b1s7`` Event"]
pub type NcB1s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s8` reader - Write a ``1`` to enable the ``nc_b1s8`` Event"]
pub type NcB1s8R = crate::BitReader;
#[doc = "Field `nc_b1s8` writer - Write a ``1`` to enable the ``nc_b1s8`` Event"]
pub type NcB1s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s9` reader - Write a ``1`` to enable the ``nc_b1s9`` Event"]
pub type NcB1s9R = crate::BitReader;
#[doc = "Field `nc_b1s9` writer - Write a ``1`` to enable the ``nc_b1s9`` Event"]
pub type NcB1s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s10` reader - Write a ``1`` to enable the ``nc_b1s10`` Event"]
pub type NcB1s10R = crate::BitReader;
#[doc = "Field `nc_b1s10` writer - Write a ``1`` to enable the ``nc_b1s10`` Event"]
pub type NcB1s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s11` reader - Write a ``1`` to enable the ``nc_b1s11`` Event"]
pub type NcB1s11R = crate::BitReader;
#[doc = "Field `nc_b1s11` writer - Write a ``1`` to enable the ``nc_b1s11`` Event"]
pub type NcB1s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s12` reader - Write a ``1`` to enable the ``nc_b1s12`` Event"]
pub type NcB1s12R = crate::BitReader;
#[doc = "Field `nc_b1s12` writer - Write a ``1`` to enable the ``nc_b1s12`` Event"]
pub type NcB1s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s13` reader - Write a ``1`` to enable the ``nc_b1s13`` Event"]
pub type NcB1s13R = crate::BitReader;
#[doc = "Field `nc_b1s13` writer - Write a ``1`` to enable the ``nc_b1s13`` Event"]
pub type NcB1s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s14` reader - Write a ``1`` to enable the ``nc_b1s14`` Event"]
pub type NcB1s14R = crate::BitReader;
#[doc = "Field `nc_b1s14` writer - Write a ``1`` to enable the ``nc_b1s14`` Event"]
pub type NcB1s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s15` reader - Write a ``1`` to enable the ``nc_b1s15`` Event"]
pub type NcB1s15R = crate::BitReader;
#[doc = "Field `nc_b1s15` writer - Write a ``1`` to enable the ``nc_b1s15`` Event"]
pub type NcB1s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``usbc_dupe`` Event"]
    #[inline(always)]
    pub fn usbc_dupe(&self) -> UsbcDupeR {
        UsbcDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b1s1`` Event"]
    #[inline(always)]
    pub fn nc_b1s1(&self) -> NcB1s1R {
        NcB1s1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b1s2`` Event"]
    #[inline(always)]
    pub fn nc_b1s2(&self) -> NcB1s2R {
        NcB1s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b1s3`` Event"]
    #[inline(always)]
    pub fn nc_b1s3(&self) -> NcB1s3R {
        NcB1s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b1s4`` Event"]
    #[inline(always)]
    pub fn nc_b1s4(&self) -> NcB1s4R {
        NcB1s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b1s5`` Event"]
    #[inline(always)]
    pub fn nc_b1s5(&self) -> NcB1s5R {
        NcB1s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b1s6`` Event"]
    #[inline(always)]
    pub fn nc_b1s6(&self) -> NcB1s6R {
        NcB1s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b1s7`` Event"]
    #[inline(always)]
    pub fn nc_b1s7(&self) -> NcB1s7R {
        NcB1s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b1s8`` Event"]
    #[inline(always)]
    pub fn nc_b1s8(&self) -> NcB1s8R {
        NcB1s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b1s9`` Event"]
    #[inline(always)]
    pub fn nc_b1s9(&self) -> NcB1s9R {
        NcB1s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b1s10`` Event"]
    #[inline(always)]
    pub fn nc_b1s10(&self) -> NcB1s10R {
        NcB1s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b1s11`` Event"]
    #[inline(always)]
    pub fn nc_b1s11(&self) -> NcB1s11R {
        NcB1s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b1s12`` Event"]
    #[inline(always)]
    pub fn nc_b1s12(&self) -> NcB1s12R {
        NcB1s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b1s13`` Event"]
    #[inline(always)]
    pub fn nc_b1s13(&self) -> NcB1s13R {
        NcB1s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b1s14`` Event"]
    #[inline(always)]
    pub fn nc_b1s14(&self) -> NcB1s14R {
        NcB1s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b1s15`` Event"]
    #[inline(always)]
    pub fn nc_b1s15(&self) -> NcB1s15R {
        NcB1s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``usbc_dupe`` Event"]
    #[inline(always)]
    pub fn usbc_dupe(&mut self) -> UsbcDupeW<'_, EvEnableSpec> {
        UsbcDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b1s1`` Event"]
    #[inline(always)]
    pub fn nc_b1s1(&mut self) -> NcB1s1W<'_, EvEnableSpec> {
        NcB1s1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b1s2`` Event"]
    #[inline(always)]
    pub fn nc_b1s2(&mut self) -> NcB1s2W<'_, EvEnableSpec> {
        NcB1s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b1s3`` Event"]
    #[inline(always)]
    pub fn nc_b1s3(&mut self) -> NcB1s3W<'_, EvEnableSpec> {
        NcB1s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b1s4`` Event"]
    #[inline(always)]
    pub fn nc_b1s4(&mut self) -> NcB1s4W<'_, EvEnableSpec> {
        NcB1s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b1s5`` Event"]
    #[inline(always)]
    pub fn nc_b1s5(&mut self) -> NcB1s5W<'_, EvEnableSpec> {
        NcB1s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b1s6`` Event"]
    #[inline(always)]
    pub fn nc_b1s6(&mut self) -> NcB1s6W<'_, EvEnableSpec> {
        NcB1s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b1s7`` Event"]
    #[inline(always)]
    pub fn nc_b1s7(&mut self) -> NcB1s7W<'_, EvEnableSpec> {
        NcB1s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b1s8`` Event"]
    #[inline(always)]
    pub fn nc_b1s8(&mut self) -> NcB1s8W<'_, EvEnableSpec> {
        NcB1s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b1s9`` Event"]
    #[inline(always)]
    pub fn nc_b1s9(&mut self) -> NcB1s9W<'_, EvEnableSpec> {
        NcB1s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b1s10`` Event"]
    #[inline(always)]
    pub fn nc_b1s10(&mut self) -> NcB1s10W<'_, EvEnableSpec> {
        NcB1s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b1s11`` Event"]
    #[inline(always)]
    pub fn nc_b1s11(&mut self) -> NcB1s11W<'_, EvEnableSpec> {
        NcB1s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b1s12`` Event"]
    #[inline(always)]
    pub fn nc_b1s12(&mut self) -> NcB1s12W<'_, EvEnableSpec> {
        NcB1s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b1s13`` Event"]
    #[inline(always)]
    pub fn nc_b1s13(&mut self) -> NcB1s13W<'_, EvEnableSpec> {
        NcB1s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b1s14`` Event"]
    #[inline(always)]
    pub fn nc_b1s14(&mut self) -> NcB1s14W<'_, EvEnableSpec> {
        NcB1s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b1s15`` Event"]
    #[inline(always)]
    pub fn nc_b1s15(&mut self) -> NcB1s15W<'_, EvEnableSpec> {
        NcB1s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b1s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
