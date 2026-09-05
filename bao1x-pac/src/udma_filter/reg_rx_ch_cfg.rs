#[doc = "Register `REG_RX_CH_CFG` reader"]
pub type R = crate::R<RegRxChCfgSpec>;
#[doc = "Register `REG_RX_CH_CFG` writer"]
pub type W = crate::W<RegRxChCfgSpec>;
#[doc = "Field `r_filter_rx_datasize` reader - r_filter_rx_datasize"]
pub type RFilterRxDatasizeR = crate::FieldReader;
#[doc = "Field `r_filter_rx_datasize` writer - r_filter_rx_datasize"]
pub type RFilterRxDatasizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_filter_rx_mode` reader - r_filter_rx_mode"]
pub type RFilterRxModeR = crate::FieldReader;
#[doc = "Field `r_filter_rx_mode` writer - r_filter_rx_mode"]
pub type RFilterRxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - r_filter_rx_datasize"]
    #[inline(always)]
    pub fn r_filter_rx_datasize(&self) -> RFilterRxDatasizeR {
        RFilterRxDatasizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - r_filter_rx_mode"]
    #[inline(always)]
    pub fn r_filter_rx_mode(&self) -> RFilterRxModeR {
        RFilterRxModeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - r_filter_rx_datasize"]
    #[inline(always)]
    pub fn r_filter_rx_datasize(&mut self) -> RFilterRxDatasizeW<'_, RegRxChCfgSpec> {
        RFilterRxDatasizeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - r_filter_rx_mode"]
    #[inline(always)]
    pub fn r_filter_rx_mode(&mut self) -> RFilterRxModeW<'_, RegRxChCfgSpec> {
        RFilterRxModeW::new(self, 8)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxChCfgSpec;
impl crate::RegisterSpec for RegRxChCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_ch_cfg::R`](R) reader structure"]
impl crate::Readable for RegRxChCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_ch_cfg::W`](W) writer structure"]
impl crate::Writable for RegRxChCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CH_CFG to value 0"]
impl crate::Resettable for RegRxChCfgSpec {}
