#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `i2c1_rx_dupe` reader - Level of the ``i2c1_rx_dupe`` event"]
pub type I2c1RxDupeR = crate::BitReader;
#[doc = "Field `i2c1_rx_dupe` writer - Level of the ``i2c1_rx_dupe`` event"]
pub type I2c1RxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_tx_dupe` reader - Level of the ``i2c1_tx_dupe`` event"]
pub type I2c1TxDupeR = crate::BitReader;
#[doc = "Field `i2c1_tx_dupe` writer - Level of the ``i2c1_tx_dupe`` event"]
pub type I2c1TxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_cmd_dupe` reader - Level of the ``i2c1_cmd_dupe`` event"]
pub type I2c1CmdDupeR = crate::BitReader;
#[doc = "Field `i2c1_cmd_dupe` writer - Level of the ``i2c1_cmd_dupe`` event"]
pub type I2c1CmdDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_eot_dupe` reader - Level of the ``i2c1_eot_dupe`` event"]
pub type I2c1EotDupeR = crate::BitReader;
#[doc = "Field `i2c1_eot_dupe` writer - Level of the ``i2c1_eot_dupe`` event"]
pub type I2c1EotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `qfcirq_dupe` reader - Level of the ``qfcirq_dupe`` event"]
pub type QfcirqDupeR = crate::BitReader;
#[doc = "Field `qfcirq_dupe` writer - Level of the ``qfcirq_dupe`` event"]
pub type QfcirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_rx_dupe` reader - Level of the ``adc_rx_dupe`` event"]
pub type AdcRxDupeR = crate::BitReader;
#[doc = "Field `adc_rx_dupe` writer - Level of the ``adc_rx_dupe`` event"]
pub type AdcRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioxirq_dupe` reader - Level of the ``ioxirq_dupe`` event"]
pub type IoxirqDupeR = crate::BitReader;
#[doc = "Field `ioxirq_dupe` writer - Level of the ``ioxirq_dupe`` event"]
pub type IoxirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sddcirq_dupe` reader - Level of the ``sddcirq_dupe`` event"]
pub type SddcirqDupeR = crate::BitReader;
#[doc = "Field `sddcirq_dupe` writer - Level of the ``sddcirq_dupe`` event"]
pub type SddcirqDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s12` reader - Level of the ``nc_b17s12`` event"]
pub type NcB17s12R = crate::BitReader;
#[doc = "Field `nc_b17s12` writer - Level of the ``nc_b17s12`` event"]
pub type NcB17s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s13` reader - Level of the ``nc_b17s13`` event"]
pub type NcB17s13R = crate::BitReader;
#[doc = "Field `nc_b17s13` writer - Level of the ``nc_b17s13`` event"]
pub type NcB17s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s14` reader - Level of the ``nc_b17s14`` event"]
pub type NcB17s14R = crate::BitReader;
#[doc = "Field `nc_b17s14` writer - Level of the ``nc_b17s14`` event"]
pub type NcB17s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b17s15` reader - Level of the ``nc_b17s15`` event"]
pub type NcB17s15R = crate::BitReader;
#[doc = "Field `nc_b17s15` writer - Level of the ``nc_b17s15`` event"]
pub type NcB17s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``i2c1_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_rx_dupe(&self) -> I2c1RxDupeR {
        I2c1RxDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``i2c1_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_tx_dupe(&self) -> I2c1TxDupeR {
        I2c1TxDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``i2c1_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_cmd_dupe(&self) -> I2c1CmdDupeR {
        I2c1CmdDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``i2c1_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_eot_dupe(&self) -> I2c1EotDupeR {
        I2c1EotDupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&self) -> Pioirq0DupeR {
        Pioirq0DupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&self) -> Pioirq1DupeR {
        Pioirq1DupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&self) -> Pioirq2DupeR {
        Pioirq2DupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&self) -> Pioirq3DupeR {
        Pioirq3DupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``qfcirq_dupe`` event"]
    #[inline(always)]
    pub fn qfcirq_dupe(&self) -> QfcirqDupeR {
        QfcirqDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``adc_rx_dupe`` event"]
    #[inline(always)]
    pub fn adc_rx_dupe(&self) -> AdcRxDupeR {
        AdcRxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``ioxirq_dupe`` event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&self) -> IoxirqDupeR {
        IoxirqDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``sddcirq_dupe`` event"]
    #[inline(always)]
    pub fn sddcirq_dupe(&self) -> SddcirqDupeR {
        SddcirqDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b17s12`` event"]
    #[inline(always)]
    pub fn nc_b17s12(&self) -> NcB17s12R {
        NcB17s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b17s13`` event"]
    #[inline(always)]
    pub fn nc_b17s13(&self) -> NcB17s13R {
        NcB17s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b17s14`` event"]
    #[inline(always)]
    pub fn nc_b17s14(&self) -> NcB17s14R {
        NcB17s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b17s15`` event"]
    #[inline(always)]
    pub fn nc_b17s15(&self) -> NcB17s15R {
        NcB17s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``i2c1_rx_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_rx_dupe(&mut self) -> I2c1RxDupeW<'_, EvStatusSpec> {
        I2c1RxDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``i2c1_tx_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_tx_dupe(&mut self) -> I2c1TxDupeW<'_, EvStatusSpec> {
        I2c1TxDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``i2c1_cmd_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_cmd_dupe(&mut self) -> I2c1CmdDupeW<'_, EvStatusSpec> {
        I2c1CmdDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``i2c1_eot_dupe`` event"]
    #[inline(always)]
    pub fn i2c1_eot_dupe(&mut self) -> I2c1EotDupeW<'_, EvStatusSpec> {
        I2c1EotDupeW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&mut self) -> Pioirq0DupeW<'_, EvStatusSpec> {
        Pioirq0DupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&mut self) -> Pioirq1DupeW<'_, EvStatusSpec> {
        Pioirq1DupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&mut self) -> Pioirq2DupeW<'_, EvStatusSpec> {
        Pioirq2DupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&mut self) -> Pioirq3DupeW<'_, EvStatusSpec> {
        Pioirq3DupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``qfcirq_dupe`` event"]
    #[inline(always)]
    pub fn qfcirq_dupe(&mut self) -> QfcirqDupeW<'_, EvStatusSpec> {
        QfcirqDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``adc_rx_dupe`` event"]
    #[inline(always)]
    pub fn adc_rx_dupe(&mut self) -> AdcRxDupeW<'_, EvStatusSpec> {
        AdcRxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``ioxirq_dupe`` event"]
    #[inline(always)]
    pub fn ioxirq_dupe(&mut self) -> IoxirqDupeW<'_, EvStatusSpec> {
        IoxirqDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``sddcirq_dupe`` event"]
    #[inline(always)]
    pub fn sddcirq_dupe(&mut self) -> SddcirqDupeW<'_, EvStatusSpec> {
        SddcirqDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b17s12`` event"]
    #[inline(always)]
    pub fn nc_b17s12(&mut self) -> NcB17s12W<'_, EvStatusSpec> {
        NcB17s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b17s13`` event"]
    #[inline(always)]
    pub fn nc_b17s13(&mut self) -> NcB17s13W<'_, EvStatusSpec> {
        NcB17s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b17s14`` event"]
    #[inline(always)]
    pub fn nc_b17s14(&mut self) -> NcB17s14W<'_, EvStatusSpec> {
        NcB17s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b17s15`` event"]
    #[inline(always)]
    pub fn nc_b17s15(&mut self) -> NcB17s15W<'_, EvStatusSpec> {
        NcB17s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b17s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
