#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `spim0_rx` reader - `1` when a \"spim0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0RxR = crate::BitReader;
#[doc = "Field `spim0_rx` writer - `1` when a \"spim0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_tx` reader - `1` when a \"spim0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0TxR = crate::BitReader;
#[doc = "Field `spim0_tx` writer - `1` when a \"spim0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_cmd` reader - `1` when a \"spim0_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0CmdR = crate::BitReader;
#[doc = "Field `spim0_cmd` writer - `1` when a \"spim0_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim0_eot` reader - `1` when a \"spim0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0EotR = crate::BitReader;
#[doc = "Field `spim0_eot` writer - `1` when a \"spim0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim0EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_rx` reader - `1` when a \"spim1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1RxR = crate::BitReader;
#[doc = "Field `spim1_rx` writer - `1` when a \"spim1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_tx` reader - `1` when a \"spim1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1TxR = crate::BitReader;
#[doc = "Field `spim1_tx` writer - `1` when a \"spim1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_cmd` reader - `1` when a \"spim1_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1CmdR = crate::BitReader;
#[doc = "Field `spim1_cmd` writer - `1` when a \"spim1_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_eot` reader - `1` when a \"spim1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1EotR = crate::BitReader;
#[doc = "Field `spim1_eot` writer - `1` when a \"spim1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim1EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_rx` reader - `1` when a \"spim2_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2RxR = crate::BitReader;
#[doc = "Field `spim2_rx` writer - `1` when a \"spim2_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_tx` reader - `1` when a \"spim2_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2TxR = crate::BitReader;
#[doc = "Field `spim2_tx` writer - `1` when a \"spim2_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_cmd` reader - `1` when a \"spim2_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2CmdR = crate::BitReader;
#[doc = "Field `spim2_cmd` writer - `1` when a \"spim2_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_eot` reader - `1` when a \"spim2_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2EotR = crate::BitReader;
#[doc = "Field `spim2_eot` writer - `1` when a \"spim2_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim2EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_rx` reader - `1` when a \"spim3_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3RxR = crate::BitReader;
#[doc = "Field `spim3_rx` writer - `1` when a \"spim3_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_tx` reader - `1` when a \"spim3_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3TxR = crate::BitReader;
#[doc = "Field `spim3_tx` writer - `1` when a \"spim3_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_cmd` reader - `1` when a \"spim3_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3CmdR = crate::BitReader;
#[doc = "Field `spim3_cmd` writer - `1` when a \"spim3_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3CmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim3_eot` reader - `1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3EotR = crate::BitReader;
#[doc = "Field `spim3_eot` writer - `1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Spim3EotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"spim0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_rx(&self) -> Spim0RxR {
        Spim0RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"spim0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_tx(&self) -> Spim0TxR {
        Spim0TxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"spim0_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_cmd(&self) -> Spim0CmdR {
        Spim0CmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"spim0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_eot(&self) -> Spim0EotR {
        Spim0EotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"spim1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_rx(&self) -> Spim1RxR {
        Spim1RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"spim1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_tx(&self) -> Spim1TxR {
        Spim1TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"spim1_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_cmd(&self) -> Spim1CmdR {
        Spim1CmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"spim1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_eot(&self) -> Spim1EotR {
        Spim1EotR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"spim2_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_rx(&self) -> Spim2RxR {
        Spim2RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"spim2_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_tx(&self) -> Spim2TxR {
        Spim2TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"spim2_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_cmd(&self) -> Spim2CmdR {
        Spim2CmdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"spim2_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_eot(&self) -> Spim2EotR {
        Spim2EotR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"spim3_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_rx(&self) -> Spim3RxR {
        Spim3RxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"spim3_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_tx(&self) -> Spim3TxR {
        Spim3TxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"spim3_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_cmd(&self) -> Spim3CmdR {
        Spim3CmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_eot(&self) -> Spim3EotR {
        Spim3EotR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"spim0_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_rx(&mut self) -> Spim0RxW<'_, EvPendingSpec> {
        Spim0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"spim0_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_tx(&mut self) -> Spim0TxW<'_, EvPendingSpec> {
        Spim0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"spim0_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_cmd(&mut self) -> Spim0CmdW<'_, EvPendingSpec> {
        Spim0CmdW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"spim0_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim0_eot(&mut self) -> Spim0EotW<'_, EvPendingSpec> {
        Spim0EotW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"spim1_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_rx(&mut self) -> Spim1RxW<'_, EvPendingSpec> {
        Spim1RxW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"spim1_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_tx(&mut self) -> Spim1TxW<'_, EvPendingSpec> {
        Spim1TxW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"spim1_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_cmd(&mut self) -> Spim1CmdW<'_, EvPendingSpec> {
        Spim1CmdW::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"spim1_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim1_eot(&mut self) -> Spim1EotW<'_, EvPendingSpec> {
        Spim1EotW::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"spim2_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_rx(&mut self) -> Spim2RxW<'_, EvPendingSpec> {
        Spim2RxW::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"spim2_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_tx(&mut self) -> Spim2TxW<'_, EvPendingSpec> {
        Spim2TxW::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"spim2_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_cmd(&mut self) -> Spim2CmdW<'_, EvPendingSpec> {
        Spim2CmdW::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"spim2_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim2_eot(&mut self) -> Spim2EotW<'_, EvPendingSpec> {
        Spim2EotW::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"spim3_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_rx(&mut self) -> Spim3RxW<'_, EvPendingSpec> {
        Spim3RxW::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"spim3_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_tx(&mut self) -> Spim3TxW<'_, EvPendingSpec> {
        Spim3TxW::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"spim3_cmd\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_cmd(&mut self) -> Spim3CmdW<'_, EvPendingSpec> {
        Spim3CmdW::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn spim3_eot(&mut self) -> Spim3EotW<'_, EvPendingSpec> {
        Spim3EotW::new(self, 15)
    }
}
#[doc = "`1` when a \"spim3_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
