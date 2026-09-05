#[doc = "Register `REG_TX_CH1_LEN2` reader"]
pub type R = crate::R<RegTxCh1Len2Spec>;
#[doc = "Register `REG_TX_CH1_LEN2` writer"]
pub type W = crate::W<RegTxCh1Len2Spec>;
#[doc = "Field `r_filter_tx_len2_1` reader - r_filter_tx_len2_1"]
pub type RFilterTxLen2_1R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_tx_len2_1` writer - r_filter_tx_len2_1"]
pub type RFilterTxLen2_1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - r_filter_tx_len2_1"]
    #[inline(always)]
    pub fn r_filter_tx_len2_1(&self) -> RFilterTxLen2_1R {
        RFilterTxLen2_1R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - r_filter_tx_len2_1"]
    #[inline(always)]
    pub fn r_filter_tx_len2_1(&mut self) -> RFilterTxLen2_1W<'_, RegTxCh1Len2Spec> {
        RFilterTxLen2_1W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch1_len2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch1_len2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxCh1Len2Spec;
impl crate::RegisterSpec for RegTxCh1Len2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_ch1_len2::R`](R) reader structure"]
impl crate::Readable for RegTxCh1Len2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_ch1_len2::W`](W) writer structure"]
impl crate::Writable for RegTxCh1Len2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_CH1_LEN2 to value 0"]
impl crate::Resettable for RegTxCh1Len2Spec {}
