#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `available` reader - Triggers when the `done` signal was asserted by the corresponding peer"]
pub type AvailableR = crate::BitReader;
#[doc = "Field `available` writer - Triggers when the `done` signal was asserted by the corresponding peer"]
pub type AvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_init` reader - Triggers when abort is asserted by the peer, and there is currently no abort in progress"]
pub type AbortInitR = crate::BitReader;
#[doc = "Field `abort_init` writer - Triggers when abort is asserted by the peer, and there is currently no abort in progress"]
pub type AbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_done` reader - Triggers when a previously initiated abort is acknowledged by peer"]
pub type AbortDoneR = crate::BitReader;
#[doc = "Field `abort_done` writer - Triggers when a previously initiated abort is acknowledged by peer"]
pub type AbortDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `error` reader - Triggers if either `tx_err` or `rx_err` are asserted"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `error` writer - Triggers if either `tx_err` or `rx_err` are asserted"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Triggers when the `done` signal was asserted by the corresponding peer"]
    #[inline(always)]
    pub fn available(&self) -> AvailableR {
        AvailableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggers when abort is asserted by the peer, and there is currently no abort in progress"]
    #[inline(always)]
    pub fn abort_init(&self) -> AbortInitR {
        AbortInitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Triggers when a previously initiated abort is acknowledged by peer"]
    #[inline(always)]
    pub fn abort_done(&self) -> AbortDoneR {
        AbortDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Triggers if either `tx_err` or `rx_err` are asserted"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggers when the `done` signal was asserted by the corresponding peer"]
    #[inline(always)]
    pub fn available(&mut self) -> AvailableW<'_, EvPendingSpec> {
        AvailableW::new(self, 0)
    }
    #[doc = "Bit 1 - Triggers when abort is asserted by the peer, and there is currently no abort in progress"]
    #[inline(always)]
    pub fn abort_init(&mut self) -> AbortInitW<'_, EvPendingSpec> {
        AbortInitW::new(self, 1)
    }
    #[doc = "Bit 2 - Triggers when a previously initiated abort is acknowledged by peer"]
    #[inline(always)]
    pub fn abort_done(&mut self) -> AbortDoneW<'_, EvPendingSpec> {
        AbortDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Triggers if either `tx_err` or `rx_err` are asserted"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, EvPendingSpec> {
        ErrorW::new(self, 3)
    }
}
#[doc = "Triggers if either `tx_err` or `rx_err` are asserted\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
