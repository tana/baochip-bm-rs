#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `sdio_rx` reader - Write a ``1`` to enable the ``sdio_rx`` Event"]
pub type SdioRxR = crate::BitReader;
#[doc = "Field `sdio_rx` writer - Write a ``1`` to enable the ``sdio_rx`` Event"]
pub type SdioRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_tx` reader - Write a ``1`` to enable the ``sdio_tx`` Event"]
pub type SdioTxR = crate::BitReader;
#[doc = "Field `sdio_tx` writer - Write a ``1`` to enable the ``sdio_tx`` Event"]
pub type SdioTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_eot` reader - Write a ``1`` to enable the ``sdio_eot`` Event"]
pub type SdioEotR = crate::BitReader;
#[doc = "Field `sdio_eot` writer - Write a ``1`` to enable the ``sdio_eot`` Event"]
pub type SdioEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_err` reader - Write a ``1`` to enable the ``sdio_err`` Event"]
pub type SdioErrR = crate::BitReader;
#[doc = "Field `sdio_err` writer - Write a ``1`` to enable the ``sdio_err`` Event"]
pub type SdioErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_rx` reader - Write a ``1`` to enable the ``i2s_rx`` Event"]
pub type I2sRxR = crate::BitReader;
#[doc = "Field `i2s_rx` writer - Write a ``1`` to enable the ``i2s_rx`` Event"]
pub type I2sRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx` reader - Write a ``1`` to enable the ``i2s_tx`` Event"]
pub type I2sTxR = crate::BitReader;
#[doc = "Field `i2s_tx` writer - Write a ``1`` to enable the ``i2s_tx`` Event"]
pub type I2sTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s6` reader - Write a ``1`` to enable the ``nc_b8s6`` Event"]
pub type NcB8s6R = crate::BitReader;
#[doc = "Field `nc_b8s6` writer - Write a ``1`` to enable the ``nc_b8s6`` Event"]
pub type NcB8s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s7` reader - Write a ``1`` to enable the ``nc_b8s7`` Event"]
pub type NcB8s7R = crate::BitReader;
#[doc = "Field `nc_b8s7` writer - Write a ``1`` to enable the ``nc_b8s7`` Event"]
pub type NcB8s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cam_rx` reader - Write a ``1`` to enable the ``cam_rx`` Event"]
pub type CamRxR = crate::BitReader;
#[doc = "Field `cam_rx` writer - Write a ``1`` to enable the ``cam_rx`` Event"]
pub type CamRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_rx` reader - Write a ``1`` to enable the ``adc_rx`` Event"]
pub type AdcRxR = crate::BitReader;
#[doc = "Field `adc_rx` writer - Write a ``1`` to enable the ``adc_rx`` Event"]
pub type AdcRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s10` reader - Write a ``1`` to enable the ``nc_b8s10`` Event"]
pub type NcB8s10R = crate::BitReader;
#[doc = "Field `nc_b8s10` writer - Write a ``1`` to enable the ``nc_b8s10`` Event"]
pub type NcB8s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s11` reader - Write a ``1`` to enable the ``nc_b8s11`` Event"]
pub type NcB8s11R = crate::BitReader;
#[doc = "Field `nc_b8s11` writer - Write a ``1`` to enable the ``nc_b8s11`` Event"]
pub type NcB8s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_eot` reader - Write a ``1`` to enable the ``filter_eot`` Event"]
pub type FilterEotR = crate::BitReader;
#[doc = "Field `filter_eot` writer - Write a ``1`` to enable the ``filter_eot`` Event"]
pub type FilterEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_act` reader - Write a ``1`` to enable the ``filter_act`` Event"]
pub type FilterActR = crate::BitReader;
#[doc = "Field `filter_act` writer - Write a ``1`` to enable the ``filter_act`` Event"]
pub type FilterActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s14` reader - Write a ``1`` to enable the ``nc_b8s14`` Event"]
pub type NcB8s14R = crate::BitReader;
#[doc = "Field `nc_b8s14` writer - Write a ``1`` to enable the ``nc_b8s14`` Event"]
pub type NcB8s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s15` reader - Write a ``1`` to enable the ``nc_b8s15`` Event"]
pub type NcB8s15R = crate::BitReader;
#[doc = "Field `nc_b8s15` writer - Write a ``1`` to enable the ``nc_b8s15`` Event"]
pub type NcB8s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``sdio_rx`` Event"]
    #[inline(always)]
    pub fn sdio_rx(&self) -> SdioRxR {
        SdioRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``sdio_tx`` Event"]
    #[inline(always)]
    pub fn sdio_tx(&self) -> SdioTxR {
        SdioTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``sdio_eot`` Event"]
    #[inline(always)]
    pub fn sdio_eot(&self) -> SdioEotR {
        SdioEotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``sdio_err`` Event"]
    #[inline(always)]
    pub fn sdio_err(&self) -> SdioErrR {
        SdioErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``i2s_rx`` Event"]
    #[inline(always)]
    pub fn i2s_rx(&self) -> I2sRxR {
        I2sRxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``i2s_tx`` Event"]
    #[inline(always)]
    pub fn i2s_tx(&self) -> I2sTxR {
        I2sTxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b8s6`` Event"]
    #[inline(always)]
    pub fn nc_b8s6(&self) -> NcB8s6R {
        NcB8s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b8s7`` Event"]
    #[inline(always)]
    pub fn nc_b8s7(&self) -> NcB8s7R {
        NcB8s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``cam_rx`` Event"]
    #[inline(always)]
    pub fn cam_rx(&self) -> CamRxR {
        CamRxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``adc_rx`` Event"]
    #[inline(always)]
    pub fn adc_rx(&self) -> AdcRxR {
        AdcRxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b8s10`` Event"]
    #[inline(always)]
    pub fn nc_b8s10(&self) -> NcB8s10R {
        NcB8s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b8s11`` Event"]
    #[inline(always)]
    pub fn nc_b8s11(&self) -> NcB8s11R {
        NcB8s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``filter_eot`` Event"]
    #[inline(always)]
    pub fn filter_eot(&self) -> FilterEotR {
        FilterEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``filter_act`` Event"]
    #[inline(always)]
    pub fn filter_act(&self) -> FilterActR {
        FilterActR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b8s14`` Event"]
    #[inline(always)]
    pub fn nc_b8s14(&self) -> NcB8s14R {
        NcB8s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b8s15`` Event"]
    #[inline(always)]
    pub fn nc_b8s15(&self) -> NcB8s15R {
        NcB8s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``sdio_rx`` Event"]
    #[inline(always)]
    pub fn sdio_rx(&mut self) -> SdioRxW<'_, EvEnableSpec> {
        SdioRxW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``sdio_tx`` Event"]
    #[inline(always)]
    pub fn sdio_tx(&mut self) -> SdioTxW<'_, EvEnableSpec> {
        SdioTxW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``sdio_eot`` Event"]
    #[inline(always)]
    pub fn sdio_eot(&mut self) -> SdioEotW<'_, EvEnableSpec> {
        SdioEotW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``sdio_err`` Event"]
    #[inline(always)]
    pub fn sdio_err(&mut self) -> SdioErrW<'_, EvEnableSpec> {
        SdioErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``i2s_rx`` Event"]
    #[inline(always)]
    pub fn i2s_rx(&mut self) -> I2sRxW<'_, EvEnableSpec> {
        I2sRxW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``i2s_tx`` Event"]
    #[inline(always)]
    pub fn i2s_tx(&mut self) -> I2sTxW<'_, EvEnableSpec> {
        I2sTxW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b8s6`` Event"]
    #[inline(always)]
    pub fn nc_b8s6(&mut self) -> NcB8s6W<'_, EvEnableSpec> {
        NcB8s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b8s7`` Event"]
    #[inline(always)]
    pub fn nc_b8s7(&mut self) -> NcB8s7W<'_, EvEnableSpec> {
        NcB8s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``cam_rx`` Event"]
    #[inline(always)]
    pub fn cam_rx(&mut self) -> CamRxW<'_, EvEnableSpec> {
        CamRxW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``adc_rx`` Event"]
    #[inline(always)]
    pub fn adc_rx(&mut self) -> AdcRxW<'_, EvEnableSpec> {
        AdcRxW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b8s10`` Event"]
    #[inline(always)]
    pub fn nc_b8s10(&mut self) -> NcB8s10W<'_, EvEnableSpec> {
        NcB8s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b8s11`` Event"]
    #[inline(always)]
    pub fn nc_b8s11(&mut self) -> NcB8s11W<'_, EvEnableSpec> {
        NcB8s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``filter_eot`` Event"]
    #[inline(always)]
    pub fn filter_eot(&mut self) -> FilterEotW<'_, EvEnableSpec> {
        FilterEotW::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``filter_act`` Event"]
    #[inline(always)]
    pub fn filter_act(&mut self) -> FilterActW<'_, EvEnableSpec> {
        FilterActW::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b8s14`` Event"]
    #[inline(always)]
    pub fn nc_b8s14(&mut self) -> NcB8s14W<'_, EvEnableSpec> {
        NcB8s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``nc_b8s15`` Event"]
    #[inline(always)]
    pub fn nc_b8s15(&mut self) -> NcB8s15W<'_, EvEnableSpec> {
        NcB8s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
