#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `coresuberr` reader - Level of the ``coresuberr`` event"]
pub type CoresuberrR = crate::BitReader;
#[doc = "Field `coresuberr` writer - Level of the ``coresuberr`` event"]
pub type CoresuberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sceerr` reader - Level of the ``sceerr`` event"]
pub type SceerrR = crate::BitReader;
#[doc = "Field `sceerr` writer - Level of the ``sceerr`` event"]
pub type SceerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ifsuberr` reader - Level of the ``ifsuberr`` event"]
pub type IfsuberrR = crate::BitReader;
#[doc = "Field `ifsuberr` writer - Level of the ``ifsuberr`` event"]
pub type IfsuberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `secirq` reader - Level of the ``secirq`` event"]
pub type SecirqR = crate::BitReader;
#[doc = "Field `secirq` writer - Level of the ``secirq`` event"]
pub type SecirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s4` reader - Level of the ``nc_b13s4`` event"]
pub type NcB13s4R = crate::BitReader;
#[doc = "Field `nc_b13s4` writer - Level of the ``nc_b13s4`` event"]
pub type NcB13s4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s5` reader - Level of the ``nc_b13s5`` event"]
pub type NcB13s5R = crate::BitReader;
#[doc = "Field `nc_b13s5` writer - Level of the ``nc_b13s5`` event"]
pub type NcB13s5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s6` reader - Level of the ``nc_b13s6`` event"]
pub type NcB13s6R = crate::BitReader;
#[doc = "Field `nc_b13s6` writer - Level of the ``nc_b13s6`` event"]
pub type NcB13s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s7` reader - Level of the ``nc_b13s7`` event"]
pub type NcB13s7R = crate::BitReader;
#[doc = "Field `nc_b13s7` writer - Level of the ``nc_b13s7`` event"]
pub type NcB13s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s8` reader - Level of the ``nc_b13s8`` event"]
pub type NcB13s8R = crate::BitReader;
#[doc = "Field `nc_b13s8` writer - Level of the ``nc_b13s8`` event"]
pub type NcB13s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s9` reader - Level of the ``nc_b13s9`` event"]
pub type NcB13s9R = crate::BitReader;
#[doc = "Field `nc_b13s9` writer - Level of the ``nc_b13s9`` event"]
pub type NcB13s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s10` reader - Level of the ``nc_b13s10`` event"]
pub type NcB13s10R = crate::BitReader;
#[doc = "Field `nc_b13s10` writer - Level of the ``nc_b13s10`` event"]
pub type NcB13s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s11` reader - Level of the ``nc_b13s11`` event"]
pub type NcB13s11R = crate::BitReader;
#[doc = "Field `nc_b13s11` writer - Level of the ``nc_b13s11`` event"]
pub type NcB13s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s12` reader - Level of the ``nc_b13s12`` event"]
pub type NcB13s12R = crate::BitReader;
#[doc = "Field `nc_b13s12` writer - Level of the ``nc_b13s12`` event"]
pub type NcB13s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s13` reader - Level of the ``nc_b13s13`` event"]
pub type NcB13s13R = crate::BitReader;
#[doc = "Field `nc_b13s13` writer - Level of the ``nc_b13s13`` event"]
pub type NcB13s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s14` reader - Level of the ``nc_b13s14`` event"]
pub type NcB13s14R = crate::BitReader;
#[doc = "Field `nc_b13s14` writer - Level of the ``nc_b13s14`` event"]
pub type NcB13s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b13s15` reader - Level of the ``nc_b13s15`` event"]
pub type NcB13s15R = crate::BitReader;
#[doc = "Field `nc_b13s15` writer - Level of the ``nc_b13s15`` event"]
pub type NcB13s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``coresuberr`` event"]
    #[inline(always)]
    pub fn coresuberr(&self) -> CoresuberrR {
        CoresuberrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``sceerr`` event"]
    #[inline(always)]
    pub fn sceerr(&self) -> SceerrR {
        SceerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``ifsuberr`` event"]
    #[inline(always)]
    pub fn ifsuberr(&self) -> IfsuberrR {
        IfsuberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``secirq`` event"]
    #[inline(always)]
    pub fn secirq(&self) -> SecirqR {
        SecirqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``nc_b13s4`` event"]
    #[inline(always)]
    pub fn nc_b13s4(&self) -> NcB13s4R {
        NcB13s4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``nc_b13s5`` event"]
    #[inline(always)]
    pub fn nc_b13s5(&self) -> NcB13s5R {
        NcB13s5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``nc_b13s6`` event"]
    #[inline(always)]
    pub fn nc_b13s6(&self) -> NcB13s6R {
        NcB13s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b13s7`` event"]
    #[inline(always)]
    pub fn nc_b13s7(&self) -> NcB13s7R {
        NcB13s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b13s8`` event"]
    #[inline(always)]
    pub fn nc_b13s8(&self) -> NcB13s8R {
        NcB13s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b13s9`` event"]
    #[inline(always)]
    pub fn nc_b13s9(&self) -> NcB13s9R {
        NcB13s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b13s10`` event"]
    #[inline(always)]
    pub fn nc_b13s10(&self) -> NcB13s10R {
        NcB13s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b13s11`` event"]
    #[inline(always)]
    pub fn nc_b13s11(&self) -> NcB13s11R {
        NcB13s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b13s12`` event"]
    #[inline(always)]
    pub fn nc_b13s12(&self) -> NcB13s12R {
        NcB13s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b13s13`` event"]
    #[inline(always)]
    pub fn nc_b13s13(&self) -> NcB13s13R {
        NcB13s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b13s14`` event"]
    #[inline(always)]
    pub fn nc_b13s14(&self) -> NcB13s14R {
        NcB13s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b13s15`` event"]
    #[inline(always)]
    pub fn nc_b13s15(&self) -> NcB13s15R {
        NcB13s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``coresuberr`` event"]
    #[inline(always)]
    pub fn coresuberr(&mut self) -> CoresuberrW<'_, EvStatusSpec> {
        CoresuberrW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``sceerr`` event"]
    #[inline(always)]
    pub fn sceerr(&mut self) -> SceerrW<'_, EvStatusSpec> {
        SceerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``ifsuberr`` event"]
    #[inline(always)]
    pub fn ifsuberr(&mut self) -> IfsuberrW<'_, EvStatusSpec> {
        IfsuberrW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``secirq`` event"]
    #[inline(always)]
    pub fn secirq(&mut self) -> SecirqW<'_, EvStatusSpec> {
        SecirqW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``nc_b13s4`` event"]
    #[inline(always)]
    pub fn nc_b13s4(&mut self) -> NcB13s4W<'_, EvStatusSpec> {
        NcB13s4W::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``nc_b13s5`` event"]
    #[inline(always)]
    pub fn nc_b13s5(&mut self) -> NcB13s5W<'_, EvStatusSpec> {
        NcB13s5W::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``nc_b13s6`` event"]
    #[inline(always)]
    pub fn nc_b13s6(&mut self) -> NcB13s6W<'_, EvStatusSpec> {
        NcB13s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b13s7`` event"]
    #[inline(always)]
    pub fn nc_b13s7(&mut self) -> NcB13s7W<'_, EvStatusSpec> {
        NcB13s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b13s8`` event"]
    #[inline(always)]
    pub fn nc_b13s8(&mut self) -> NcB13s8W<'_, EvStatusSpec> {
        NcB13s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b13s9`` event"]
    #[inline(always)]
    pub fn nc_b13s9(&mut self) -> NcB13s9W<'_, EvStatusSpec> {
        NcB13s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b13s10`` event"]
    #[inline(always)]
    pub fn nc_b13s10(&mut self) -> NcB13s10W<'_, EvStatusSpec> {
        NcB13s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b13s11`` event"]
    #[inline(always)]
    pub fn nc_b13s11(&mut self) -> NcB13s11W<'_, EvStatusSpec> {
        NcB13s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b13s12`` event"]
    #[inline(always)]
    pub fn nc_b13s12(&mut self) -> NcB13s12W<'_, EvStatusSpec> {
        NcB13s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b13s13`` event"]
    #[inline(always)]
    pub fn nc_b13s13(&mut self) -> NcB13s13W<'_, EvStatusSpec> {
        NcB13s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b13s14`` event"]
    #[inline(always)]
    pub fn nc_b13s14(&mut self) -> NcB13s14W<'_, EvStatusSpec> {
        NcB13s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b13s15`` event"]
    #[inline(always)]
    pub fn nc_b13s15(&mut self) -> NcB13s15W<'_, EvStatusSpec> {
        NcB13s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b13s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
