#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `mdmairq_dupe` reader - Write a ``1`` to enable the ``mdmairq_dupe`` Event"]
pub type MdmairqDupeR = crate::BitReader;
#[doc = "Field `mdmairq_dupe` writer - Write a ``1`` to enable the ``mdmairq_dupe`` Event"]
pub type MdmairqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s1` reader - Write a ``1`` to enable the ``nc_b0s1`` Event"]
pub type NcB0s1R = crate::BitReader;
#[doc = "Field `nc_b0s1` writer - Write a ``1`` to enable the ``nc_b0s1`` Event"]
pub type NcB0s1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s2` reader - Write a ``1`` to enable the ``nc_b0s2`` Event"]
pub type NcB0s2R = crate::BitReader;
#[doc = "Field `nc_b0s2` writer - Write a ``1`` to enable the ``nc_b0s2`` Event"]
pub type NcB0s2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s3` reader - Write a ``1`` to enable the ``nc_b0s3`` Event"]
pub type NcB0s3R = crate::BitReader;
#[doc = "Field `nc_b0s3` writer - Write a ``1`` to enable the ``nc_b0s3`` Event"]
pub type NcB0s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0_dupe` reader - Write a ``1`` to enable the ``pioirq0_dupe`` Event"]
pub type Pioirq0DupeR = crate::BitReader;
#[doc = "Field `pioirq0_dupe` writer - Write a ``1`` to enable the ``pioirq0_dupe`` Event"]
pub type Pioirq0DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1_dupe` reader - Write a ``1`` to enable the ``pioirq1_dupe`` Event"]
pub type Pioirq1DupeR = crate::BitReader;
#[doc = "Field `pioirq1_dupe` writer - Write a ``1`` to enable the ``pioirq1_dupe`` Event"]
pub type Pioirq1DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2_dupe` reader - Write a ``1`` to enable the ``pioirq2_dupe`` Event"]
pub type Pioirq2DupeR = crate::BitReader;
#[doc = "Field `pioirq2_dupe` writer - Write a ``1`` to enable the ``pioirq2_dupe`` Event"]
pub type Pioirq2DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3_dupe` reader - Write a ``1`` to enable the ``pioirq3_dupe`` Event"]
pub type Pioirq3DupeR = crate::BitReader;
#[doc = "Field `pioirq3_dupe` writer - Write a ``1`` to enable the ``pioirq3_dupe`` Event"]
pub type Pioirq3DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s8` reader - Write a ``1`` to enable the ``nc_b0s8`` Event"]
pub type NcB0s8R = crate::BitReader;
#[doc = "Field `nc_b0s8` writer - Write a ``1`` to enable the ``nc_b0s8`` Event"]
pub type NcB0s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s9` reader - Write a ``1`` to enable the ``nc_b0s9`` Event"]
pub type NcB0s9R = crate::BitReader;
#[doc = "Field `nc_b0s9` writer - Write a ``1`` to enable the ``nc_b0s9`` Event"]
pub type NcB0s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s10` reader - Write a ``1`` to enable the ``nc_b0s10`` Event"]
pub type NcB0s10R = crate::BitReader;
#[doc = "Field `nc_b0s10` writer - Write a ``1`` to enable the ``nc_b0s10`` Event"]
pub type NcB0s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s11` reader - Write a ``1`` to enable the ``nc_b0s11`` Event"]
pub type NcB0s11R = crate::BitReader;
#[doc = "Field `nc_b0s11` writer - Write a ``1`` to enable the ``nc_b0s11`` Event"]
pub type NcB0s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s12` reader - Write a ``1`` to enable the ``nc_b0s12`` Event"]
pub type NcB0s12R = crate::BitReader;
#[doc = "Field `nc_b0s12` writer - Write a ``1`` to enable the ``nc_b0s12`` Event"]
pub type NcB0s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s13` reader - Write a ``1`` to enable the ``nc_b0s13`` Event"]
pub type NcB0s13R = crate::BitReader;
#[doc = "Field `nc_b0s13` writer - Write a ``1`` to enable the ``nc_b0s13`` Event"]
pub type NcB0s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s14` reader - Write a ``1`` to enable the ``nc_b0s14`` Event"]
pub type NcB0s14R = crate::BitReader;
#[doc = "Field `nc_b0s14` writer - Write a ``1`` to enable the ``nc_b0s14`` Event"]
pub type NcB0s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b0s15` reader - Write a ``1`` to enable the ``nc_b0s15`` Event"]
pub type NcB0s15R = crate::BitReader;
#[doc = "Field `nc_b0s15` writer - Write a ``1`` to enable the ``nc_b0s15`` Event"]
pub type NcB0s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``mdmairq_dupe`` Event"]
    #[inline(always)]
    pub fn mdmairq_dupe(&self) -> MdmairqDupeR {
        MdmairqDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b0s1`` Event"]
    #[inline(always)]
    pub fn nc_b0s1(&self) -> NcB0s1R {
        NcB0s1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b0s2`` Event"]
    #[inline(always)]
    pub fn nc_b0s2(&self) -> NcB0s2R {
        NcB0s2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b0s3`` Event"]
    #[inline(always)]
    pub fn nc_b0s3(&self) -> NcB0s3R {
        NcB0s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``pioirq0_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&self) -> Pioirq0DupeR {
        Pioirq0DupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``pioirq1_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&self) -> Pioirq1DupeR {
        Pioirq1DupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``pioirq2_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&self) -> Pioirq2DupeR {
        Pioirq2DupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``pioirq3_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&self) -> Pioirq3DupeR {
        Pioirq3DupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b0s8`` Event"]
    #[inline(always)]
    pub fn nc_b0s8(&self) -> NcB0s8R {
        NcB0s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b0s9`` Event"]
    #[inline(always)]
    pub fn nc_b0s9(&self) -> NcB0s9R {
        NcB0s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b0s10`` Event"]
    #[inline(always)]
    pub fn nc_b0s10(&self) -> NcB0s10R {
        NcB0s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b0s11`` Event"]
    #[inline(always)]
    pub fn nc_b0s11(&self) -> NcB0s11R {
        NcB0s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b0s12`` Event"]
    #[inline(always)]
    pub fn nc_b0s12(&self) -> NcB0s12R {
        NcB0s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b0s13`` Event"]
    #[inline(always)]
    pub fn nc_b0s13(&self) -> NcB0s13R {
        NcB0s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b0s14`` Event"]
    #[inline(always)]
    pub fn nc_b0s14(&self) -> NcB0s14R {
        NcB0s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b0s15`` Event"]
    #[inline(always)]
    pub fn nc_b0s15(&self) -> NcB0s15R {
        NcB0s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``mdmairq_dupe`` Event"]
    #[inline(always)]
    pub fn mdmairq_dupe(&mut self) -> MdmairqDupeW<'_, EvEnableSpec> {
        MdmairqDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``nc_b0s1`` Event"]
    #[inline(always)]
    pub fn nc_b0s1(&mut self) -> NcB0s1W<'_, EvEnableSpec> {
        NcB0s1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``nc_b0s2`` Event"]
    #[inline(always)]
    pub fn nc_b0s2(&mut self) -> NcB0s2W<'_, EvEnableSpec> {
        NcB0s2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b0s3`` Event"]
    #[inline(always)]
    pub fn nc_b0s3(&mut self) -> NcB0s3W<'_, EvEnableSpec> {
        NcB0s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``pioirq0_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&mut self) -> Pioirq0DupeW<'_, EvEnableSpec> {
        Pioirq0DupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``pioirq1_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&mut self) -> Pioirq1DupeW<'_, EvEnableSpec> {
        Pioirq1DupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``pioirq2_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&mut self) -> Pioirq2DupeW<'_, EvEnableSpec> {
        Pioirq2DupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``pioirq3_dupe`` Event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&mut self) -> Pioirq3DupeW<'_, EvEnableSpec> {
        Pioirq3DupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b0s8`` Event"]
    #[inline(always)]
    pub fn nc_b0s8(&mut self) -> NcB0s8W<'_, EvEnableSpec> {
        NcB0s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b0s9`` Event"]
    #[inline(always)]
    pub fn nc_b0s9(&mut self) -> NcB0s9W<'_, EvEnableSpec> {
        NcB0s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b0s10`` Event"]
    #[inline(always)]
    pub fn nc_b0s10(&mut self) -> NcB0s10W<'_, EvEnableSpec> {
        NcB0s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b0s11`` Event"]
    #[inline(always)]
    pub fn nc_b0s11(&mut self) -> NcB0s11W<'_, EvEnableSpec> {
        NcB0s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b0s12`` Event"]
    #[inline(always)]
    pub fn nc_b0s12(&mut self) -> NcB0s12W<'_, EvEnableSpec> {
        NcB0s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b0s13`` Event"]
    #[inline(always)]
    pub fn nc_b0s13(&mut self) -> NcB0s13W<'_, EvEnableSpec> {
        NcB0s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b0s14`` Event"]
    #[inline(always)]
    pub fn nc_b0s14(&mut self) -> NcB0s14W<'_, EvEnableSpec> {
        NcB0s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b0s15`` Event"]
    #[inline(always)]
    pub fn nc_b0s15(&mut self) -> NcB0s15W<'_, EvEnableSpec> {
        NcB0s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b0s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
