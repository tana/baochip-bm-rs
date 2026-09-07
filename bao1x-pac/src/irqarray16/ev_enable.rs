#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `cam_rx_dupe` reader - Write a ``1`` to enable the ``cam_rx_dupe`` Event"]
pub type CamRxDupeR = crate::BitReader;
#[doc = "Field `cam_rx_dupe` writer - Write a ``1`` to enable the ``cam_rx_dupe`` Event"]
pub type CamRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_rx_dupe` reader - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
pub type I2sRxDupeR = crate::BitReader;
#[doc = "Field `i2s_rx_dupe` writer - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
pub type I2sRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx_dupe` reader - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
pub type I2sTxDupeR = crate::BitReader;
#[doc = "Field `i2s_tx_dupe` writer - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
pub type I2sTxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b16s3` reader - Write a ``1`` to enable the ``nc_b16s3`` Event"]
pub type NcB16s3R = crate::BitReader;
#[doc = "Field `nc_b16s3` writer - Write a ``1`` to enable the ``nc_b16s3`` Event"]
pub type NcB16s3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_rx_dupe` reader - Write a ``1`` to enable the ``spim1_rx_dupe`` Event"]
pub type Spim1RxDupeR = crate::BitReader;
#[doc = "Field `spim1_rx_dupe` writer - Write a ``1`` to enable the ``spim1_rx_dupe`` Event"]
pub type Spim1RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_tx_dupe` reader - Write a ``1`` to enable the ``spim1_tx_dupe`` Event"]
pub type Spim1TxDupeR = crate::BitReader;
#[doc = "Field `spim1_tx_dupe` writer - Write a ``1`` to enable the ``spim1_tx_dupe`` Event"]
pub type Spim1TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_cmd_dupe` reader - Write a ``1`` to enable the ``spim1_cmd_dupe`` Event"]
pub type Spim1CmdDupeR = crate::BitReader;
#[doc = "Field `spim1_cmd_dupe` writer - Write a ``1`` to enable the ``spim1_cmd_dupe`` Event"]
pub type Spim1CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim1_eot_dupe` reader - Write a ``1`` to enable the ``spim1_eot_dupe`` Event"]
pub type Spim1EotDupeR = crate::BitReader;
#[doc = "Field `spim1_eot_dupe` writer - Write a ``1`` to enable the ``spim1_eot_dupe`` Event"]
pub type Spim1EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_rx_dupe` reader - Write a ``1`` to enable the ``spim2_rx_dupe`` Event"]
pub type Spim2RxDupeR = crate::BitReader;
#[doc = "Field `spim2_rx_dupe` writer - Write a ``1`` to enable the ``spim2_rx_dupe`` Event"]
pub type Spim2RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_tx_dupe` reader - Write a ``1`` to enable the ``spim2_tx_dupe`` Event"]
pub type Spim2TxDupeR = crate::BitReader;
#[doc = "Field `spim2_tx_dupe` writer - Write a ``1`` to enable the ``spim2_tx_dupe`` Event"]
pub type Spim2TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_cmd_dupe` reader - Write a ``1`` to enable the ``spim2_cmd_dupe`` Event"]
pub type Spim2CmdDupeR = crate::BitReader;
#[doc = "Field `spim2_cmd_dupe` writer - Write a ``1`` to enable the ``spim2_cmd_dupe`` Event"]
pub type Spim2CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spim2_eot_dupe` reader - Write a ``1`` to enable the ``spim2_eot_dupe`` Event"]
pub type Spim2EotDupeR = crate::BitReader;
#[doc = "Field `spim2_eot_dupe` writer - Write a ``1`` to enable the ``spim2_eot_dupe`` Event"]
pub type Spim2EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_rx_dupe` reader - Write a ``1`` to enable the ``i2c0_rx_dupe`` Event"]
pub type I2c0RxDupeR = crate::BitReader;
#[doc = "Field `i2c0_rx_dupe` writer - Write a ``1`` to enable the ``i2c0_rx_dupe`` Event"]
pub type I2c0RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_tx_dupe` reader - Write a ``1`` to enable the ``i2c0_tx_dupe`` Event"]
pub type I2c0TxDupeR = crate::BitReader;
#[doc = "Field `i2c0_tx_dupe` writer - Write a ``1`` to enable the ``i2c0_tx_dupe`` Event"]
pub type I2c0TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_cmd_dupe` reader - Write a ``1`` to enable the ``i2c0_cmd_dupe`` Event"]
pub type I2c0CmdDupeR = crate::BitReader;
#[doc = "Field `i2c0_cmd_dupe` writer - Write a ``1`` to enable the ``i2c0_cmd_dupe`` Event"]
pub type I2c0CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_eot_dupe` reader - Write a ``1`` to enable the ``i2c0_eot_dupe`` Event"]
pub type I2c0EotDupeR = crate::BitReader;
#[doc = "Field `i2c0_eot_dupe` writer - Write a ``1`` to enable the ``i2c0_eot_dupe`` Event"]
pub type I2c0EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``cam_rx_dupe`` Event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&self) -> CamRxDupeR {
        CamRxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&self) -> I2sRxDupeR {
        I2sRxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&self) -> I2sTxDupeR {
        I2sTxDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b16s3`` Event"]
    #[inline(always)]
    pub fn nc_b16s3(&self) -> NcB16s3R {
        NcB16s3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spim1_rx_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_rx_dupe(&self) -> Spim1RxDupeR {
        Spim1RxDupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spim1_tx_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_tx_dupe(&self) -> Spim1TxDupeR {
        Spim1TxDupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spim1_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_cmd_dupe(&self) -> Spim1CmdDupeR {
        Spim1CmdDupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``spim1_eot_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_eot_dupe(&self) -> Spim1EotDupeR {
        Spim1EotDupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spim2_rx_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_rx_dupe(&self) -> Spim2RxDupeR {
        Spim2RxDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spim2_tx_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_tx_dupe(&self) -> Spim2TxDupeR {
        Spim2TxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spim2_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_cmd_dupe(&self) -> Spim2CmdDupeR {
        Spim2CmdDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``spim2_eot_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_eot_dupe(&self) -> Spim2EotDupeR {
        Spim2EotDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``i2c0_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_rx_dupe(&self) -> I2c0RxDupeR {
        I2c0RxDupeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``i2c0_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_tx_dupe(&self) -> I2c0TxDupeR {
        I2c0TxDupeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``i2c0_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_cmd_dupe(&self) -> I2c0CmdDupeR {
        I2c0CmdDupeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``i2c0_eot_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_eot_dupe(&self) -> I2c0EotDupeR {
        I2c0EotDupeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``cam_rx_dupe`` Event"]
    #[inline(always)]
    pub fn cam_rx_dupe(&mut self) -> CamRxDupeW<'_, EvEnableSpec> {
        CamRxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2s_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_rx_dupe(&mut self) -> I2sRxDupeW<'_, EvEnableSpec> {
        I2sRxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``i2s_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2s_tx_dupe(&mut self) -> I2sTxDupeW<'_, EvEnableSpec> {
        I2sTxDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``nc_b16s3`` Event"]
    #[inline(always)]
    pub fn nc_b16s3(&mut self) -> NcB16s3W<'_, EvEnableSpec> {
        NcB16s3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``spim1_rx_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_rx_dupe(&mut self) -> Spim1RxDupeW<'_, EvEnableSpec> {
        Spim1RxDupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``spim1_tx_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_tx_dupe(&mut self) -> Spim1TxDupeW<'_, EvEnableSpec> {
        Spim1TxDupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``spim1_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_cmd_dupe(&mut self) -> Spim1CmdDupeW<'_, EvEnableSpec> {
        Spim1CmdDupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``spim1_eot_dupe`` Event"]
    #[inline(always)]
    pub fn spim1_eot_dupe(&mut self) -> Spim1EotDupeW<'_, EvEnableSpec> {
        Spim1EotDupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``spim2_rx_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_rx_dupe(&mut self) -> Spim2RxDupeW<'_, EvEnableSpec> {
        Spim2RxDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``spim2_tx_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_tx_dupe(&mut self) -> Spim2TxDupeW<'_, EvEnableSpec> {
        Spim2TxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``spim2_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_cmd_dupe(&mut self) -> Spim2CmdDupeW<'_, EvEnableSpec> {
        Spim2CmdDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``spim2_eot_dupe`` Event"]
    #[inline(always)]
    pub fn spim2_eot_dupe(&mut self) -> Spim2EotDupeW<'_, EvEnableSpec> {
        Spim2EotDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``i2c0_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_rx_dupe(&mut self) -> I2c0RxDupeW<'_, EvEnableSpec> {
        I2c0RxDupeW::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``i2c0_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_tx_dupe(&mut self) -> I2c0TxDupeW<'_, EvEnableSpec> {
        I2c0TxDupeW::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``i2c0_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_cmd_dupe(&mut self) -> I2c0CmdDupeW<'_, EvEnableSpec> {
        I2c0CmdDupeW::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``i2c0_eot_dupe`` Event"]
    #[inline(always)]
    pub fn i2c0_eot_dupe(&mut self) -> I2c0EotDupeW<'_, EvEnableSpec> {
        I2c0EotDupeW::new(self, 15)
    }
}
#[doc = "`1` when a \"i2c0_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
