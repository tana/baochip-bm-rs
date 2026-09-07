#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `rx_avail` reader - Rx data is available"]
pub type RxAvailR = crate::BitReader;
#[doc = "Field `rx_avail` writer - Rx data is available"]
pub type RxAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_free` reader - Tx register can be written"]
pub type TxFreeR = crate::BitReader;
#[doc = "Field `tx_free` writer - Tx register can be written"]
pub type TxFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_in_progress` reader - This bit is set if an `aborting` event was initiated and is still in progress."]
pub type AbortInProgressR = crate::BitReader;
#[doc = "Field `abort_in_progress` writer - This bit is set if an `aborting` event was initiated and is still in progress."]
pub type AbortInProgressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_ack` reader - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
pub type AbortAckR = crate::BitReader;
#[doc = "Field `abort_ack` writer - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
pub type AbortAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_err` reader - Set if the recipient was not ready for the data. Cleared on read."]
pub type TxErrR = crate::BitReader;
#[doc = "Field `tx_err` writer - Set if the recipient was not ready for the data. Cleared on read."]
pub type TxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_err` reader - Set if the recipient didn't have data available for a read. Cleared on read."]
pub type RxErrR = crate::BitReader;
#[doc = "Field `rx_err` writer - Set if the recipient didn't have data available for a read. Cleared on read."]
pub type RxErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx data is available"]
    #[inline(always)]
    pub fn rx_avail(&self) -> RxAvailR {
        RxAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx register can be written"]
    #[inline(always)]
    pub fn tx_free(&self) -> TxFreeR {
        TxFreeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set if an `aborting` event was initiated and is still in progress."]
    #[inline(always)]
    pub fn abort_in_progress(&self) -> AbortInProgressR {
        AbortInProgressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
    #[inline(always)]
    pub fn abort_ack(&self) -> AbortAckR {
        AbortAckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set if the recipient was not ready for the data. Cleared on read."]
    #[inline(always)]
    pub fn tx_err(&self) -> TxErrR {
        TxErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set if the recipient didn't have data available for a read. Cleared on read."]
    #[inline(always)]
    pub fn rx_err(&self) -> RxErrR {
        RxErrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx data is available"]
    #[inline(always)]
    pub fn rx_avail(&mut self) -> RxAvailW<'_, StatusSpec> {
        RxAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx register can be written"]
    #[inline(always)]
    pub fn tx_free(&mut self) -> TxFreeW<'_, StatusSpec> {
        TxFreeW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is set if an `aborting` event was initiated and is still in progress."]
    #[inline(always)]
    pub fn abort_in_progress(&mut self) -> AbortInProgressW<'_, StatusSpec> {
        AbortInProgressW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
    #[inline(always)]
    pub fn abort_ack(&mut self) -> AbortAckW<'_, StatusSpec> {
        AbortAckW::new(self, 3)
    }
    #[doc = "Bit 4 - Set if the recipient was not ready for the data. Cleared on read."]
    #[inline(always)]
    pub fn tx_err(&mut self) -> TxErrW<'_, StatusSpec> {
        TxErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Set if the recipient didn't have data available for a read. Cleared on read."]
    #[inline(always)]
    pub fn rx_err(&mut self) -> RxErrW<'_, StatusSpec> {
        RxErrW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
