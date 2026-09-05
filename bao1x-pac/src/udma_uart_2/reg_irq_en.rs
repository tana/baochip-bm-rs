#[doc = "Register `REG_IRQ_EN` reader"]
pub type R = crate::R<RegIrqEnSpec>;
#[doc = "Register `REG_IRQ_EN` writer"]
pub type W = crate::W<RegIrqEnSpec>;
#[doc = "Field `r_uart_rx_irq_en` reader - r_uart_rx_irq_en"]
pub type RUartRxIrqEnR = crate::BitReader;
#[doc = "Field `r_uart_rx_irq_en` writer - r_uart_rx_irq_en"]
pub type RUartRxIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_uart_err_irq_en` reader - r_uart_err_irq_en"]
pub type RUartErrIrqEnR = crate::BitReader;
#[doc = "Field `r_uart_err_irq_en` writer - r_uart_err_irq_en"]
pub type RUartErrIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_uart_rx_irq_en"]
    #[inline(always)]
    pub fn r_uart_rx_irq_en(&self) -> RUartRxIrqEnR {
        RUartRxIrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_uart_err_irq_en"]
    #[inline(always)]
    pub fn r_uart_err_irq_en(&self) -> RUartErrIrqEnR {
        RUartErrIrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_uart_rx_irq_en"]
    #[inline(always)]
    pub fn r_uart_rx_irq_en(&mut self) -> RUartRxIrqEnW<'_, RegIrqEnSpec> {
        RUartRxIrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - r_uart_err_irq_en"]
    #[inline(always)]
    pub fn r_uart_err_irq_en(&mut self) -> RUartErrIrqEnW<'_, RegIrqEnSpec> {
        RUartErrIrqEnW::new(self, 1)
    }
}
#[doc = "See `udma_uart_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/udma/udma_uart/rtl/udma_uart_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIrqEnSpec;
impl crate::RegisterSpec for RegIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_irq_en::R`](R) reader structure"]
impl crate::Readable for RegIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_irq_en::W`](W) writer structure"]
impl crate::Writable for RegIrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_IRQ_EN to value 0"]
impl crate::Resettable for RegIrqEnSpec {}
