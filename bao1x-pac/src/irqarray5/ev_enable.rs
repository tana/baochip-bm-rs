#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `uart0_rx` reader - Write a ``1`` to enable the ``uart0_rx`` Event"]
pub type Uart0RxR = crate::BitReader;
#[doc = "Field `uart0_rx` writer - Write a ``1`` to enable the ``uart0_rx`` Event"]
pub type Uart0RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart0_tx` reader - Write a ``1`` to enable the ``uart0_tx`` Event"]
pub type Uart0TxR = crate::BitReader;
#[doc = "Field `uart0_tx` writer - Write a ``1`` to enable the ``uart0_tx`` Event"]
pub type Uart0TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart0_rx_char` reader - Write a ``1`` to enable the ``uart0_rx_char`` Event"]
pub type Uart0RxCharR = crate::BitReader;
#[doc = "Field `uart0_rx_char` writer - Write a ``1`` to enable the ``uart0_rx_char`` Event"]
pub type Uart0RxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart0_err` reader - Write a ``1`` to enable the ``uart0_err`` Event"]
pub type Uart0ErrR = crate::BitReader;
#[doc = "Field `uart0_err` writer - Write a ``1`` to enable the ``uart0_err`` Event"]
pub type Uart0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_rx` reader - Write a ``1`` to enable the ``uart1_rx`` Event"]
pub type Uart1RxR = crate::BitReader;
#[doc = "Field `uart1_rx` writer - Write a ``1`` to enable the ``uart1_rx`` Event"]
pub type Uart1RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_tx` reader - Write a ``1`` to enable the ``uart1_tx`` Event"]
pub type Uart1TxR = crate::BitReader;
#[doc = "Field `uart1_tx` writer - Write a ``1`` to enable the ``uart1_tx`` Event"]
pub type Uart1TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_rx_char` reader - Write a ``1`` to enable the ``uart1_rx_char`` Event"]
pub type Uart1RxCharR = crate::BitReader;
#[doc = "Field `uart1_rx_char` writer - Write a ``1`` to enable the ``uart1_rx_char`` Event"]
pub type Uart1RxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1_err` reader - Write a ``1`` to enable the ``uart1_err`` Event"]
pub type Uart1ErrR = crate::BitReader;
#[doc = "Field `uart1_err` writer - Write a ``1`` to enable the ``uart1_err`` Event"]
pub type Uart1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_rx` reader - Write a ``1`` to enable the ``uart2_rx`` Event"]
pub type Uart2RxR = crate::BitReader;
#[doc = "Field `uart2_rx` writer - Write a ``1`` to enable the ``uart2_rx`` Event"]
pub type Uart2RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_tx` reader - Write a ``1`` to enable the ``uart2_tx`` Event"]
pub type Uart2TxR = crate::BitReader;
#[doc = "Field `uart2_tx` writer - Write a ``1`` to enable the ``uart2_tx`` Event"]
pub type Uart2TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_rx_char` reader - Write a ``1`` to enable the ``uart2_rx_char`` Event"]
pub type Uart2RxCharR = crate::BitReader;
#[doc = "Field `uart2_rx_char` writer - Write a ``1`` to enable the ``uart2_rx_char`` Event"]
pub type Uart2RxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart2_err` reader - Write a ``1`` to enable the ``uart2_err`` Event"]
pub type Uart2ErrR = crate::BitReader;
#[doc = "Field `uart2_err` writer - Write a ``1`` to enable the ``uart2_err`` Event"]
pub type Uart2ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_rx` reader - Write a ``1`` to enable the ``uart3_rx`` Event"]
pub type Uart3RxR = crate::BitReader;
#[doc = "Field `uart3_rx` writer - Write a ``1`` to enable the ``uart3_rx`` Event"]
pub type Uart3RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_tx` reader - Write a ``1`` to enable the ``uart3_tx`` Event"]
pub type Uart3TxR = crate::BitReader;
#[doc = "Field `uart3_tx` writer - Write a ``1`` to enable the ``uart3_tx`` Event"]
pub type Uart3TxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_rx_char` reader - Write a ``1`` to enable the ``uart3_rx_char`` Event"]
pub type Uart3RxCharR = crate::BitReader;
#[doc = "Field `uart3_rx_char` writer - Write a ``1`` to enable the ``uart3_rx_char`` Event"]
pub type Uart3RxCharW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart3_err` reader - Write a ``1`` to enable the ``uart3_err`` Event"]
pub type Uart3ErrR = crate::BitReader;
#[doc = "Field `uart3_err` writer - Write a ``1`` to enable the ``uart3_err`` Event"]
pub type Uart3ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``uart0_rx`` Event"]
    #[inline(always)]
    pub fn uart0_rx(&self) -> Uart0RxR {
        Uart0RxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``uart0_tx`` Event"]
    #[inline(always)]
    pub fn uart0_tx(&self) -> Uart0TxR {
        Uart0TxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``uart0_rx_char`` Event"]
    #[inline(always)]
    pub fn uart0_rx_char(&self) -> Uart0RxCharR {
        Uart0RxCharR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``uart0_err`` Event"]
    #[inline(always)]
    pub fn uart0_err(&self) -> Uart0ErrR {
        Uart0ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``uart1_rx`` Event"]
    #[inline(always)]
    pub fn uart1_rx(&self) -> Uart1RxR {
        Uart1RxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``uart1_tx`` Event"]
    #[inline(always)]
    pub fn uart1_tx(&self) -> Uart1TxR {
        Uart1TxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``uart1_rx_char`` Event"]
    #[inline(always)]
    pub fn uart1_rx_char(&self) -> Uart1RxCharR {
        Uart1RxCharR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``uart1_err`` Event"]
    #[inline(always)]
    pub fn uart1_err(&self) -> Uart1ErrR {
        Uart1ErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``uart2_rx`` Event"]
    #[inline(always)]
    pub fn uart2_rx(&self) -> Uart2RxR {
        Uart2RxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``uart2_tx`` Event"]
    #[inline(always)]
    pub fn uart2_tx(&self) -> Uart2TxR {
        Uart2TxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``uart2_rx_char`` Event"]
    #[inline(always)]
    pub fn uart2_rx_char(&self) -> Uart2RxCharR {
        Uart2RxCharR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``uart2_err`` Event"]
    #[inline(always)]
    pub fn uart2_err(&self) -> Uart2ErrR {
        Uart2ErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``uart3_rx`` Event"]
    #[inline(always)]
    pub fn uart3_rx(&self) -> Uart3RxR {
        Uart3RxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``uart3_tx`` Event"]
    #[inline(always)]
    pub fn uart3_tx(&self) -> Uart3TxR {
        Uart3TxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``uart3_rx_char`` Event"]
    #[inline(always)]
    pub fn uart3_rx_char(&self) -> Uart3RxCharR {
        Uart3RxCharR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``uart3_err`` Event"]
    #[inline(always)]
    pub fn uart3_err(&self) -> Uart3ErrR {
        Uart3ErrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``uart0_rx`` Event"]
    #[inline(always)]
    pub fn uart0_rx(&mut self) -> Uart0RxW<'_, EvEnableSpec> {
        Uart0RxW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``uart0_tx`` Event"]
    #[inline(always)]
    pub fn uart0_tx(&mut self) -> Uart0TxW<'_, EvEnableSpec> {
        Uart0TxW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``uart0_rx_char`` Event"]
    #[inline(always)]
    pub fn uart0_rx_char(&mut self) -> Uart0RxCharW<'_, EvEnableSpec> {
        Uart0RxCharW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``uart0_err`` Event"]
    #[inline(always)]
    pub fn uart0_err(&mut self) -> Uart0ErrW<'_, EvEnableSpec> {
        Uart0ErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``uart1_rx`` Event"]
    #[inline(always)]
    pub fn uart1_rx(&mut self) -> Uart1RxW<'_, EvEnableSpec> {
        Uart1RxW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``uart1_tx`` Event"]
    #[inline(always)]
    pub fn uart1_tx(&mut self) -> Uart1TxW<'_, EvEnableSpec> {
        Uart1TxW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``uart1_rx_char`` Event"]
    #[inline(always)]
    pub fn uart1_rx_char(&mut self) -> Uart1RxCharW<'_, EvEnableSpec> {
        Uart1RxCharW::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``uart1_err`` Event"]
    #[inline(always)]
    pub fn uart1_err(&mut self) -> Uart1ErrW<'_, EvEnableSpec> {
        Uart1ErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``uart2_rx`` Event"]
    #[inline(always)]
    pub fn uart2_rx(&mut self) -> Uart2RxW<'_, EvEnableSpec> {
        Uart2RxW::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``uart2_tx`` Event"]
    #[inline(always)]
    pub fn uart2_tx(&mut self) -> Uart2TxW<'_, EvEnableSpec> {
        Uart2TxW::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``uart2_rx_char`` Event"]
    #[inline(always)]
    pub fn uart2_rx_char(&mut self) -> Uart2RxCharW<'_, EvEnableSpec> {
        Uart2RxCharW::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``uart2_err`` Event"]
    #[inline(always)]
    pub fn uart2_err(&mut self) -> Uart2ErrW<'_, EvEnableSpec> {
        Uart2ErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``uart3_rx`` Event"]
    #[inline(always)]
    pub fn uart3_rx(&mut self) -> Uart3RxW<'_, EvEnableSpec> {
        Uart3RxW::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``uart3_tx`` Event"]
    #[inline(always)]
    pub fn uart3_tx(&mut self) -> Uart3TxW<'_, EvEnableSpec> {
        Uart3TxW::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``uart3_rx_char`` Event"]
    #[inline(always)]
    pub fn uart3_rx_char(&mut self) -> Uart3RxCharW<'_, EvEnableSpec> {
        Uart3RxCharW::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``uart3_err`` Event"]
    #[inline(always)]
    pub fn uart3_err(&mut self) -> Uart3ErrW<'_, EvEnableSpec> {
        Uart3ErrW::new(self, 15)
    }
}
#[doc = "`1` when a \"uart3_err\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
