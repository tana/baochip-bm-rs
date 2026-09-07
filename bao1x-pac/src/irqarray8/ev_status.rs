#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `sdio_rx` reader - Level of the ``sdio_rx`` event"]
pub type SdioRxR = crate::BitReader;
#[doc = "Field `sdio_rx` writer - Level of the ``sdio_rx`` event"]
pub type SdioRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_tx` reader - Level of the ``sdio_tx`` event"]
pub type SdioTxR = crate::BitReader;
#[doc = "Field `sdio_tx` writer - Level of the ``sdio_tx`` event"]
pub type SdioTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_eot` reader - Level of the ``sdio_eot`` event"]
pub type SdioEotR = crate::BitReader;
#[doc = "Field `sdio_eot` writer - Level of the ``sdio_eot`` event"]
pub type SdioEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_err` reader - Level of the ``sdio_err`` event"]
pub type SdioErrR = crate::BitReader;
#[doc = "Field `sdio_err` writer - Level of the ``sdio_err`` event"]
pub type SdioErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_rx` reader - Level of the ``i2s_rx`` event"]
pub type I2sRxR = crate::BitReader;
#[doc = "Field `i2s_rx` writer - Level of the ``i2s_rx`` event"]
pub type I2sRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx` reader - Level of the ``i2s_tx`` event"]
pub type I2sTxR = crate::BitReader;
#[doc = "Field `i2s_tx` writer - Level of the ``i2s_tx`` event"]
pub type I2sTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s6` reader - Level of the ``nc_b8s6`` event"]
pub type NcB8s6R = crate::BitReader;
#[doc = "Field `nc_b8s6` writer - Level of the ``nc_b8s6`` event"]
pub type NcB8s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s7` reader - Level of the ``nc_b8s7`` event"]
pub type NcB8s7R = crate::BitReader;
#[doc = "Field `nc_b8s7` writer - Level of the ``nc_b8s7`` event"]
pub type NcB8s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cam_rx` reader - Level of the ``cam_rx`` event"]
pub type CamRxR = crate::BitReader;
#[doc = "Field `cam_rx` writer - Level of the ``cam_rx`` event"]
pub type CamRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_rx` reader - Level of the ``adc_rx`` event"]
pub type AdcRxR = crate::BitReader;
#[doc = "Field `adc_rx` writer - Level of the ``adc_rx`` event"]
pub type AdcRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s10` reader - Level of the ``nc_b8s10`` event"]
pub type NcB8s10R = crate::BitReader;
#[doc = "Field `nc_b8s10` writer - Level of the ``nc_b8s10`` event"]
pub type NcB8s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s11` reader - Level of the ``nc_b8s11`` event"]
pub type NcB8s11R = crate::BitReader;
#[doc = "Field `nc_b8s11` writer - Level of the ``nc_b8s11`` event"]
pub type NcB8s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_eot` reader - Level of the ``filter_eot`` event"]
pub type FilterEotR = crate::BitReader;
#[doc = "Field `filter_eot` writer - Level of the ``filter_eot`` event"]
pub type FilterEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_act` reader - Level of the ``filter_act`` event"]
pub type FilterActR = crate::BitReader;
#[doc = "Field `filter_act` writer - Level of the ``filter_act`` event"]
pub type FilterActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s14` reader - Level of the ``nc_b8s14`` event"]
pub type NcB8s14R = crate::BitReader;
#[doc = "Field `nc_b8s14` writer - Level of the ``nc_b8s14`` event"]
pub type NcB8s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s15` reader - Level of the ``nc_b8s15`` event"]
pub type NcB8s15R = crate::BitReader;
#[doc = "Field `nc_b8s15` writer - Level of the ``nc_b8s15`` event"]
pub type NcB8s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``sdio_rx`` event"]
    #[inline(always)]
    pub fn sdio_rx(&self) -> SdioRxR {
        SdioRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``sdio_tx`` event"]
    #[inline(always)]
    pub fn sdio_tx(&self) -> SdioTxR {
        SdioTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``sdio_eot`` event"]
    #[inline(always)]
    pub fn sdio_eot(&self) -> SdioEotR {
        SdioEotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``sdio_err`` event"]
    #[inline(always)]
    pub fn sdio_err(&self) -> SdioErrR {
        SdioErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``i2s_rx`` event"]
    #[inline(always)]
    pub fn i2s_rx(&self) -> I2sRxR {
        I2sRxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``i2s_tx`` event"]
    #[inline(always)]
    pub fn i2s_tx(&self) -> I2sTxR {
        I2sTxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``nc_b8s6`` event"]
    #[inline(always)]
    pub fn nc_b8s6(&self) -> NcB8s6R {
        NcB8s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b8s7`` event"]
    #[inline(always)]
    pub fn nc_b8s7(&self) -> NcB8s7R {
        NcB8s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``cam_rx`` event"]
    #[inline(always)]
    pub fn cam_rx(&self) -> CamRxR {
        CamRxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``adc_rx`` event"]
    #[inline(always)]
    pub fn adc_rx(&self) -> AdcRxR {
        AdcRxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b8s10`` event"]
    #[inline(always)]
    pub fn nc_b8s10(&self) -> NcB8s10R {
        NcB8s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b8s11`` event"]
    #[inline(always)]
    pub fn nc_b8s11(&self) -> NcB8s11R {
        NcB8s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``filter_eot`` event"]
    #[inline(always)]
    pub fn filter_eot(&self) -> FilterEotR {
        FilterEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``filter_act`` event"]
    #[inline(always)]
    pub fn filter_act(&self) -> FilterActR {
        FilterActR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b8s14`` event"]
    #[inline(always)]
    pub fn nc_b8s14(&self) -> NcB8s14R {
        NcB8s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b8s15`` event"]
    #[inline(always)]
    pub fn nc_b8s15(&self) -> NcB8s15R {
        NcB8s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``sdio_rx`` event"]
    #[inline(always)]
    pub fn sdio_rx(&mut self) -> SdioRxW<'_, EvStatusSpec> {
        SdioRxW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``sdio_tx`` event"]
    #[inline(always)]
    pub fn sdio_tx(&mut self) -> SdioTxW<'_, EvStatusSpec> {
        SdioTxW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``sdio_eot`` event"]
    #[inline(always)]
    pub fn sdio_eot(&mut self) -> SdioEotW<'_, EvStatusSpec> {
        SdioEotW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``sdio_err`` event"]
    #[inline(always)]
    pub fn sdio_err(&mut self) -> SdioErrW<'_, EvStatusSpec> {
        SdioErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``i2s_rx`` event"]
    #[inline(always)]
    pub fn i2s_rx(&mut self) -> I2sRxW<'_, EvStatusSpec> {
        I2sRxW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``i2s_tx`` event"]
    #[inline(always)]
    pub fn i2s_tx(&mut self) -> I2sTxW<'_, EvStatusSpec> {
        I2sTxW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``nc_b8s6`` event"]
    #[inline(always)]
    pub fn nc_b8s6(&mut self) -> NcB8s6W<'_, EvStatusSpec> {
        NcB8s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b8s7`` event"]
    #[inline(always)]
    pub fn nc_b8s7(&mut self) -> NcB8s7W<'_, EvStatusSpec> {
        NcB8s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``cam_rx`` event"]
    #[inline(always)]
    pub fn cam_rx(&mut self) -> CamRxW<'_, EvStatusSpec> {
        CamRxW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``adc_rx`` event"]
    #[inline(always)]
    pub fn adc_rx(&mut self) -> AdcRxW<'_, EvStatusSpec> {
        AdcRxW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b8s10`` event"]
    #[inline(always)]
    pub fn nc_b8s10(&mut self) -> NcB8s10W<'_, EvStatusSpec> {
        NcB8s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b8s11`` event"]
    #[inline(always)]
    pub fn nc_b8s11(&mut self) -> NcB8s11W<'_, EvStatusSpec> {
        NcB8s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``filter_eot`` event"]
    #[inline(always)]
    pub fn filter_eot(&mut self) -> FilterEotW<'_, EvStatusSpec> {
        FilterEotW::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``filter_act`` event"]
    #[inline(always)]
    pub fn filter_act(&mut self) -> FilterActW<'_, EvStatusSpec> {
        FilterActW::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b8s14`` event"]
    #[inline(always)]
    pub fn nc_b8s14(&mut self) -> NcB8s14W<'_, EvStatusSpec> {
        NcB8s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b8s15`` event"]
    #[inline(always)]
    pub fn nc_b8s15(&mut self) -> NcB8s15W<'_, EvStatusSpec> {
        NcB8s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
