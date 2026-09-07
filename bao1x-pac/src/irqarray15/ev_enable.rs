#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `sec0` reader - Write a ``1`` to enable the ``sec0`` Event"]
pub type Sec0R = crate::BitReader;
#[doc = "Field `sec0` writer - Write a ``1`` to enable the ``sec0`` Event"]
pub type Sec0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s1` reader - Write a ``1`` to enable the ``nc_b15s1`` Event"]
pub type NcB15s1R = crate::BitReader;
#[doc = "Field `nc_b15s1` writer - Write a ``1`` to enable the ``nc_b15s1`` Event"]
pub type NcB15s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s2` reader - Write a ``1`` to enable the ``nc_b15s2`` Event"]
pub type NcB15s2R = crate::BitReader;
#[doc = "Field `nc_b15s2` writer - Write a ``1`` to enable the ``nc_b15s2`` Event"]
pub type NcB15s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s3` reader - Write a ``1`` to enable the ``nc_b15s3`` Event"]
pub type NcB15s3R = crate::BitReader;
#[doc = "Field `nc_b15s3` writer - Write a ``1`` to enable the ``nc_b15s3`` Event"]
pub type NcB15s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s4` reader - Write a ``1`` to enable the ``nc_b15s4`` Event"]
pub type NcB15s4R = crate::BitReader;
#[doc = "Field `nc_b15s4` writer - Write a ``1`` to enable the ``nc_b15s4`` Event"]
pub type NcB15s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s5` reader - Write a ``1`` to enable the ``nc_b15s5`` Event"]
pub type NcB15s5R = crate::BitReader;
#[doc = "Field `nc_b15s5` writer - Write a ``1`` to enable the ``nc_b15s5`` Event"]
pub type NcB15s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s6` reader - Write a ``1`` to enable the ``nc_b15s6`` Event"]
pub type NcB15s6R = crate::BitReader;
#[doc = "Field `nc_b15s6` writer - Write a ``1`` to enable the ``nc_b15s6`` Event"]
pub type NcB15s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s7` reader - Write a ``1`` to enable the ``nc_b15s7`` Event"]
pub type NcB15s7R = crate::BitReader;
#[doc = "Field `nc_b15s7` writer - Write a ``1`` to enable the ``nc_b15s7`` Event"]
pub type NcB15s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s8` reader - Write a ``1`` to enable the ``nc_b15s8`` Event"]
pub type NcB15s8R = crate::BitReader;
#[doc = "Field `nc_b15s8` writer - Write a ``1`` to enable the ``nc_b15s8`` Event"]
pub type NcB15s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s9` reader - Write a ``1`` to enable the ``nc_b15s9`` Event"]
pub type NcB15s9R = crate::BitReader;
#[doc = "Field `nc_b15s9` writer - Write a ``1`` to enable the ``nc_b15s9`` Event"]
pub type NcB15s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s10` reader - Write a ``1`` to enable the ``nc_b15s10`` Event"]
pub type NcB15s10R = crate::BitReader;
#[doc = "Field `nc_b15s10` writer - Write a ``1`` to enable the ``nc_b15s10`` Event"]
pub type NcB15s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s11` reader - Write a ``1`` to enable the ``nc_b15s11`` Event"]
pub type NcB15s11R = crate::BitReader;
#[doc = "Field `nc_b15s11` writer - Write a ``1`` to enable the ``nc_b15s11`` Event"]
pub type NcB15s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s12` reader - Write a ``1`` to enable the ``nc_b15s12`` Event"]
pub type NcB15s12R = crate::BitReader;
#[doc = "Field `nc_b15s12` writer - Write a ``1`` to enable the ``nc_b15s12`` Event"]
pub type NcB15s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s13` reader - Write a ``1`` to enable the ``nc_b15s13`` Event"]
pub type NcB15s13R = crate::BitReader;
#[doc = "Field `nc_b15s13` writer - Write a ``1`` to enable the ``nc_b15s13`` Event"]
pub type NcB15s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s14` reader - Write a ``1`` to enable the ``nc_b15s14`` Event"]
pub type NcB15s14R = crate::BitReader;
#[doc = "Field `nc_b15s14` writer - Write a ``1`` to enable the ``nc_b15s14`` Event"]
pub type NcB15s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b15s15` reader - Write a ``1`` to enable the ``nc_b15s15`` Event"]
pub type NcB15s15R = crate::BitReader;
#[doc = "Field `nc_b15s15` writer - Write a ``1`` to enable the ``nc_b15s15`` Event"]
pub type NcB15s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``sec0`` Event"]
    #[inline(always)]
    pub fn sec0(&self) -> Sec0R {
        Sec0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b15s1`` Event"]
    #[inline(always)]
    pub fn nc_b15s1(&self) -> NcB15s1R {
        NcB15s1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b15s2`` Event"]
    #[inline(always)]
    pub fn nc_b15s2(&self) -> NcB15s2R {
        NcB15s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b15s3`` Event"]
    #[inline(always)]
    pub fn nc_b15s3(&self) -> NcB15s3R {
        NcB15s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b15s4`` Event"]
    #[inline(always)]
    pub fn nc_b15s4(&self) -> NcB15s4R {
        NcB15s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b15s5`` Event"]
    #[inline(always)]
    pub fn nc_b15s5(&self) -> NcB15s5R {
        NcB15s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b15s6`` Event"]
    #[inline(always)]
    pub fn nc_b15s6(&self) -> NcB15s6R {
        NcB15s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b15s7`` Event"]
    #[inline(always)]
    pub fn nc_b15s7(&self) -> NcB15s7R {
        NcB15s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b15s8`` Event"]
    #[inline(always)]
    pub fn nc_b15s8(&self) -> NcB15s8R {
        NcB15s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b15s9`` Event"]
    #[inline(always)]
    pub fn nc_b15s9(&self) -> NcB15s9R {
        NcB15s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b15s10`` Event"]
    #[inline(always)]
    pub fn nc_b15s10(&self) -> NcB15s10R {
        NcB15s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b15s11`` Event"]
    #[inline(always)]
    pub fn nc_b15s11(&self) -> NcB15s11R {
        NcB15s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b15s12`` Event"]
    #[inline(always)]
    pub fn nc_b15s12(&self) -> NcB15s12R {
        NcB15s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b15s13`` Event"]
    #[inline(always)]
    pub fn nc_b15s13(&self) -> NcB15s13R {
        NcB15s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b15s14`` Event"]
    #[inline(always)]
    pub fn nc_b15s14(&self) -> NcB15s14R {
        NcB15s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b15s15`` Event"]
    #[inline(always)]
    pub fn nc_b15s15(&self) -> NcB15s15R {
        NcB15s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``sec0`` Event"]
    #[inline(always)]
    pub fn sec0(&mut self) -> Sec0W<'_, EvEnableSpec> {
        Sec0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b15s1`` Event"]
    #[inline(always)]
    pub fn nc_b15s1(&mut self) -> NcB15s1W<'_, EvEnableSpec> {
        NcB15s1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b15s2`` Event"]
    #[inline(always)]
    pub fn nc_b15s2(&mut self) -> NcB15s2W<'_, EvEnableSpec> {
        NcB15s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b15s3`` Event"]
    #[inline(always)]
    pub fn nc_b15s3(&mut self) -> NcB15s3W<'_, EvEnableSpec> {
        NcB15s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b15s4`` Event"]
    #[inline(always)]
    pub fn nc_b15s4(&mut self) -> NcB15s4W<'_, EvEnableSpec> {
        NcB15s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b15s5`` Event"]
    #[inline(always)]
    pub fn nc_b15s5(&mut self) -> NcB15s5W<'_, EvEnableSpec> {
        NcB15s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b15s6`` Event"]
    #[inline(always)]
    pub fn nc_b15s6(&mut self) -> NcB15s6W<'_, EvEnableSpec> {
        NcB15s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b15s7`` Event"]
    #[inline(always)]
    pub fn nc_b15s7(&mut self) -> NcB15s7W<'_, EvEnableSpec> {
        NcB15s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b15s8`` Event"]
    #[inline(always)]
    pub fn nc_b15s8(&mut self) -> NcB15s8W<'_, EvEnableSpec> {
        NcB15s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b15s9`` Event"]
    #[inline(always)]
    pub fn nc_b15s9(&mut self) -> NcB15s9W<'_, EvEnableSpec> {
        NcB15s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b15s10`` Event"]
    #[inline(always)]
    pub fn nc_b15s10(&mut self) -> NcB15s10W<'_, EvEnableSpec> {
        NcB15s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b15s11`` Event"]
    #[inline(always)]
    pub fn nc_b15s11(&mut self) -> NcB15s11W<'_, EvEnableSpec> {
        NcB15s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b15s12`` Event"]
    #[inline(always)]
    pub fn nc_b15s12(&mut self) -> NcB15s12W<'_, EvEnableSpec> {
        NcB15s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b15s13`` Event"]
    #[inline(always)]
    pub fn nc_b15s13(&mut self) -> NcB15s13W<'_, EvEnableSpec> {
        NcB15s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b15s14`` Event"]
    #[inline(always)]
    pub fn nc_b15s14(&mut self) -> NcB15s14W<'_, EvEnableSpec> {
        NcB15s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b15s15`` Event"]
    #[inline(always)]
    pub fn nc_b15s15(&mut self) -> NcB15s15W<'_, EvEnableSpec> {
        NcB15s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b15s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
