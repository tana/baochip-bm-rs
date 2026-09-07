#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `cam_rx_dupe` reader - Level of the ``cam_rx_dupe`` event"]
pub type CamRxDupeR = crate::BitReader;
#[doc = "Field `cam_rx_dupe` writer - Level of the ``cam_rx_dupe`` event"]
pub type CamRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_rx_dupe` reader - Level of the ``i2s_rx_dupe`` event"]
pub type I2sRxDupeR = crate::BitReader;
#[doc = "Field `i2s_rx_dupe` writer - Level of the ``i2s_rx_dupe`` event"]
pub type I2sRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx_dupe` reader - Level of the ``i2s_tx_dupe`` event"]
pub type I2sTxDupeR = crate::BitReader;
#[doc = "Field `i2s_tx_dupe` writer - Level of the ``i2s_tx_dupe`` event"]
pub type I2sTxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b16s3` reader - Level of the ``nc_b16s3`` event"]
pub type NcB16s3R = crate::BitReader;
#[doc = "Field `nc_b16s3` writer - Level of the ``nc_b16s3`` event"]
pub type NcB16s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_rx_dupe` reader - Level of the ``spim1_rx_dupe`` event"]
pub type Spim1RxDupeR = crate::BitReader;
#[doc = "Field `spim1_rx_dupe` writer - Level of the ``spim1_rx_dupe`` event"]
pub type Spim1RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_tx_dupe` reader - Level of the ``spim1_tx_dupe`` event"]
pub type Spim1TxDupeR = crate::BitReader;
#[doc = "Field `spim1_tx_dupe` writer - Level of the ``spim1_tx_dupe`` event"]
pub type Spim1TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_cmd_dupe` reader - Level of the ``spim1_cmd_dupe`` event"]
pub type Spim1CmdDupeR = crate::BitReader;
#[doc = "Field `spim1_cmd_dupe` writer - Level of the ``spim1_cmd_dupe`` event"]
pub type Spim1CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_eot_dupe` reader - Level of the ``spim1_eot_dupe`` event"]
pub type Spim1EotDupeR = crate::BitReader;
#[doc = "Field `spim1_eot_dupe` writer - Level of the ``spim1_eot_dupe`` event"]
pub type Spim1EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_rx_dupe` reader - Level of the ``spim2_rx_dupe`` event"]
pub type Spim2RxDupeR = crate::BitReader;
#[doc = "Field `spim2_rx_dupe` writer - Level of the ``spim2_rx_dupe`` event"]
pub type Spim2RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_tx_dupe` reader - Level of the ``spim2_tx_dupe`` event"]
pub type Spim2TxDupeR = crate::BitReader;
#[doc = "Field `spim2_tx_dupe` writer - Level of the ``spim2_tx_dupe`` event"]
pub type Spim2TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_cmd_dupe` reader - Level of the ``spim2_cmd_dupe`` event"]
pub type Spim2CmdDupeR = crate::BitReader;
#[doc = "Field `spim2_cmd_dupe` writer - Level of the ``spim2_cmd_dupe`` event"]
pub type Spim2CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_eot_dupe` reader - Level of the ``spim2_eot_dupe`` event"]
pub type Spim2EotDupeR = crate::BitReader;
#[doc = "Field `spim2_eot_dupe` writer - Level of the ``spim2_eot_dupe`` event"]
pub type Spim2EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_rx_dupe` reader - Level of the ``i2c0_rx_dupe`` event"]
pub type I2c0RxDupeR = crate::BitReader;
#[doc = "Field `i2c0_rx_dupe` writer - Level of the ``i2c0_rx_dupe`` event"]
pub type I2c0RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_tx_dupe` reader - Level of the ``i2c0_tx_dupe`` event"]
pub type I2c0TxDupeR = crate::BitReader;
#[doc = "Field `i2c0_tx_dupe` writer - Level of the ``i2c0_tx_dupe`` event"]
pub type I2c0TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_cmd_dupe` reader - Level of the ``i2c0_cmd_dupe`` event"]
pub type I2c0CmdDupeR = crate::BitReader;
#[doc = "Field `i2c0_cmd_dupe` writer - Level of the ``i2c0_cmd_dupe`` event"]
pub type I2c0CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_eot_dupe` reader - Level of the ``i2c0_eot_dupe`` event"]
pub type I2c0EotDupeR = crate::BitReader;
#[doc = "Field `i2c0_eot_dupe` writer - Level of the ``i2c0_eot_dupe`` event"]
pub type I2c0EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``cam_rx_dupe`` event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&self) -> CamRxDupeR {
        CamRxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``i2s_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&self) -> I2sRxDupeR {
        I2sRxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``i2s_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&self) -> I2sTxDupeR {
        I2sTxDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``nc_b16s3`` event"]
    #[inline(always)]
    pub fn nc_b16s3(&self) -> NcB16s3R {
        NcB16s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``spim1_rx_dupe`` event"]
    #[inline(always)]
    pub fn spim1_rx_dupe(&self) -> Spim1RxDupeR {
        Spim1RxDupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``spim1_tx_dupe`` event"]
    #[inline(always)]
    pub fn spim1_tx_dupe(&self) -> Spim1TxDupeR {
        Spim1TxDupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``spim1_cmd_dupe`` event"]
    #[inline(always)]
    pub fn spim1_cmd_dupe(&self) -> Spim1CmdDupeR {
        Spim1CmdDupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``spim1_eot_dupe`` event"]
    #[inline(always)]
    pub fn spim1_eot_dupe(&self) -> Spim1EotDupeR {
        Spim1EotDupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``spim2_rx_dupe`` event"]
    #[inline(always)]
    pub fn spim2_rx_dupe(&self) -> Spim2RxDupeR {
        Spim2RxDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``spim2_tx_dupe`` event"]
    #[inline(always)]
    pub fn spim2_tx_dupe(&self) -> Spim2TxDupeR {
        Spim2TxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``spim2_cmd_dupe`` event"]
    #[inline(always)]
    pub fn spim2_cmd_dupe(&self) -> Spim2CmdDupeR {
        Spim2CmdDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``spim2_eot_dupe`` event"]
    #[inline(always)]
    pub fn spim2_eot_dupe(&self) -> Spim2EotDupeR {
        Spim2EotDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``i2c0_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_rx_dupe(&self) -> I2c0RxDupeR {
        I2c0RxDupeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``i2c0_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_tx_dupe(&self) -> I2c0TxDupeR {
        I2c0TxDupeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``i2c0_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_cmd_dupe(&self) -> I2c0CmdDupeR {
        I2c0CmdDupeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``i2c0_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_eot_dupe(&self) -> I2c0EotDupeR {
        I2c0EotDupeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``cam_rx_dupe`` event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&mut self) -> CamRxDupeW<'_, EvStatusSpec> {
        CamRxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``i2s_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&mut self) -> I2sRxDupeW<'_, EvStatusSpec> {
        I2sRxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``i2s_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&mut self) -> I2sTxDupeW<'_, EvStatusSpec> {
        I2sTxDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``nc_b16s3`` event"]
    #[inline(always)]
    pub fn nc_b16s3(&mut self) -> NcB16s3W<'_, EvStatusSpec> {
        NcB16s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``spim1_rx_dupe`` event"]
    #[inline(always)]
    pub fn spim1_rx_dupe(&mut self) -> Spim1RxDupeW<'_, EvStatusSpec> {
        Spim1RxDupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``spim1_tx_dupe`` event"]
    #[inline(always)]
    pub fn spim1_tx_dupe(&mut self) -> Spim1TxDupeW<'_, EvStatusSpec> {
        Spim1TxDupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``spim1_cmd_dupe`` event"]
    #[inline(always)]
    pub fn spim1_cmd_dupe(&mut self) -> Spim1CmdDupeW<'_, EvStatusSpec> {
        Spim1CmdDupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``spim1_eot_dupe`` event"]
    #[inline(always)]
    pub fn spim1_eot_dupe(&mut self) -> Spim1EotDupeW<'_, EvStatusSpec> {
        Spim1EotDupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``spim2_rx_dupe`` event"]
    #[inline(always)]
    pub fn spim2_rx_dupe(&mut self) -> Spim2RxDupeW<'_, EvStatusSpec> {
        Spim2RxDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``spim2_tx_dupe`` event"]
    #[inline(always)]
    pub fn spim2_tx_dupe(&mut self) -> Spim2TxDupeW<'_, EvStatusSpec> {
        Spim2TxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``spim2_cmd_dupe`` event"]
    #[inline(always)]
    pub fn spim2_cmd_dupe(&mut self) -> Spim2CmdDupeW<'_, EvStatusSpec> {
        Spim2CmdDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``spim2_eot_dupe`` event"]
    #[inline(always)]
    pub fn spim2_eot_dupe(&mut self) -> Spim2EotDupeW<'_, EvStatusSpec> {
        Spim2EotDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``i2c0_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_rx_dupe(&mut self) -> I2c0RxDupeW<'_, EvStatusSpec> {
        I2c0RxDupeW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``i2c0_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_tx_dupe(&mut self) -> I2c0TxDupeW<'_, EvStatusSpec> {
        I2c0TxDupeW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``i2c0_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_cmd_dupe(&mut self) -> I2c0CmdDupeW<'_, EvStatusSpec> {
        I2c0CmdDupeW::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``i2c0_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c0_eot_dupe(&mut self) -> I2c0EotDupeW<'_, EvStatusSpec> {
        I2c0EotDupeW::new(self, 15)
    }
}
#[doc = "`1` when a \"i2c0_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
