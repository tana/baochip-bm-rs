#[doc = "Register `REG_TX_CH0_LEN1` reader"]
pub type R = crate::R<RegTxCh0Len1Spec>;
#[doc = "Register `REG_TX_CH0_LEN1` writer"]
pub type W = crate::W<RegTxCh0Len1Spec>;
#[doc = "Field `r_filter_tx_len1_0` reader - r_filter_tx_len1_0"]
pub type RFilterTxLen1_0R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_tx_len1_0` writer - r_filter_tx_len1_0"]
pub type RFilterTxLen1_0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - r_filter_tx_len1_0"]
    #[inline(always)]
    pub fn r_filter_tx_len1_0(&self) -> RFilterTxLen1_0R {
        RFilterTxLen1_0R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - r_filter_tx_len1_0"]
    #[inline(always)]
    pub fn r_filter_tx_len1_0(&mut self) -> RFilterTxLen1_0W<'_, RegTxCh0Len1Spec> {
        RFilterTxLen1_0W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tx_ch0_len1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tx_ch0_len1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxCh0Len1Spec;
impl crate::RegisterSpec for RegTxCh0Len1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tx_ch0_len1::R`](R) reader structure"]
impl crate::Readable for RegTxCh0Len1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_tx_ch0_len1::W`](W) writer structure"]
impl crate::Writable for RegTxCh0Len1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TX_CH0_LEN1 to value 0"]
impl crate::Resettable for RegTxCh0Len1Spec {}
