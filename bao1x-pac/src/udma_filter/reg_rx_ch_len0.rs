#[doc = "Register `REG_RX_CH_LEN0` reader"]
pub type R = crate::R<RegRxChLen0Spec>;
#[doc = "Register `REG_RX_CH_LEN0` writer"]
pub type W = crate::W<RegRxChLen0Spec>;
#[doc = "Field `r_filter_rx_len0` reader - r_filter_rx_len0"]
pub type RFilterRxLen0R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_rx_len0` writer - r_filter_rx_len0"]
pub type RFilterRxLen0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_filter_rx_len0"]
    #[inline(always)]
    pub fn r_filter_rx_len0(&self) -> RFilterRxLen0R {
        RFilterRxLen0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_filter_rx_len0"]
    #[inline(always)]
    pub fn r_filter_rx_len0(&mut self) -> RFilterRxLen0W<'_, RegRxChLen0Spec> {
        RFilterRxLen0W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxChLen0Spec;
impl crate::RegisterSpec for RegRxChLen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_ch_len0::R`](R) reader structure"]
impl crate::Readable for RegRxChLen0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_ch_len0::W`](W) writer structure"]
impl crate::Writable for RegRxChLen0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CH_LEN0 to value 0"]
impl crate::Resettable for RegRxChLen0Spec {}
