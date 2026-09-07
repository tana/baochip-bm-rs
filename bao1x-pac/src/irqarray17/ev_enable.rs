#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `i2c1_rx_dupe` reader - Write a ``1`` to enable the ``i2c1_rx_dupe`` Event"]
pub type I2c1RxDupeR = crate::BitReader;
#[doc = "Field `i2c1_rx_dupe` writer - Write a ``1`` to enable the ``i2c1_rx_dupe`` Event"]
pub type I2c1RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_tx_dupe` reader - Write a ``1`` to enable the ``i2c1_tx_dupe`` Event"]
pub type I2c1TxDupeR = crate::BitReader;
#[doc = "Field `i2c1_tx_dupe` writer - Write a ``1`` to enable the ``i2c1_tx_dupe`` Event"]
pub type I2c1TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_cmd_dupe` reader - Write a ``1`` to enable the ``i2c1_cmd_dupe`` Event"]
pub type I2c1CmdDupeR = crate::BitReader;
#[doc = "Field `i2c1_cmd_dupe` writer - Write a ``1`` to enable the ``i2c1_cmd_dupe`` Event"]
pub type I2c1CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_eot_dupe` reader - Write a ``1`` to enable the ``i2c1_eot_dupe`` Event"]
pub type I2c1EotDupeR = crate::BitReader;
#[doc = "Field `i2c1_eot_dupe` writer - Write a ``1`` to enable the ``i2c1_eot_dupe`` Event"]
pub type I2c1EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `qfcirq_dupe` reader - Write a ``1`` to enable the ``qfcirq_dupe`` Event"]
pub type QfcirqDupeR = crate::BitReader;
#[doc = "Field `qfcirq_dupe` writer - Write a ``1`` to enable the ``qfcirq_dupe`` Event"]
pub type QfcirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_rx_dupe` reader - Write a ``1`` to enable the ``adc_rx_dupe`` Event"]
pub type AdcRxDupeR = crate::BitReader;
#[doc = "Field `adc_rx_dupe` writer - Write a ``1`` to enable the ``adc_rx_dupe`` Event"]
pub type AdcRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioxirq_dupe` reader - Write a ``1`` to enable the ``ioxirq_dupe`` Event"]
pub type IoxirqDupeR = crate::BitReader;
#[doc = "Field `ioxirq_dupe` writer - Write a ``1`` to enable the ``ioxirq_dupe`` Event"]
pub type IoxirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sddcirq_dupe` reader - Write a ``1`` to enable the ``sddcirq_dupe`` Event"]
pub type SddcirqDupeR = crate::BitReader;
#[doc = "Field `sddcirq_dupe` writer - Write a ``1`` to enable the ``sddcirq_dupe`` Event"]
pub type SddcirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s12` reader - Write a ``1`` to enable the ``nc_b17s12`` Event"]
pub type NcB17s12R = crate::BitReader;
#[doc = "Field `nc_b17s12` writer - Write a ``1`` to enable the ``nc_b17s12`` Event"]
pub type NcB17s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s13` reader - Write a ``1`` to enable the ``nc_b17s13`` Event"]
pub type NcB17s13R = crate::BitReader;
#[doc = "Field `nc_b17s13` writer - Write a ``1`` to enable the ``nc_b17s13`` Event"]
pub type NcB17s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s14` reader - Write a ``1`` to enable the ``nc_b17s14`` Event"]
pub type NcB17s14R = crate::BitReader;
#[doc = "Field `nc_b17s14` writer - Write a ``1`` to enable the ``nc_b17s14`` Event"]
pub type NcB17s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s15` reader - Write a ``1`` to enable the ``nc_b17s15`` Event"]
pub type NcB17s15R = crate::BitReader;
#[doc = "Field `nc_b17s15` writer - Write a ``1`` to enable the ``nc_b17s15`` Event"]
pub type NcB17s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``i2c1_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_rx_dupe(&self) -> I2c1RxDupeR {
        I2c1RxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2c1_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_tx_dupe(&self) -> I2c1TxDupeR {
        I2c1TxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``i2c1_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_cmd_dupe(&self) -> I2c1CmdDupeR {
        I2c1CmdDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``i2c1_eot_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_eot_dupe(&self) -> I2c1EotDupeR {
        I2c1EotDupeR::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 8 - Write a ``1`` to enable the ``qfcirq_dupe`` Event"]
    #[inline(always)]
    pub fn qfcirq_dupe(&self) -> QfcirqDupeR {
        QfcirqDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``adc_rx_dupe`` Event"]
    #[inline(always)]
    pub fn adc_rx_dupe(&self) -> AdcRxDupeR {
        AdcRxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``ioxirq_dupe`` Event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&self) -> IoxirqDupeR {
        IoxirqDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``sddcirq_dupe`` Event"]
    #[inline(always)]
    pub fn sddcirq_dupe(&self) -> SddcirqDupeR {
        SddcirqDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b17s12`` Event"]
    #[inline(always)]
    pub fn nc_b17s12(&self) -> NcB17s12R {
        NcB17s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b17s13`` Event"]
    #[inline(always)]
    pub fn nc_b17s13(&self) -> NcB17s13R {
        NcB17s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b17s14`` Event"]
    #[inline(always)]
    pub fn nc_b17s14(&self) -> NcB17s14R {
        NcB17s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b17s15`` Event"]
    #[inline(always)]
    pub fn nc_b17s15(&self) -> NcB17s15R {
        NcB17s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``i2c1_rx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_rx_dupe(&mut self) -> I2c1RxDupeW<'_, EvEnableSpec> {
        I2c1RxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``i2c1_tx_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_tx_dupe(&mut self) -> I2c1TxDupeW<'_, EvEnableSpec> {
        I2c1TxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``i2c1_cmd_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_cmd_dupe(&mut self) -> I2c1CmdDupeW<'_, EvEnableSpec> {
        I2c1CmdDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``i2c1_eot_dupe`` Event"]
    #[inline(always)]
    pub fn i2c1_eot_dupe(&mut self) -> I2c1EotDupeW<'_, EvEnableSpec> {
        I2c1EotDupeW::new(self, 3)
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
    #[doc = "Bit 8 - Write a ``1`` to enable the ``qfcirq_dupe`` Event"]
    #[inline(always)]
    pub fn qfcirq_dupe(&mut self) -> QfcirqDupeW<'_, EvEnableSpec> {
        QfcirqDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``adc_rx_dupe`` Event"]
    #[inline(always)]
    pub fn adc_rx_dupe(&mut self) -> AdcRxDupeW<'_, EvEnableSpec> {
        AdcRxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``ioxirq_dupe`` Event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&mut self) -> IoxirqDupeW<'_, EvEnableSpec> {
        IoxirqDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``sddcirq_dupe`` Event"]
    #[inline(always)]
    pub fn sddcirq_dupe(&mut self) -> SddcirqDupeW<'_, EvEnableSpec> {
        SddcirqDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b17s12`` Event"]
    #[inline(always)]
    pub fn nc_b17s12(&mut self) -> NcB17s12W<'_, EvEnableSpec> {
        NcB17s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b17s13`` Event"]
    #[inline(always)]
    pub fn nc_b17s13(&mut self) -> NcB17s13W<'_, EvEnableSpec> {
        NcB17s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b17s14`` Event"]
    #[inline(always)]
    pub fn nc_b17s14(&mut self) -> NcB17s14W<'_, EvEnableSpec> {
        NcB17s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b17s15`` Event"]
    #[inline(always)]
    pub fn nc_b17s15(&mut self) -> NcB17s15W<'_, EvEnableSpec> {
        NcB17s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b17s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
