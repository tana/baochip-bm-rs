#[doc = "Register `REG_TX_CH0_LEN0` reader"]
pub type R = crate::R<RegTxCh0Len0Spec>;
#[doc = "Register `REG_TX_CH0_LEN0` writer"]
pub type W = crate::W<RegTxCh0Len0Spec>;
#[doc = "Field `r_filter_tx_len0_0` reader - r_filter_tx_len0_0"]
pub type RFilterTxLen0_0R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_tx_len0_0` writer - r_filter_tx_len0_0"]
pub type RFilterTxLen0_0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - r_filter_tx_len0_0"]
    #[inline(always)]
    pub fn r_filter_tx_len0_0(&self) -> RFilterTxLen0_0R {
        RFilterTxLen0_0R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - r_filter_tx_len0_0"]
    #[inline(always)]
    pub fn r_filter_tx_len0_0(&mut self) -> RFilterTxLen0_0W<'_, RegTxCh0Len0Spec> {
        RFilterTxLen0_0W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_len0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_len0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxCh0Len0Spec;
impl crate::RegisterSpec for RegTxCh0Len0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_ch0_len0::R`](R) reader structure"]
impl crate::Readable for RegTxCh0Len0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_ch0_len0::W`](W) writer structure"]
impl crate::Writable for RegTxCh0Len0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_CH0_LEN0 to value 0"]
impl crate::Resettable for RegTxCh0Len0Spec {}
