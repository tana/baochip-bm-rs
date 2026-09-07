#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `ioxirq` reader - Write a ``1`` to enable the ``ioxirq`` Event"]
pub type IoxirqR = crate::BitReader;
#[doc = "Field `ioxirq` writer - Write a ``1`` to enable the ``ioxirq`` Event"]
pub type IoxirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbc` reader - Write a ``1`` to enable the ``usbc`` Event"]
pub type UsbcR = crate::BitReader;
#[doc = "Field `usbc` writer - Write a ``1`` to enable the ``usbc`` Event"]
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sddcirq` reader - Write a ``1`` to enable the ``sddcirq`` Event"]
pub type SddcirqR = crate::BitReader;
#[doc = "Field `sddcirq` writer - Write a ``1`` to enable the ``sddcirq`` Event"]
pub type SddcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0` reader - Write a ``1`` to enable the ``pioirq0`` Event"]
pub type Pioirq0R = crate::BitReader;
#[doc = "Field `pioirq0` writer - Write a ``1`` to enable the ``pioirq0`` Event"]
pub type Pioirq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1` reader - Write a ``1`` to enable the ``pioirq1`` Event"]
pub type Pioirq1R = crate::BitReader;
#[doc = "Field `pioirq1` writer - Write a ``1`` to enable the ``pioirq1`` Event"]
pub type Pioirq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2` reader - Write a ``1`` to enable the ``pioirq2`` Event"]
pub type Pioirq2R = crate::BitReader;
#[doc = "Field `pioirq2` writer - Write a ``1`` to enable the ``pioirq2`` Event"]
pub type Pioirq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3` reader - Write a ``1`` to enable the ``pioirq3`` Event"]
pub type Pioirq3R = crate::BitReader;
#[doc = "Field `pioirq3` writer - Write a ``1`` to enable the ``pioirq3`` Event"]
pub type Pioirq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s7` reader - Write a ``1`` to enable the ``nc_b10s7`` Event"]
pub type NcB10s7R = crate::BitReader;
#[doc = "Field `nc_b10s7` writer - Write a ``1`` to enable the ``nc_b10s7`` Event"]
pub type NcB10s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s8` reader - Write a ``1`` to enable the ``nc_b10s8`` Event"]
pub type NcB10s8R = crate::BitReader;
#[doc = "Field `nc_b10s8` writer - Write a ``1`` to enable the ``nc_b10s8`` Event"]
pub type NcB10s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s9` reader - Write a ``1`` to enable the ``nc_b10s9`` Event"]
pub type NcB10s9R = crate::BitReader;
#[doc = "Field `nc_b10s9` writer - Write a ``1`` to enable the ``nc_b10s9`` Event"]
pub type NcB10s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s10` reader - Write a ``1`` to enable the ``nc_b10s10`` Event"]
pub type NcB10s10R = crate::BitReader;
#[doc = "Field `nc_b10s10` writer - Write a ``1`` to enable the ``nc_b10s10`` Event"]
pub type NcB10s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s11` reader - Write a ``1`` to enable the ``nc_b10s11`` Event"]
pub type NcB10s11R = crate::BitReader;
#[doc = "Field `nc_b10s11` writer - Write a ``1`` to enable the ``nc_b10s11`` Event"]
pub type NcB10s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s12` reader - Write a ``1`` to enable the ``nc_b10s12`` Event"]
pub type NcB10s12R = crate::BitReader;
#[doc = "Field `nc_b10s12` writer - Write a ``1`` to enable the ``nc_b10s12`` Event"]
pub type NcB10s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s13` reader - Write a ``1`` to enable the ``nc_b10s13`` Event"]
pub type NcB10s13R = crate::BitReader;
#[doc = "Field `nc_b10s13` writer - Write a ``1`` to enable the ``nc_b10s13`` Event"]
pub type NcB10s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s14` reader - Write a ``1`` to enable the ``nc_b10s14`` Event"]
pub type NcB10s14R = crate::BitReader;
#[doc = "Field `nc_b10s14` writer - Write a ``1`` to enable the ``nc_b10s14`` Event"]
pub type NcB10s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s15` reader - Write a ``1`` to enable the ``nc_b10s15`` Event"]
pub type NcB10s15R = crate::BitReader;
#[doc = "Field `nc_b10s15` writer - Write a ``1`` to enable the ``nc_b10s15`` Event"]
pub type NcB10s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``ioxirq`` Event"]
    #[inline(always)]
    pub fn ioxirq(&self) -> IoxirqR {
        IoxirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``usbc`` Event"]
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``sddcirq`` Event"]
    #[inline(always)]
    pub fn sddcirq(&self) -> SddcirqR {
        SddcirqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``pioirq0`` Event"]
    #[inline(always)]
    pub fn pioirq0(&self) -> Pioirq0R {
        Pioirq0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``pioirq1`` Event"]
    #[inline(always)]
    pub fn pioirq1(&self) -> Pioirq1R {
        Pioirq1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``pioirq2`` Event"]
    #[inline(always)]
    pub fn pioirq2(&self) -> Pioirq2R {
        Pioirq2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``pioirq3`` Event"]
    #[inline(always)]
    pub fn pioirq3(&self) -> Pioirq3R {
        Pioirq3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b10s7`` Event"]
    #[inline(always)]
    pub fn nc_b10s7(&self) -> NcB10s7R {
        NcB10s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b10s8`` Event"]
    #[inline(always)]
    pub fn nc_b10s8(&self) -> NcB10s8R {
        NcB10s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b10s9`` Event"]
    #[inline(always)]
    pub fn nc_b10s9(&self) -> NcB10s9R {
        NcB10s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b10s10`` Event"]
    #[inline(always)]
    pub fn nc_b10s10(&self) -> NcB10s10R {
        NcB10s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b10s11`` Event"]
    #[inline(always)]
    pub fn nc_b10s11(&self) -> NcB10s11R {
        NcB10s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b10s12`` Event"]
    #[inline(always)]
    pub fn nc_b10s12(&self) -> NcB10s12R {
        NcB10s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b10s13`` Event"]
    #[inline(always)]
    pub fn nc_b10s13(&self) -> NcB10s13R {
        NcB10s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b10s14`` Event"]
    #[inline(always)]
    pub fn nc_b10s14(&self) -> NcB10s14R {
        NcB10s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b10s15`` Event"]
    #[inline(always)]
    pub fn nc_b10s15(&self) -> NcB10s15R {
        NcB10s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``ioxirq`` Event"]
    #[inline(always)]
    pub fn ioxirq(&mut self) -> IoxirqW<'_, EvEnableSpec> {
        IoxirqW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``usbc`` Event"]
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<'_, EvEnableSpec> {
        UsbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``sddcirq`` Event"]
    #[inline(always)]
    pub fn sddcirq(&mut self) -> SddcirqW<'_, EvEnableSpec> {
        SddcirqW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``pioirq0`` Event"]
    #[inline(always)]
    pub fn pioirq0(&mut self) -> Pioirq0W<'_, EvEnableSpec> {
        Pioirq0W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``pioirq1`` Event"]
    #[inline(always)]
    pub fn pioirq1(&mut self) -> Pioirq1W<'_, EvEnableSpec> {
        Pioirq1W::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``pioirq2`` Event"]
    #[inline(always)]
    pub fn pioirq2(&mut self) -> Pioirq2W<'_, EvEnableSpec> {
        Pioirq2W::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``pioirq3`` Event"]
    #[inline(always)]
    pub fn pioirq3(&mut self) -> Pioirq3W<'_, EvEnableSpec> {
        Pioirq3W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b10s7`` Event"]
    #[inline(always)]
    pub fn nc_b10s7(&mut self) -> NcB10s7W<'_, EvEnableSpec> {
        NcB10s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b10s8`` Event"]
    #[inline(always)]
    pub fn nc_b10s8(&mut self) -> NcB10s8W<'_, EvEnableSpec> {
        NcB10s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b10s9`` Event"]
    #[inline(always)]
    pub fn nc_b10s9(&mut self) -> NcB10s9W<'_, EvEnableSpec> {
        NcB10s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b10s10`` Event"]
    #[inline(always)]
    pub fn nc_b10s10(&mut self) -> NcB10s10W<'_, EvEnableSpec> {
        NcB10s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b10s11`` Event"]
    #[inline(always)]
    pub fn nc_b10s11(&mut self) -> NcB10s11W<'_, EvEnableSpec> {
        NcB10s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b10s12`` Event"]
    #[inline(always)]
    pub fn nc_b10s12(&mut self) -> NcB10s12W<'_, EvEnableSpec> {
        NcB10s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b10s13`` Event"]
    #[inline(always)]
    pub fn nc_b10s13(&mut self) -> NcB10s13W<'_, EvEnableSpec> {
        NcB10s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b10s14`` Event"]
    #[inline(always)]
    pub fn nc_b10s14(&mut self) -> NcB10s14W<'_, EvEnableSpec> {
        NcB10s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b10s15`` Event"]
    #[inline(always)]
    pub fn nc_b10s15(&mut self) -> NcB10s15W<'_, EvEnableSpec> {
        NcB10s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
