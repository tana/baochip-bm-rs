#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `coresuberr` reader - Write a ``1`` to enable the ``coresuberr`` Event"]
pub type CoresuberrR = crate::BitReader;
#[doc = "Field `coresuberr` writer - Write a ``1`` to enable the ``coresuberr`` Event"]
pub type CoresuberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sceerr` reader - Write a ``1`` to enable the ``sceerr`` Event"]
pub type SceerrR = crate::BitReader;
#[doc = "Field `sceerr` writer - Write a ``1`` to enable the ``sceerr`` Event"]
pub type SceerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ifsuberr` reader - Write a ``1`` to enable the ``ifsuberr`` Event"]
pub type IfsuberrR = crate::BitReader;
#[doc = "Field `ifsuberr` writer - Write a ``1`` to enable the ``ifsuberr`` Event"]
pub type IfsuberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `secirq` reader - Write a ``1`` to enable the ``secirq`` Event"]
pub type SecirqR = crate::BitReader;
#[doc = "Field `secirq` writer - Write a ``1`` to enable the ``secirq`` Event"]
pub type SecirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s4` reader - Write a ``1`` to enable the ``nc_b13s4`` Event"]
pub type NcB13s4R = crate::BitReader;
#[doc = "Field `nc_b13s4` writer - Write a ``1`` to enable the ``nc_b13s4`` Event"]
pub type NcB13s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s5` reader - Write a ``1`` to enable the ``nc_b13s5`` Event"]
pub type NcB13s5R = crate::BitReader;
#[doc = "Field `nc_b13s5` writer - Write a ``1`` to enable the ``nc_b13s5`` Event"]
pub type NcB13s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s6` reader - Write a ``1`` to enable the ``nc_b13s6`` Event"]
pub type NcB13s6R = crate::BitReader;
#[doc = "Field `nc_b13s6` writer - Write a ``1`` to enable the ``nc_b13s6`` Event"]
pub type NcB13s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s7` reader - Write a ``1`` to enable the ``nc_b13s7`` Event"]
pub type NcB13s7R = crate::BitReader;
#[doc = "Field `nc_b13s7` writer - Write a ``1`` to enable the ``nc_b13s7`` Event"]
pub type NcB13s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s8` reader - Write a ``1`` to enable the ``nc_b13s8`` Event"]
pub type NcB13s8R = crate::BitReader;
#[doc = "Field `nc_b13s8` writer - Write a ``1`` to enable the ``nc_b13s8`` Event"]
pub type NcB13s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s9` reader - Write a ``1`` to enable the ``nc_b13s9`` Event"]
pub type NcB13s9R = crate::BitReader;
#[doc = "Field `nc_b13s9` writer - Write a ``1`` to enable the ``nc_b13s9`` Event"]
pub type NcB13s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s10` reader - Write a ``1`` to enable the ``nc_b13s10`` Event"]
pub type NcB13s10R = crate::BitReader;
#[doc = "Field `nc_b13s10` writer - Write a ``1`` to enable the ``nc_b13s10`` Event"]
pub type NcB13s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s11` reader - Write a ``1`` to enable the ``nc_b13s11`` Event"]
pub type NcB13s11R = crate::BitReader;
#[doc = "Field `nc_b13s11` writer - Write a ``1`` to enable the ``nc_b13s11`` Event"]
pub type NcB13s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s12` reader - Write a ``1`` to enable the ``nc_b13s12`` Event"]
pub type NcB13s12R = crate::BitReader;
#[doc = "Field `nc_b13s12` writer - Write a ``1`` to enable the ``nc_b13s12`` Event"]
pub type NcB13s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s13` reader - Write a ``1`` to enable the ``nc_b13s13`` Event"]
pub type NcB13s13R = crate::BitReader;
#[doc = "Field `nc_b13s13` writer - Write a ``1`` to enable the ``nc_b13s13`` Event"]
pub type NcB13s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s14` reader - Write a ``1`` to enable the ``nc_b13s14`` Event"]
pub type NcB13s14R = crate::BitReader;
#[doc = "Field `nc_b13s14` writer - Write a ``1`` to enable the ``nc_b13s14`` Event"]
pub type NcB13s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s15` reader - Write a ``1`` to enable the ``nc_b13s15`` Event"]
pub type NcB13s15R = crate::BitReader;
#[doc = "Field `nc_b13s15` writer - Write a ``1`` to enable the ``nc_b13s15`` Event"]
pub type NcB13s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``coresuberr`` Event"]
    #[inline(always)]
    pub fn coresuberr(&self) -> CoresuberrR {
        CoresuberrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``sceerr`` Event"]
    #[inline(always)]
    pub fn sceerr(&self) -> SceerrR {
        SceerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``ifsuberr`` Event"]
    #[inline(always)]
    pub fn ifsuberr(&self) -> IfsuberrR {
        IfsuberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``secirq`` Event"]
    #[inline(always)]
    pub fn secirq(&self) -> SecirqR {
        SecirqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b13s4`` Event"]
    #[inline(always)]
    pub fn nc_b13s4(&self) -> NcB13s4R {
        NcB13s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b13s5`` Event"]
    #[inline(always)]
    pub fn nc_b13s5(&self) -> NcB13s5R {
        NcB13s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b13s6`` Event"]
    #[inline(always)]
    pub fn nc_b13s6(&self) -> NcB13s6R {
        NcB13s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b13s7`` Event"]
    #[inline(always)]
    pub fn nc_b13s7(&self) -> NcB13s7R {
        NcB13s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b13s8`` Event"]
    #[inline(always)]
    pub fn nc_b13s8(&self) -> NcB13s8R {
        NcB13s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b13s9`` Event"]
    #[inline(always)]
    pub fn nc_b13s9(&self) -> NcB13s9R {
        NcB13s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b13s10`` Event"]
    #[inline(always)]
    pub fn nc_b13s10(&self) -> NcB13s10R {
        NcB13s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b13s11`` Event"]
    #[inline(always)]
    pub fn nc_b13s11(&self) -> NcB13s11R {
        NcB13s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b13s12`` Event"]
    #[inline(always)]
    pub fn nc_b13s12(&self) -> NcB13s12R {
        NcB13s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b13s13`` Event"]
    #[inline(always)]
    pub fn nc_b13s13(&self) -> NcB13s13R {
        NcB13s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b13s14`` Event"]
    #[inline(always)]
    pub fn nc_b13s14(&self) -> NcB13s14R {
        NcB13s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b13s15`` Event"]
    #[inline(always)]
    pub fn nc_b13s15(&self) -> NcB13s15R {
        NcB13s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``coresuberr`` Event"]
    #[inline(always)]
    pub fn coresuberr(&mut self) -> CoresuberrW<'_, EvEnableSpec> {
        CoresuberrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``sceerr`` Event"]
    #[inline(always)]
    pub fn sceerr(&mut self) -> SceerrW<'_, EvEnableSpec> {
        SceerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``ifsuberr`` Event"]
    #[inline(always)]
    pub fn ifsuberr(&mut self) -> IfsuberrW<'_, EvEnableSpec> {
        IfsuberrW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``secirq`` Event"]
    #[inline(always)]
    pub fn secirq(&mut self) -> SecirqW<'_, EvEnableSpec> {
        SecirqW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``nc_b13s4`` Event"]
    #[inline(always)]
    pub fn nc_b13s4(&mut self) -> NcB13s4W<'_, EvEnableSpec> {
        NcB13s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``nc_b13s5`` Event"]
    #[inline(always)]
    pub fn nc_b13s5(&mut self) -> NcB13s5W<'_, EvEnableSpec> {
        NcB13s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b13s6`` Event"]
    #[inline(always)]
    pub fn nc_b13s6(&mut self) -> NcB13s6W<'_, EvEnableSpec> {
        NcB13s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b13s7`` Event"]
    #[inline(always)]
    pub fn nc_b13s7(&mut self) -> NcB13s7W<'_, EvEnableSpec> {
        NcB13s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b13s8`` Event"]
    #[inline(always)]
    pub fn nc_b13s8(&mut self) -> NcB13s8W<'_, EvEnableSpec> {
        NcB13s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b13s9`` Event"]
    #[inline(always)]
    pub fn nc_b13s9(&mut self) -> NcB13s9W<'_, EvEnableSpec> {
        NcB13s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b13s10`` Event"]
    #[inline(always)]
    pub fn nc_b13s10(&mut self) -> NcB13s10W<'_, EvEnableSpec> {
        NcB13s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b13s11`` Event"]
    #[inline(always)]
    pub fn nc_b13s11(&mut self) -> NcB13s11W<'_, EvEnableSpec> {
        NcB13s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b13s12`` Event"]
    #[inline(always)]
    pub fn nc_b13s12(&mut self) -> NcB13s12W<'_, EvEnableSpec> {
        NcB13s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b13s13`` Event"]
    #[inline(always)]
    pub fn nc_b13s13(&mut self) -> NcB13s13W<'_, EvEnableSpec> {
        NcB13s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b13s14`` Event"]
    #[inline(always)]
    pub fn nc_b13s14(&mut self) -> NcB13s14W<'_, EvEnableSpec> {
        NcB13s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b13s15`` Event"]
    #[inline(always)]
    pub fn nc_b13s15(&mut self) -> NcB13s15W<'_, EvEnableSpec> {
        NcB13s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b13s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
