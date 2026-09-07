#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `sdio_rx` reader - `1` when a \"sdio_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioRxR = crate::BitReader;
#[doc = "Field `sdio_rx` writer - `1` when a \"sdio_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_tx` reader - `1` when a \"sdio_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioTxR = crate::BitReader;
#[doc = "Field `sdio_tx` writer - `1` when a \"sdio_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_eot` reader - `1` when a \"sdio_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioEotR = crate::BitReader;
#[doc = "Field `sdio_eot` writer - `1` when a \"sdio_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_err` reader - `1` when a \"sdio_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioErrR = crate::BitReader;
#[doc = "Field `sdio_err` writer - `1` when a \"sdio_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_rx` reader - `1` when a \"i2s_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type I2sRxR = crate::BitReader;
#[doc = "Field `i2s_rx` writer - `1` when a \"i2s_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type I2sRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2s_tx` reader - `1` when a \"i2s_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type I2sTxR = crate::BitReader;
#[doc = "Field `i2s_tx` writer - `1` when a \"i2s_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type I2sTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s6` reader - `1` when a \"nc_b8s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s6R = crate::BitReader;
#[doc = "Field `nc_b8s6` writer - `1` when a \"nc_b8s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s7` reader - `1` when a \"nc_b8s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s7R = crate::BitReader;
#[doc = "Field `nc_b8s7` writer - `1` when a \"nc_b8s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cam_rx` reader - `1` when a \"cam_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type CamRxR = crate::BitReader;
#[doc = "Field `cam_rx` writer - `1` when a \"cam_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type CamRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_rx` reader - `1` when a \"adc_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AdcRxR = crate::BitReader;
#[doc = "Field `adc_rx` writer - `1` when a \"adc_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AdcRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s10` reader - `1` when a \"nc_b8s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s10R = crate::BitReader;
#[doc = "Field `nc_b8s10` writer - `1` when a \"nc_b8s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s11` reader - `1` when a \"nc_b8s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s11R = crate::BitReader;
#[doc = "Field `nc_b8s11` writer - `1` when a \"nc_b8s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_eot` reader - `1` when a \"filter_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type FilterEotR = crate::BitReader;
#[doc = "Field `filter_eot` writer - `1` when a \"filter_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type FilterEotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filter_act` reader - `1` when a \"filter_act\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type FilterActR = crate::BitReader;
#[doc = "Field `filter_act` writer - `1` when a \"filter_act\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type FilterActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s14` reader - `1` when a \"nc_b8s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s14R = crate::BitReader;
#[doc = "Field `nc_b8s14` writer - `1` when a \"nc_b8s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b8s15` reader - `1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s15R = crate::BitReader;
#[doc = "Field `nc_b8s15` writer - `1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB8s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"sdio_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_rx(&self) -> SdioRxR {
        SdioRxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"sdio_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_tx(&self) -> SdioTxR {
        SdioTxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"sdio_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_eot(&self) -> SdioEotR {
        SdioEotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"sdio_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_err(&self) -> SdioErrR {
        SdioErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"i2s_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn i2s_rx(&self) -> I2sRxR {
        I2sRxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"i2s_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn i2s_tx(&self) -> I2sTxR {
        I2sTxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"nc_b8s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s6(&self) -> NcB8s6R {
        NcB8s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b8s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s7(&self) -> NcB8s7R {
        NcB8s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"cam_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn cam_rx(&self) -> CamRxR {
        CamRxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"adc_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn adc_rx(&self) -> AdcRxR {
        AdcRxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b8s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s10(&self) -> NcB8s10R {
        NcB8s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b8s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s11(&self) -> NcB8s11R {
        NcB8s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"filter_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn filter_eot(&self) -> FilterEotR {
        FilterEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"filter_act\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn filter_act(&self) -> FilterActR {
        FilterActR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b8s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s14(&self) -> NcB8s14R {
        NcB8s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s15(&self) -> NcB8s15R {
        NcB8s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"sdio_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_rx(&mut self) -> SdioRxW<'_, EvPendingSpec> {
        SdioRxW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"sdio_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_tx(&mut self) -> SdioTxW<'_, EvPendingSpec> {
        SdioTxW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"sdio_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_eot(&mut self) -> SdioEotW<'_, EvPendingSpec> {
        SdioEotW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"sdio_err\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_err(&mut self) -> SdioErrW<'_, EvPendingSpec> {
        SdioErrW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"i2s_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn i2s_rx(&mut self) -> I2sRxW<'_, EvPendingSpec> {
        I2sRxW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"i2s_tx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn i2s_tx(&mut self) -> I2sTxW<'_, EvPendingSpec> {
        I2sTxW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"nc_b8s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s6(&mut self) -> NcB8s6W<'_, EvPendingSpec> {
        NcB8s6W::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b8s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s7(&mut self) -> NcB8s7W<'_, EvPendingSpec> {
        NcB8s7W::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"cam_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn cam_rx(&mut self) -> CamRxW<'_, EvPendingSpec> {
        CamRxW::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"adc_rx\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn adc_rx(&mut self) -> AdcRxW<'_, EvPendingSpec> {
        AdcRxW::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b8s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s10(&mut self) -> NcB8s10W<'_, EvPendingSpec> {
        NcB8s10W::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b8s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s11(&mut self) -> NcB8s11W<'_, EvPendingSpec> {
        NcB8s11W::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"filter_eot\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn filter_eot(&mut self) -> FilterEotW<'_, EvPendingSpec> {
        FilterEotW::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"filter_act\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn filter_act(&mut self) -> FilterActW<'_, EvPendingSpec> {
        FilterActW::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b8s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s14(&mut self) -> NcB8s14W<'_, EvPendingSpec> {
        NcB8s14W::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b8s15(&mut self) -> NcB8s15W<'_, EvPendingSpec> {
        NcB8s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b8s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
