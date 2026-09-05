#[doc = "Register `REG_RX_CH_LEN2` reader"]
pub type R = crate::R<RegRxChLen2Spec>;
#[doc = "Register `REG_RX_CH_LEN2` writer"]
pub type W = crate::W<RegRxChLen2Spec>;
#[doc = "Field `r_filter_rx_len2` reader - r_filter_rx_len2"]
pub type RFilterRxLen2R = crate::FieldReader<u16>;
#[doc = "Field `r_filter_rx_len2` writer - r_filter_rx_len2"]
pub type RFilterRxLen2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - r_filter_rx_len2"]
    #[inline(always)]
    pub fn r_filter_rx_len2(&self) -> RFilterRxLen2R {
        RFilterRxLen2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - r_filter_rx_len2"]
    #[inline(always)]
    pub fn r_filter_rx_len2(&mut self) -> RFilterRxLen2W<'_, RegRxChLen2Spec> {
        RFilterRxLen2W::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_len2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_len2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxChLen2Spec;
impl crate::RegisterSpec for RegRxChLen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_ch_len2::R`](R) reader structure"]
impl crate::Readable for RegRxChLen2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_ch_len2::W`](W) writer structure"]
impl crate::Writable for RegRxChLen2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CH_LEN2 to value 0"]
impl crate::Resettable for RegRxChLen2Spec {}
