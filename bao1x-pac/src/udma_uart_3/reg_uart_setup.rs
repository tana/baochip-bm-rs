#[doc = "Register `REG_UART_SETUP` reader"]
pub type R = crate::R<RegUartSetupSpec>;
#[doc = "Register `REG_UART_SETUP` writer"]
pub type W = crate::W<RegUartSetupSpec>;
#[doc = "Field `r_uart_parity_en` reader - r_uart_parity_en"]
pub type RUartParityEnR = crate::BitReader;
#[doc = "Field `r_uart_parity_en` writer - r_uart_parity_en"]
pub type RUartParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_bits` reader - r_uart_bits"]
pub type RUartBitsR = crate::FieldReader;
#[doc = "Field `r_uart_bits` writer - r_uart_bits"]
pub type RUartBitsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_uart_stop_bits` reader - r_uart_stop_bits"]
pub type RUartStopBitsR = crate::BitReader;
#[doc = "Field `r_uart_stop_bits` writer - r_uart_stop_bits"]
pub type RUartStopBitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_rx_polling_en` reader - r_uart_rx_polling_en"]
pub type RUartRxPollingEnR = crate::BitReader;
#[doc = "Field `r_uart_rx_polling_en` writer - r_uart_rx_polling_en"]
pub type RUartRxPollingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_rx_clean_fifo` reader - r_uart_rx_clean_fifo"]
pub type RUartRxCleanFifoR = crate::BitReader;
#[doc = "Field `r_uart_rx_clean_fifo` writer - r_uart_rx_clean_fifo"]
pub type RUartRxCleanFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_en_tx` reader - r_uart_en_tx"]
pub type RUartEnTxR = crate::BitReader;
#[doc = "Field `r_uart_en_tx` writer - r_uart_en_tx"]
pub type RUartEnTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_en_rx` reader - r_uart_en_rx"]
pub type RUartEnRxR = crate::BitReader;
#[doc = "Field `r_uart_en_rx` writer - r_uart_en_rx"]
pub type RUartEnRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_div` reader - r_uart_div"]
pub type RUartDivR = crate::FieldReader<u16>;
#[doc = "Field `r_uart_div` writer - r_uart_div"]
pub type RUartDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - r_uart_parity_en"]
    #[inline(always)]
    pub fn r_uart_parity_en(&self) -> RUartParityEnR {
        RUartParityEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - r_uart_bits"]
    #[inline(always)]
    pub fn r_uart_bits(&self) -> RUartBitsR {
        RUartBitsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - r_uart_stop_bits"]
    #[inline(always)]
    pub fn r_uart_stop_bits(&self) -> RUartStopBitsR {
        RUartStopBitsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_uart_rx_polling_en"]
    #[inline(always)]
    pub fn r_uart_rx_polling_en(&self) -> RUartRxPollingEnR {
        RUartRxPollingEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - r_uart_rx_clean_fifo"]
    #[inline(always)]
    pub fn r_uart_rx_clean_fifo(&self) -> RUartRxCleanFifoR {
        RUartRxCleanFifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - r_uart_en_tx"]
    #[inline(always)]
    pub fn r_uart_en_tx(&self) -> RUartEnTxR {
        RUartEnTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - r_uart_en_rx"]
    #[inline(always)]
    pub fn r_uart_en_rx(&self) -> RUartEnRxR {
        RUartEnRxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - r_uart_div"]
    #[inline(always)]
    pub fn r_uart_div(&self) -> RUartDivR {
        RUartDivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - r_uart_parity_en"]
    #[inline(always)]
    pub fn r_uart_parity_en(&mut self) -> RUartParityEnW<'_, RegUartSetupSpec> {
        RUartParityEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - r_uart_bits"]
    #[inline(always)]
    pub fn r_uart_bits(&mut self) -> RUartBitsW<'_, RegUartSetupSpec> {
        RUartBitsW::new(self, 1)
    }
    #[doc = "Bit 3 - r_uart_stop_bits"]
    #[inline(always)]
    pub fn r_uart_stop_bits(&mut self) -> RUartStopBitsW<'_, RegUartSetupSpec> {
        RUartStopBitsW::new(self, 3)
    }
    #[doc = "Bit 4 - r_uart_rx_polling_en"]
    #[inline(always)]
    pub fn r_uart_rx_polling_en(&mut self) -> RUartRxPollingEnW<'_, RegUartSetupSpec> {
        RUartRxPollingEnW::new(self, 4)
    }
    #[doc = "Bit 5 - r_uart_rx_clean_fifo"]
    #[inline(always)]
    pub fn r_uart_rx_clean_fifo(&mut self) -> RUartRxCleanFifoW<'_, RegUartSetupSpec> {
        RUartRxCleanFifoW::new(self, 5)
    }
    #[doc = "Bit 8 - r_uart_en_tx"]
    #[inline(always)]
    pub fn r_uart_en_tx(&mut self) -> RUartEnTxW<'_, RegUartSetupSpec> {
        RUartEnTxW::new(self, 8)
    }
    #[doc = "Bit 9 - r_uart_en_rx"]
    #[inline(always)]
    pub fn r_uart_en_rx(&mut self) -> RUartEnRxW<'_, RegUartSetupSpec> {
        RUartEnRxW::new(self, 9)
    }
    #[doc = "Bits 16:31 - r_uart_div"]
    #[inline(always)]
    pub fn r_uart_div(&mut self) -> RUartDivW<'_, RegUartSetupSpec> {
        RUartDivW::new(self, 16)
    }
}
#[doc = "See `udma_uart_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_uart/rtl/udma_uart_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_uart_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_uart_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUartSetupSpec;
impl crate::RegisterSpec for RegUartSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_uart_setup::R`](R) reader structure"]
impl crate::Readable for RegUartSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_uart_setup::W`](W) writer structure"]
impl crate::Writable for RegUartSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UART_SETUP to value 0"]
impl crate::Resettable for RegUartSetupSpec {}
