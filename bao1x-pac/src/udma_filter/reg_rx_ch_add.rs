#[doc = "Register `REG_RX_CH_ADD` reader"]
pub type R = crate::R<RegRxChAddSpec>;
#[doc = "Register `REG_RX_CH_ADD` writer"]
pub type W = crate::W<RegRxChAddSpec>;
#[doc = "Field `r_filter_rx_start_addr` reader - r_filter_rx_start_addr"]
pub type RFilterRxStartAddrR = crate::FieldReader<u16>;
#[doc = "Field `r_filter_rx_start_addr` writer - r_filter_rx_start_addr"]
pub type RFilterRxStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - r_filter_rx_start_addr"]
    #[inline(always)]
    pub fn r_filter_rx_start_addr(&self) -> RFilterRxStartAddrR {
        RFilterRxStartAddrR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - r_filter_rx_start_addr"]
    #[inline(always)]
    pub fn r_filter_rx_start_addr(&mut self) -> RFilterRxStartAddrW<'_, RegRxChAddSpec> {
        RFilterRxStartAddrW::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rx_ch_add::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rx_ch_add::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxChAddSpec;
impl crate::RegisterSpec for RegRxChAddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rx_ch_add::R`](R) reader structure"]
impl crate::Readable for RegRxChAddSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rx_ch_add::W`](W) writer structure"]
impl crate::Writable for RegRxChAddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RX_CH_ADD to value 0"]
impl crate::Resettable for RegRxChAddSpec {}
