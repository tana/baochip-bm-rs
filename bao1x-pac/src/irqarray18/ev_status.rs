#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `pioirq0_dupe` reader - Level of the ``pioirq0_dupe`` event"]
pub type Pioirq0DupeR = crate::BitReader;
#[doc = "Field `pioirq0_dupe` writer - Level of the ``pioirq0_dupe`` event"]
pub type Pioirq0DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1_dupe` reader - Level of the ``pioirq1_dupe`` event"]
pub type Pioirq1DupeR = crate::BitReader;
#[doc = "Field `pioirq1_dupe` writer - Level of the ``pioirq1_dupe`` event"]
pub type Pioirq1DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2_dupe` reader - Level of the ``pioirq2_dupe`` event"]
pub type Pioirq2DupeR = crate::BitReader;
#[doc = "Field `pioirq2_dupe` writer - Level of the ``pioirq2_dupe`` event"]
pub type Pioirq2DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3_dupe` reader - Level of the ``pioirq3_dupe`` event"]
pub type Pioirq3DupeR = crate::BitReader;
#[doc = "Field `pioirq3_dupe` writer - Level of the ``pioirq3_dupe`` event"]
pub type Pioirq3DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_rx_dupe` reader - Level of the ``i2c2_rx_dupe`` event"]
pub type I2c2RxDupeR = crate::BitReader;
#[doc = "Field `i2c2_rx_dupe` writer - Level of the ``i2c2_rx_dupe`` event"]
pub type I2c2RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_tx_dupe` reader - Level of the ``i2c2_tx_dupe`` event"]
pub type I2c2TxDupeR = crate::BitReader;
#[doc = "Field `i2c2_tx_dupe` writer - Level of the ``i2c2_tx_dupe`` event"]
pub type I2c2TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_cmd_dupe` reader - Level of the ``i2c2_cmd_dupe`` event"]
pub type I2c2CmdDupeR = crate::BitReader;
#[doc = "Field `i2c2_cmd_dupe` writer - Level of the ``i2c2_cmd_dupe`` event"]
pub type I2c2CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_eot_dupe` reader - Level of the ``i2c2_eot_dupe`` event"]
pub type I2c2EotDupeR = crate::BitReader;
#[doc = "Field `i2c2_eot_dupe` writer - Level of the ``i2c2_eot_dupe`` event"]
pub type I2c2EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_nack_dupe` reader - Level of the ``i2c0_nack_dupe`` event"]
pub type I2c0NackDupeR = crate::BitReader;
#[doc = "Field `i2c0_nack_dupe` writer - Level of the ``i2c0_nack_dupe`` event"]
pub type I2c0NackDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_nack_dupe` reader - Level of the ``i2c1_nack_dupe`` event"]
pub type I2c1NackDupeR = crate::BitReader;
#[doc = "Field `i2c1_nack_dupe` writer - Level of the ``i2c1_nack_dupe`` event"]
pub type I2c1NackDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_nack_dupe` reader - Level of the ``i2c2_nack_dupe`` event"]
pub type I2c2NackDupeR = crate::BitReader;
#[doc = "Field `i2c2_nack_dupe` writer - Level of the ``i2c2_nack_dupe`` event"]
pub type I2c2NackDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_err_dupe` reader - Level of the ``i2c0_err_dupe`` event"]
pub type I2c0ErrDupeR = crate::BitReader;
#[doc = "Field `i2c0_err_dupe` writer - Level of the ``i2c0_err_dupe`` event"]
pub type I2c0ErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_err_dupe` reader - Level of the ``i2c1_err_dupe`` event"]
pub type I2c1ErrDupeR = crate::BitReader;
#[doc = "Field `i2c1_err_dupe` writer - Level of the ``i2c1_err_dupe`` event"]
pub type I2c1ErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c2_err_dupe` reader - Level of the ``i2c2_err_dupe`` event"]
pub type I2c2ErrDupeR = crate::BitReader;
#[doc = "Field `i2c2_err_dupe` writer - Level of the ``i2c2_err_dupe`` event"]
pub type I2c2ErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioxirq_dupe` reader - Level of the ``ioxirq_dupe`` event"]
pub type IoxirqDupeR = crate::BitReader;
#[doc = "Field `ioxirq_dupe` writer - Level of the ``ioxirq_dupe`` event"]
pub type IoxirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cam_rx_dupe` reader - Level of the ``cam_rx_dupe`` event"]
pub type CamRxDupeR = crate::BitReader;
#[doc = "Field `cam_rx_dupe` writer - Level of the ``cam_rx_dupe`` event"]
pub type CamRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&self) -> Pioirq0DupeR {
        Pioirq0DupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&self) -> Pioirq1DupeR {
        Pioirq1DupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&self) -> Pioirq2DupeR {
        Pioirq2DupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&self) -> Pioirq3DupeR {
        Pioirq3DupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``i2c2_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_rx_dupe(&self) -> I2c2RxDupeR {
        I2c2RxDupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``i2c2_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_tx_dupe(&self) -> I2c2TxDupeR {
        I2c2TxDupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``i2c2_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_cmd_dupe(&self) -> I2c2CmdDupeR {
        I2c2CmdDupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``i2c2_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_eot_dupe(&self) -> I2c2EotDupeR {
        I2c2EotDupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``i2c0_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_nack_dupe(&self) -> I2c0NackDupeR {
        I2c0NackDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``i2c1_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_nack_dupe(&self) -> I2c1NackDupeR {
        I2c1NackDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_nack_dupe(&self) -> I2c2NackDupeR {
        I2c2NackDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``i2c0_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_err_dupe(&self) -> I2c0ErrDupeR {
        I2c0ErrDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``i2c1_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_err_dupe(&self) -> I2c1ErrDupeR {
        I2c1ErrDupeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``i2c2_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_err_dupe(&self) -> I2c2ErrDupeR {
        I2c2ErrDupeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``ioxirq_dupe`` event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&self) -> IoxirqDupeR {
        IoxirqDupeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``cam_rx_dupe`` event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&self) -> CamRxDupeR {
        CamRxDupeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&mut self) -> Pioirq0DupeW<'_, EvStatusSpec> {
        Pioirq0DupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&mut self) -> Pioirq1DupeW<'_, EvStatusSpec> {
        Pioirq1DupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&mut self) -> Pioirq2DupeW<'_, EvStatusSpec> {
        Pioirq2DupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&mut self) -> Pioirq3DupeW<'_, EvStatusSpec> {
        Pioirq3DupeW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``i2c2_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_rx_dupe(&mut self) -> I2c2RxDupeW<'_, EvStatusSpec> {
        I2c2RxDupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``i2c2_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_tx_dupe(&mut self) -> I2c2TxDupeW<'_, EvStatusSpec> {
        I2c2TxDupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``i2c2_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_cmd_dupe(&mut self) -> I2c2CmdDupeW<'_, EvStatusSpec> {
        I2c2CmdDupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``i2c2_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_eot_dupe(&mut self) -> I2c2EotDupeW<'_, EvStatusSpec> {
        I2c2EotDupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``i2c0_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_nack_dupe(&mut self) -> I2c0NackDupeW<'_, EvStatusSpec> {
        I2c0NackDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``i2c1_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_nack_dupe(&mut self) -> I2c1NackDupeW<'_, EvStatusSpec> {
        I2c1NackDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``i2c2_nack_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_nack_dupe(&mut self) -> I2c2NackDupeW<'_, EvStatusSpec> {
        I2c2NackDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``i2c0_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_err_dupe(&mut self) -> I2c0ErrDupeW<'_, EvStatusSpec> {
        I2c0ErrDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``i2c1_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_err_dupe(&mut self) -> I2c1ErrDupeW<'_, EvStatusSpec> {
        I2c1ErrDupeW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``i2c2_err_dupe`` event"]
    #[inline(always)]
    pub fn i2c2_err_dupe(&mut self) -> I2c2ErrDupeW<'_, EvStatusSpec> {
        I2c2ErrDupeW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``ioxirq_dupe`` event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&mut self) -> IoxirqDupeW<'_, EvStatusSpec> {
        IoxirqDupeW::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``cam_rx_dupe`` event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&mut self) -> CamRxDupeW<'_, EvStatusSpec> {
        CamRxDupeW::new(self, 15)
    }
}
#[doc = "`1` when a \"cam_rx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
