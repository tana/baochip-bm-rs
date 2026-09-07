#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `ioxirq` reader - `1` when a \"ioxirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type IoxirqR = crate::BitReader;
#[doc = "Field `ioxirq` writer - `1` when a \"ioxirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type IoxirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbc` reader - `1` when a \"usbc\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type UsbcR = crate::BitReader;
#[doc = "Field `usbc` writer - `1` when a \"usbc\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type UsbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sddcirq` reader - `1` when a \"sddcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SddcirqR = crate::BitReader;
#[doc = "Field `sddcirq` writer - `1` when a \"sddcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SddcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0` reader - `1` when a \"pioirq0\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq0R = crate::BitReader;
#[doc = "Field `pioirq0` writer - `1` when a \"pioirq0\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1` reader - `1` when a \"pioirq1\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq1R = crate::BitReader;
#[doc = "Field `pioirq1` writer - `1` when a \"pioirq1\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2` reader - `1` when a \"pioirq2\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq2R = crate::BitReader;
#[doc = "Field `pioirq2` writer - `1` when a \"pioirq2\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3` reader - `1` when a \"pioirq3\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq3R = crate::BitReader;
#[doc = "Field `pioirq3` writer - `1` when a \"pioirq3\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s7` reader - `1` when a \"nc_b10s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s7R = crate::BitReader;
#[doc = "Field `nc_b10s7` writer - `1` when a \"nc_b10s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s8` reader - `1` when a \"nc_b10s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s8R = crate::BitReader;
#[doc = "Field `nc_b10s8` writer - `1` when a \"nc_b10s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s9` reader - `1` when a \"nc_b10s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s9R = crate::BitReader;
#[doc = "Field `nc_b10s9` writer - `1` when a \"nc_b10s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s10` reader - `1` when a \"nc_b10s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s10R = crate::BitReader;
#[doc = "Field `nc_b10s10` writer - `1` when a \"nc_b10s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s11` reader - `1` when a \"nc_b10s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s11R = crate::BitReader;
#[doc = "Field `nc_b10s11` writer - `1` when a \"nc_b10s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s12` reader - `1` when a \"nc_b10s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s12R = crate::BitReader;
#[doc = "Field `nc_b10s12` writer - `1` when a \"nc_b10s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s13` reader - `1` when a \"nc_b10s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s13R = crate::BitReader;
#[doc = "Field `nc_b10s13` writer - `1` when a \"nc_b10s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s14` reader - `1` when a \"nc_b10s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s14R = crate::BitReader;
#[doc = "Field `nc_b10s14` writer - `1` when a \"nc_b10s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b10s15` reader - `1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s15R = crate::BitReader;
#[doc = "Field `nc_b10s15` writer - `1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB10s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"ioxirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn ioxirq(&self) -> IoxirqR {
        IoxirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"usbc\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn usbc(&self) -> UsbcR {
        UsbcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"sddcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sddcirq(&self) -> SddcirqR {
        SddcirqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"pioirq0\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq0(&self) -> Pioirq0R {
        Pioirq0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"pioirq1\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq1(&self) -> Pioirq1R {
        Pioirq1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"pioirq2\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq2(&self) -> Pioirq2R {
        Pioirq2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"pioirq3\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq3(&self) -> Pioirq3R {
        Pioirq3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b10s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s7(&self) -> NcB10s7R {
        NcB10s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b10s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s8(&self) -> NcB10s8R {
        NcB10s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b10s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s9(&self) -> NcB10s9R {
        NcB10s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b10s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s10(&self) -> NcB10s10R {
        NcB10s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b10s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s11(&self) -> NcB10s11R {
        NcB10s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b10s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s12(&self) -> NcB10s12R {
        NcB10s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b10s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s13(&self) -> NcB10s13R {
        NcB10s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b10s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s14(&self) -> NcB10s14R {
        NcB10s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s15(&self) -> NcB10s15R {
        NcB10s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"ioxirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn ioxirq(&mut self) -> IoxirqW<'_, EvPendingSpec> {
        IoxirqW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"usbc\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn usbc(&mut self) -> UsbcW<'_, EvPendingSpec> {
        UsbcW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"sddcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sddcirq(&mut self) -> SddcirqW<'_, EvPendingSpec> {
        SddcirqW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"pioirq0\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq0(&mut self) -> Pioirq0W<'_, EvPendingSpec> {
        Pioirq0W::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"pioirq1\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq1(&mut self) -> Pioirq1W<'_, EvPendingSpec> {
        Pioirq1W::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"pioirq2\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq2(&mut self) -> Pioirq2W<'_, EvPendingSpec> {
        Pioirq2W::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"pioirq3\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq3(&mut self) -> Pioirq3W<'_, EvPendingSpec> {
        Pioirq3W::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b10s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s7(&mut self) -> NcB10s7W<'_, EvPendingSpec> {
        NcB10s7W::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b10s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s8(&mut self) -> NcB10s8W<'_, EvPendingSpec> {
        NcB10s8W::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b10s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s9(&mut self) -> NcB10s9W<'_, EvPendingSpec> {
        NcB10s9W::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b10s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s10(&mut self) -> NcB10s10W<'_, EvPendingSpec> {
        NcB10s10W::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b10s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s11(&mut self) -> NcB10s11W<'_, EvPendingSpec> {
        NcB10s11W::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b10s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s12(&mut self) -> NcB10s12W<'_, EvPendingSpec> {
        NcB10s12W::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b10s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s13(&mut self) -> NcB10s13W<'_, EvPendingSpec> {
        NcB10s13W::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b10s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s14(&mut self) -> NcB10s14W<'_, EvPendingSpec> {
        NcB10s14W::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b10s15(&mut self) -> NcB10s15W<'_, EvPendingSpec> {
        NcB10s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b10s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPendingSpec;
impl crate::RegisterSpec for EvPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EvPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EvPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EvPendingSpec {}
