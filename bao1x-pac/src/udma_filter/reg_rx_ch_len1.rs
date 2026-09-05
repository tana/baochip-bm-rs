#[doc = "Register `REG_RX_CH_LEN1` reader"]
pub type R = crate::R<RegRxChLen1Spec>;
#[doc = "Register `REG_RX_CH_LEN1` writer"]
pub type W = crate::W<RegRxChLen1Spec>;
#[doc = "Field `r_filter_rx_len1` reader - r_filter_rx_len1"]
pub type RFilterRxLen1R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_rx_len1` writer - r_filter_rx_len1"]
pub type RFilterRxLen1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_filter_rx_len1"]
    #[inline(always)]
    pub fn r_filter_rx_len1(&self) -> RFilterRxLen1R {
        RFilterRxLen1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_filter_rx_len1"]
    #[inline(always)]
    pub fn r_filter_rx_len1(&mut self) -> RFilterRxLen1W<'_, RegRxChLen1Spec> {
        RFilterRxLen1W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxChLen1Spec;
impl crate::RegisterSpec for RegRxChLen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_ch_len1::R`](R) reader structure"]
impl crate::Readable for RegRxChLen1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_ch_len1::W`](W) writer structure"]
impl crate::Writable for RegRxChLen1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CH_LEN1 to value 0"]
impl crate::Resettable for RegRxChLen1Spec {}
