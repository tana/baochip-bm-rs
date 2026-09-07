#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `rx_words` reader - Number of words available to read"]
pub type RxWordsR = crate::FieldReader<u16>;
#[doc = "Field `rx_words` writer - Number of words available to read"]
pub type RxWordsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `tx_words` reader - Number of words pending in write FIFO. Free space is 1024 - `tx_avail`"]
pub type TxWordsR = crate::FieldReader<u16>;
#[doc = "Field `tx_words` writer - Number of words pending in write FIFO. Free space is 1024 - `tx_avail`"]
pub type TxWordsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `abort_in_progress` reader - This bit is set if an `aborting` event was initiated and is still in progress."]
pub type AbortInProgressR = crate::BitReader;
#[doc = "Field `abort_in_progress` writer - This bit is set if an `aborting` event was initiated and is still in progress."]
pub type AbortInProgressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_ack` reader - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
pub type AbortAckR = crate::BitReader;
#[doc = "Field `abort_ack` writer - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
pub type AbortAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_err` reader - Set if the write FIFO overflowed because we wrote too much data. Cleared on register read."]
pub type TxErrR = crate::BitReader;
#[doc = "Field `tx_err` writer - Set if the write FIFO overflowed because we wrote too much data. Cleared on register read."]
pub type TxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_err` reader - Set if read FIFO underflowed because we read too much data. Cleared on register read."]
pub type RxErrR = crate::BitReader;
#[doc = "Field `rx_err` writer - Set if read FIFO underflowed because we read too much data. Cleared on register read."]
pub type RxErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Number of words available to read"]
    #[inline(always)]
    pub fn rx_words(&self) -> RxWordsR {
        RxWordsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - Number of words pending in write FIFO. Free space is 1024 - `tx_avail`"]
    #[inline(always)]
    pub fn tx_words(&self) -> TxWordsR {
        TxWordsR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - This bit is set if an `aborting` event was initiated and is still in progress."]
    #[inline(always)]
    pub fn abort_in_progress(&self) -> AbortInProgressR {
        AbortInProgressR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
    #[inline(always)]
    pub fn abort_ack(&self) -> AbortAckR {
        AbortAckR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set if the write FIFO overflowed because we wrote too much data. Cleared on register read."]
    #[inline(always)]
    pub fn tx_err(&self) -> TxErrR {
        TxErrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set if read FIFO underflowed because we read too much data. Cleared on register read."]
    #[inline(always)]
    pub fn rx_err(&self) -> RxErrR {
        RxErrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Number of words available to read"]
    #[inline(always)]
    pub fn rx_words(&mut self) -> RxWordsW<'_, StatusSpec> {
        RxWordsW::new(self, 0)
    }
    #[doc = "Bits 11:21 - Number of words pending in write FIFO. Free space is 1024 - `tx_avail`"]
    #[inline(always)]
    pub fn tx_words(&mut self) -> TxWordsW<'_, StatusSpec> {
        TxWordsW::new(self, 11)
    }
    #[doc = "Bit 22 - This bit is set if an `aborting` event was initiated and is still in progress."]
    #[inline(always)]
    pub fn abort_in_progress(&mut self) -> AbortInProgressW<'_, StatusSpec> {
        AbortInProgressW::new(self, 22)
    }
    #[doc = "Bit 23 - This bit is set by the peer that acknowledged the incoming abort (the later of the two, in case of an imperfect race condition). The abort response handler should check this bit; if it is set, no new acknowledgement shall be issued. The bit is cleared when an initiator initiates a new abort. The initiator shall also ignore the state of this bit if it is intending to initiate a new abort cycle."]
    #[inline(always)]
    pub fn abort_ack(&mut self) -> AbortAckW<'_, StatusSpec> {
        AbortAckW::new(self, 23)
    }
    #[doc = "Bit 24 - Set if the write FIFO overflowed because we wrote too much data. Cleared on register read."]
    #[inline(always)]
    pub fn tx_err(&mut self) -> TxErrW<'_, StatusSpec> {
        TxErrW::new(self, 24)
    }
    #[doc = "Bit 25 - Set if read FIFO underflowed because we read too much data. Cleared on register read."]
    #[inline(always)]
    pub fn rx_err(&mut self) -> RxErrW<'_, StatusSpec> {
        RxErrW::new(self, 25)
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
