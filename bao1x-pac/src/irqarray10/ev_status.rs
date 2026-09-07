#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `ioxirq` reader - Level of the ``ioxirq`` event"]
pub type IoxirqR = crate::BitReader;
#[doc = "Field `ioxirq` writer - Level of the ``ioxirq`` event"]
pub type IoxirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbc` reader - Level of the ``usbc`` event"]
pub type UsbcR = crate::BitReader;
#[doc = "Field `usbc` writer - Level of the ``usbc`` event"]
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sddcirq` reader - Level of the ``sddcirq`` event"]
pub type SddcirqR = crate::BitReader;
#[doc = "Field `sddcirq` writer - Level of the ``sddcirq`` event"]
pub type SddcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0` reader - Level of the ``pioirq0`` event"]
pub type Pioirq0R = crate::BitReader;
#[doc = "Field `pioirq0` writer - Level of the ``pioirq0`` event"]
pub type Pioirq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1` reader - Level of the ``pioirq1`` event"]
pub type Pioirq1R = crate::BitReader;
#[doc = "Field `pioirq1` writer - Level of the ``pioirq1`` event"]
pub type Pioirq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2` reader - Level of the ``pioirq2`` event"]
pub type Pioirq2R = crate::BitReader;
#[doc = "Field `pioirq2` writer - Level of the ``pioirq2`` event"]
pub type Pioirq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3` reader - Level of the ``pioirq3`` event"]
pub type Pioirq3R = crate::BitReader;
#[doc = "Field `pioirq3` writer - Level of the ``pioirq3`` event"]
pub type Pioirq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s7` reader - Level of the ``nc_b10s7`` event"]
pub type NcB10s7R = crate::BitReader;
#[doc = "Field `nc_b10s7` writer - Level of the ``nc_b10s7`` event"]
pub type NcB10s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s8` reader - Level of the ``nc_b10s8`` event"]
pub type NcB10s8R = crate::BitReader;
#[doc = "Field `nc_b10s8` writer - Level of the ``nc_b10s8`` event"]
pub type NcB10s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s9` reader - Level of the ``nc_b10s9`` event"]
pub type NcB10s9R = crate::BitReader;
#[doc = "Field `nc_b10s9` writer - Level of the ``nc_b10s9`` event"]
pub type NcB10s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s10` reader - Level of the ``nc_b10s10`` event"]
pub type NcB10s10R = crate::BitReader;
#[doc = "Field `nc_b10s10` writer - Level of the ``nc_b10s10`` event"]
pub type NcB10s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s11` reader - Level of the ``nc_b10s11`` event"]
pub type NcB10s11R = crate::BitReader;
#[doc = "Field `nc_b10s11` writer - Level of the ``nc_b10s11`` event"]
pub type NcB10s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s12` reader - Level of the ``nc_b10s12`` event"]
pub type NcB10s12R = crate::BitReader;
#[doc = "Field `nc_b10s12` writer - Level of the ``nc_b10s12`` event"]
pub type NcB10s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s13` reader - Level of the ``nc_b10s13`` event"]
pub type NcB10s13R = crate::BitReader;
#[doc = "Field `nc_b10s13` writer - Level of the ``nc_b10s13`` event"]
pub type NcB10s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s14` reader - Level of the ``nc_b10s14`` event"]
pub type NcB10s14R = crate::BitReader;
#[doc = "Field `nc_b10s14` writer - Level of the ``nc_b10s14`` event"]
pub type NcB10s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s15` reader - Level of the ``nc_b10s15`` event"]
pub type NcB10s15R = crate::BitReader;
#[doc = "Field `nc_b10s15` writer - Level of the ``nc_b10s15`` event"]
pub type NcB10s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``ioxirq`` event"]
    #[inline(always)]
    pub fn ioxirq(&self) -> IoxirqR {
        IoxirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``usbc`` event"]
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``sddcirq`` event"]
    #[inline(always)]
    pub fn sddcirq(&self) -> SddcirqR {
        SddcirqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``pioirq0`` event"]
    #[inline(always)]
    pub fn pioirq0(&self) -> Pioirq0R {
        Pioirq0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``pioirq1`` event"]
    #[inline(always)]
    pub fn pioirq1(&self) -> Pioirq1R {
        Pioirq1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``pioirq2`` event"]
    #[inline(always)]
    pub fn pioirq2(&self) -> Pioirq2R {
        Pioirq2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``pioirq3`` event"]
    #[inline(always)]
    pub fn pioirq3(&self) -> Pioirq3R {
        Pioirq3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b10s7`` event"]
    #[inline(always)]
    pub fn nc_b10s7(&self) -> NcB10s7R {
        NcB10s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b10s8`` event"]
    #[inline(always)]
    pub fn nc_b10s8(&self) -> NcB10s8R {
        NcB10s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b10s9`` event"]
    #[inline(always)]
    pub fn nc_b10s9(&self) -> NcB10s9R {
        NcB10s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b10s10`` event"]
    #[inline(always)]
    pub fn nc_b10s10(&self) -> NcB10s10R {
        NcB10s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b10s11`` event"]
    #[inline(always)]
    pub fn nc_b10s11(&self) -> NcB10s11R {
        NcB10s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b10s12`` event"]
    #[inline(always)]
    pub fn nc_b10s12(&self) -> NcB10s12R {
        NcB10s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b10s13`` event"]
    #[inline(always)]
    pub fn nc_b10s13(&self) -> NcB10s13R {
        NcB10s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b10s14`` event"]
    #[inline(always)]
    pub fn nc_b10s14(&self) -> NcB10s14R {
        NcB10s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b10s15`` event"]
    #[inline(always)]
    pub fn nc_b10s15(&self) -> NcB10s15R {
        NcB10s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``ioxirq`` event"]
    #[inline(always)]
    pub fn ioxirq(&mut self) -> IoxirqW<'_, EvStatusSpec> {
        IoxirqW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``usbc`` event"]
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<'_, EvStatusSpec> {
        UsbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``sddcirq`` event"]
    #[inline(always)]
    pub fn sddcirq(&mut self) -> SddcirqW<'_, EvStatusSpec> {
        SddcirqW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``pioirq0`` event"]
    #[inline(always)]
    pub fn pioirq0(&mut self) -> Pioirq0W<'_, EvStatusSpec> {
        Pioirq0W::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``pioirq1`` event"]
    #[inline(always)]
    pub fn pioirq1(&mut self) -> Pioirq1W<'_, EvStatusSpec> {
        Pioirq1W::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``pioirq2`` event"]
    #[inline(always)]
    pub fn pioirq2(&mut self) -> Pioirq2W<'_, EvStatusSpec> {
        Pioirq2W::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``pioirq3`` event"]
    #[inline(always)]
    pub fn pioirq3(&mut self) -> Pioirq3W<'_, EvStatusSpec> {
        Pioirq3W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b10s7`` event"]
    #[inline(always)]
    pub fn nc_b10s7(&mut self) -> NcB10s7W<'_, EvStatusSpec> {
        NcB10s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b10s8`` event"]
    #[inline(always)]
    pub fn nc_b10s8(&mut self) -> NcB10s8W<'_, EvStatusSpec> {
        NcB10s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b10s9`` event"]
    #[inline(always)]
    pub fn nc_b10s9(&mut self) -> NcB10s9W<'_, EvStatusSpec> {
        NcB10s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b10s10`` event"]
    #[inline(always)]
    pub fn nc_b10s10(&mut self) -> NcB10s10W<'_, EvStatusSpec> {
        NcB10s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b10s11`` event"]
    #[inline(always)]
    pub fn nc_b10s11(&mut self) -> NcB10s11W<'_, EvStatusSpec> {
        NcB10s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b10s12`` event"]
    #[inline(always)]
    pub fn nc_b10s12(&mut self) -> NcB10s12W<'_, EvStatusSpec> {
        NcB10s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b10s13`` event"]
    #[inline(always)]
    pub fn nc_b10s13(&mut self) -> NcB10s13W<'_, EvStatusSpec> {
        NcB10s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b10s14`` event"]
    #[inline(always)]
    pub fn nc_b10s14(&mut self) -> NcB10s14W<'_, EvStatusSpec> {
        NcB10s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b10s15`` event"]
    #[inline(always)]
    pub fn nc_b10s15(&mut self) -> NcB10s15W<'_, EvStatusSpec> {
        NcB10s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
