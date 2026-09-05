#[doc = "Register `REG_DATA` reader"]
pub type R = crate::R<RegDataSpec>;
#[doc = "Register `REG_DATA` writer"]
pub type W = crate::W<RegDataSpec>;
#[doc = "Field `r_uart_rx_data` reader - r_uart_rx_data"]
pub type RUartRxDataR = crate::FieldReader;
#[doc = "Field `r_uart_rx_data` writer - r_uart_rx_data"]
pub type RUartRxDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - r_uart_rx_data"]
    #[inline(always)]
    pub fn r_uart_rx_data(&self) -> RUartRxDataR {
        RUartRxDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - r_uart_rx_data"]
    #[inline(always)]
    pub fn r_uart_rx_data(&mut self) -> RUartRxDataW<'_, RegDataSpec> {
        RUartRxDataW::new(self, 0)
    }
}
#[doc = "See `udma_uart_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_uart/rtl/udma_uart_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDataSpec;
impl crate::RegisterSpec for RegDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_data::R`](R) reader structure"]
impl crate::Readable for RegDataSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_data::W`](W) writer structure"]
impl crate::Writable for RegDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DATA to value 0"]
impl crate::Resettable for RegDataSpec {}
