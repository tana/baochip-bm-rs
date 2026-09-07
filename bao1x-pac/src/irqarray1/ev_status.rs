#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `usbc_dupe` reader - Level of the ``usbc_dupe`` event"]
pub type UsbcDupeR = crate::BitReader;
#[doc = "Field `usbc_dupe` writer - Level of the ``usbc_dupe`` event"]
pub type UsbcDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s1` reader - Level of the ``nc_b1s1`` event"]
pub type NcB1s1R = crate::BitReader;
#[doc = "Field `nc_b1s1` writer - Level of the ``nc_b1s1`` event"]
pub type NcB1s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s2` reader - Level of the ``nc_b1s2`` event"]
pub type NcB1s2R = crate::BitReader;
#[doc = "Field `nc_b1s2` writer - Level of the ``nc_b1s2`` event"]
pub type NcB1s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s3` reader - Level of the ``nc_b1s3`` event"]
pub type NcB1s3R = crate::BitReader;
#[doc = "Field `nc_b1s3` writer - Level of the ``nc_b1s3`` event"]
pub type NcB1s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s4` reader - Level of the ``nc_b1s4`` event"]
pub type NcB1s4R = crate::BitReader;
#[doc = "Field `nc_b1s4` writer - Level of the ``nc_b1s4`` event"]
pub type NcB1s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s5` reader - Level of the ``nc_b1s5`` event"]
pub type NcB1s5R = crate::BitReader;
#[doc = "Field `nc_b1s5` writer - Level of the ``nc_b1s5`` event"]
pub type NcB1s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s6` reader - Level of the ``nc_b1s6`` event"]
pub type NcB1s6R = crate::BitReader;
#[doc = "Field `nc_b1s6` writer - Level of the ``nc_b1s6`` event"]
pub type NcB1s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s7` reader - Level of the ``nc_b1s7`` event"]
pub type NcB1s7R = crate::BitReader;
#[doc = "Field `nc_b1s7` writer - Level of the ``nc_b1s7`` event"]
pub type NcB1s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s8` reader - Level of the ``nc_b1s8`` event"]
pub type NcB1s8R = crate::BitReader;
#[doc = "Field `nc_b1s8` writer - Level of the ``nc_b1s8`` event"]
pub type NcB1s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s9` reader - Level of the ``nc_b1s9`` event"]
pub type NcB1s9R = crate::BitReader;
#[doc = "Field `nc_b1s9` writer - Level of the ``nc_b1s9`` event"]
pub type NcB1s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s10` reader - Level of the ``nc_b1s10`` event"]
pub type NcB1s10R = crate::BitReader;
#[doc = "Field `nc_b1s10` writer - Level of the ``nc_b1s10`` event"]
pub type NcB1s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s11` reader - Level of the ``nc_b1s11`` event"]
pub type NcB1s11R = crate::BitReader;
#[doc = "Field `nc_b1s11` writer - Level of the ``nc_b1s11`` event"]
pub type NcB1s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s12` reader - Level of the ``nc_b1s12`` event"]
pub type NcB1s12R = crate::BitReader;
#[doc = "Field `nc_b1s12` writer - Level of the ``nc_b1s12`` event"]
pub type NcB1s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s13` reader - Level of the ``nc_b1s13`` event"]
pub type NcB1s13R = crate::BitReader;
#[doc = "Field `nc_b1s13` writer - Level of the ``nc_b1s13`` event"]
pub type NcB1s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s14` reader - Level of the ``nc_b1s14`` event"]
pub type NcB1s14R = crate::BitReader;
#[doc = "Field `nc_b1s14` writer - Level of the ``nc_b1s14`` event"]
pub type NcB1s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b1s15` reader - Level of the ``nc_b1s15`` event"]
pub type NcB1s15R = crate::BitReader;
#[doc = "Field `nc_b1s15` writer - Level of the ``nc_b1s15`` event"]
pub type NcB1s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``usbc_dupe`` event"]
    #[inline(always)]
    pub fn usbc_dupe(&self) -> UsbcDupeR {
        UsbcDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``nc_b1s1`` event"]
    #[inline(always)]
    pub fn nc_b1s1(&self) -> NcB1s1R {
        NcB1s1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``nc_b1s2`` event"]
    #[inline(always)]
    pub fn nc_b1s2(&self) -> NcB1s2R {
        NcB1s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``nc_b1s3`` event"]
    #[inline(always)]
    pub fn nc_b1s3(&self) -> NcB1s3R {
        NcB1s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``nc_b1s4`` event"]
    #[inline(always)]
    pub fn nc_b1s4(&self) -> NcB1s4R {
        NcB1s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``nc_b1s5`` event"]
    #[inline(always)]
    pub fn nc_b1s5(&self) -> NcB1s5R {
        NcB1s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``nc_b1s6`` event"]
    #[inline(always)]
    pub fn nc_b1s6(&self) -> NcB1s6R {
        NcB1s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b1s7`` event"]
    #[inline(always)]
    pub fn nc_b1s7(&self) -> NcB1s7R {
        NcB1s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b1s8`` event"]
    #[inline(always)]
    pub fn nc_b1s8(&self) -> NcB1s8R {
        NcB1s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b1s9`` event"]
    #[inline(always)]
    pub fn nc_b1s9(&self) -> NcB1s9R {
        NcB1s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b1s10`` event"]
    #[inline(always)]
    pub fn nc_b1s10(&self) -> NcB1s10R {
        NcB1s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b1s11`` event"]
    #[inline(always)]
    pub fn nc_b1s11(&self) -> NcB1s11R {
        NcB1s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b1s12`` event"]
    #[inline(always)]
    pub fn nc_b1s12(&self) -> NcB1s12R {
        NcB1s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b1s13`` event"]
    #[inline(always)]
    pub fn nc_b1s13(&self) -> NcB1s13R {
        NcB1s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b1s14`` event"]
    #[inline(always)]
    pub fn nc_b1s14(&self) -> NcB1s14R {
        NcB1s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b1s15`` event"]
    #[inline(always)]
    pub fn nc_b1s15(&self) -> NcB1s15R {
        NcB1s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``usbc_dupe`` event"]
    #[inline(always)]
    pub fn usbc_dupe(&mut self) -> UsbcDupeW<'_, EvStatusSpec> {
        UsbcDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``nc_b1s1`` event"]
    #[inline(always)]
    pub fn nc_b1s1(&mut self) -> NcB1s1W<'_, EvStatusSpec> {
        NcB1s1W::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``nc_b1s2`` event"]
    #[inline(always)]
    pub fn nc_b1s2(&mut self) -> NcB1s2W<'_, EvStatusSpec> {
        NcB1s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``nc_b1s3`` event"]
    #[inline(always)]
    pub fn nc_b1s3(&mut self) -> NcB1s3W<'_, EvStatusSpec> {
        NcB1s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``nc_b1s4`` event"]
    #[inline(always)]
    pub fn nc_b1s4(&mut self) -> NcB1s4W<'_, EvStatusSpec> {
        NcB1s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``nc_b1s5`` event"]
    #[inline(always)]
    pub fn nc_b1s5(&mut self) -> NcB1s5W<'_, EvStatusSpec> {
        NcB1s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``nc_b1s6`` event"]
    #[inline(always)]
    pub fn nc_b1s6(&mut self) -> NcB1s6W<'_, EvStatusSpec> {
        NcB1s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b1s7`` event"]
    #[inline(always)]
    pub fn nc_b1s7(&mut self) -> NcB1s7W<'_, EvStatusSpec> {
        NcB1s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b1s8`` event"]
    #[inline(always)]
    pub fn nc_b1s8(&mut self) -> NcB1s8W<'_, EvStatusSpec> {
        NcB1s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b1s9`` event"]
    #[inline(always)]
    pub fn nc_b1s9(&mut self) -> NcB1s9W<'_, EvStatusSpec> {
        NcB1s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b1s10`` event"]
    #[inline(always)]
    pub fn nc_b1s10(&mut self) -> NcB1s10W<'_, EvStatusSpec> {
        NcB1s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b1s11`` event"]
    #[inline(always)]
    pub fn nc_b1s11(&mut self) -> NcB1s11W<'_, EvStatusSpec> {
        NcB1s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b1s12`` event"]
    #[inline(always)]
    pub fn nc_b1s12(&mut self) -> NcB1s12W<'_, EvStatusSpec> {
        NcB1s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b1s13`` event"]
    #[inline(always)]
    pub fn nc_b1s13(&mut self) -> NcB1s13W<'_, EvStatusSpec> {
        NcB1s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b1s14`` event"]
    #[inline(always)]
    pub fn nc_b1s14(&mut self) -> NcB1s14W<'_, EvStatusSpec> {
        NcB1s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b1s15`` event"]
    #[inline(always)]
    pub fn nc_b1s15(&mut self) -> NcB1s15W<'_, EvStatusSpec> {
        NcB1s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b1s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStatusSpec;
impl crate::RegisterSpec for EvStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EvStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_status::W`](W) writer structure"]
impl crate::Writable for EvStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_STATUS to value 0"]
impl crate::Resettable for EvStatusSpec {}
