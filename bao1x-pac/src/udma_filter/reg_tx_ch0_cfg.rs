#[doc = "Register `REG_TX_CH0_CFG` reader"]
pub type R = crate::R<RegTxCh0CfgSpec>;
#[doc = "Register `REG_TX_CH0_CFG` writer"]
pub type W = crate::W<RegTxCh0CfgSpec>;
#[doc = "Field `r_filter_tx_datasize_0` reader - r_filter_tx_datasize_0"]
pub type RFilterTxDatasize0R = crate::FieldReader;
#[doc = "Field `r_filter_tx_datasize_0` writer - r_filter_tx_datasize_0"]
pub type RFilterTxDatasize0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_filter_tx_mode_0` reader - r_filter_tx_mode_0"]
pub type RFilterTxMode0R = crate::FieldReader;
#[doc = "Field `r_filter_tx_mode_0` writer - r_filter_tx_mode_0"]
pub type RFilterTxMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - r_filter_tx_datasize_0"]
    #[inline(always)]
    pub fn r_filter_tx_datasize_0(&self) -> RFilterTxDatasize0R {
        RFilterTxDatasize0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - r_filter_tx_mode_0"]
    #[inline(always)]
    pub fn r_filter_tx_mode_0(&self) -> RFilterTxMode0R {
        RFilterTxMode0R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - r_filter_tx_datasize_0"]
    #[inline(always)]
    pub fn r_filter_tx_datasize_0(&mut self) -> RFilterTxDatasize0W<'_, RegTxCh0CfgSpec> {
        RFilterTxDatasize0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - r_filter_tx_mode_0"]
    #[inline(always)]
    pub fn r_filter_tx_mode_0(&mut self) -> RFilterTxMode0W<'_, RegTxCh0CfgSpec> {
        RFilterTxMode0W::new(self, 8)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxCh0CfgSpec;
impl crate::RegisterSpec for RegTxCh0CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_ch0_cfg::R`](R) reader structure"]
impl crate::Readable for RegTxCh0CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_ch0_cfg::W`](W) writer structure"]
impl crate::Writable for RegTxCh0CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_CH0_CFG to value 0"]
impl crate::Resettable for RegTxCh0CfgSpec {}
