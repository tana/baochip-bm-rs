#[doc = "Register `REG_SCIF_SETUP` reader"]
pub type R = crate::R<RegScifSetupSpec>;
#[doc = "Register `REG_SCIF_SETUP` writer"]
pub type W = crate::W<RegScifSetupSpec>;
#[doc = "Field `r_scif_parity_en` reader - r_scif_parity_en"]
pub type RScifParityEnR = crate::BitReader;
#[doc = "Field `r_scif_parity_en` writer - r_scif_parity_en"]
pub type RScifParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_bits` reader - r_scif_bits"]
pub type RScifBitsR = crate::FieldReader;
#[doc = "Field `r_scif_bits` writer - r_scif_bits"]
pub type RScifBitsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_scif_stop_bits` reader - r_scif_stop_bits"]
pub type RScifStopBitsR = crate::BitReader;
#[doc = "Field `r_scif_stop_bits` writer - r_scif_stop_bits"]
pub type RScifStopBitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_rx_polling_en` reader - r_scif_rx_polling_en"]
pub type RScifRxPollingEnR = crate::BitReader;
#[doc = "Field `r_scif_rx_polling_en` writer - r_scif_rx_polling_en"]
pub type RScifRxPollingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_rx_clean_fifo` reader - r_scif_rx_clean_fifo"]
pub type RScifRxCleanFifoR = crate::BitReader;
#[doc = "Field `r_scif_rx_clean_fifo` writer - r_scif_rx_clean_fifo"]
pub type RScifRxCleanFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_en_tx` reader - r_scif_en_tx"]
pub type RScifEnTxR = crate::BitReader;
#[doc = "Field `r_scif_en_tx` writer - r_scif_en_tx"]
pub type RScifEnTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_en_rx` reader - r_scif_en_rx"]
pub type RScifEnRxR = crate::BitReader;
#[doc = "Field `r_scif_en_rx` writer - r_scif_en_rx"]
pub type RScifEnRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_clksel` reader - r_scif_clksel"]
pub type RScifClkselR = crate::FieldReader;
#[doc = "Field `r_scif_clksel` writer - r_scif_clksel"]
pub type RScifClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `r_scif_div` reader - r_scif_div"]
pub type RScifDivR = crate::FieldReader<u16>;
#[doc = "Field `r_scif_div` writer - r_scif_div"]
pub type RScifDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - r_scif_parity_en"]
    #[inline(always)]
    pub fn r_scif_parity_en(&self) -> RScifParityEnR {
        RScifParityEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - r_scif_bits"]
    #[inline(always)]
    pub fn r_scif_bits(&self) -> RScifBitsR {
        RScifBitsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - r_scif_stop_bits"]
    #[inline(always)]
    pub fn r_scif_stop_bits(&self) -> RScifStopBitsR {
        RScifStopBitsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - r_scif_rx_polling_en"]
    #[inline(always)]
    pub fn r_scif_rx_polling_en(&self) -> RScifRxPollingEnR {
        RScifRxPollingEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - r_scif_rx_clean_fifo"]
    #[inline(always)]
    pub fn r_scif_rx_clean_fifo(&self) -> RScifRxCleanFifoR {
        RScifRxCleanFifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - r_scif_en_tx"]
    #[inline(always)]
    pub fn r_scif_en_tx(&self) -> RScifEnTxR {
        RScifEnTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - r_scif_en_rx"]
    #[inline(always)]
    pub fn r_scif_en_rx(&self) -> RScifEnRxR {
        RScifEnRxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:15 - r_scif_clksel"]
    #[inline(always)]
    pub fn r_scif_clksel(&self) -> RScifClkselR {
        RScifClkselR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - r_scif_div"]
    #[inline(always)]
    pub fn r_scif_div(&self) -> RScifDivR {
        RScifDivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - r_scif_parity_en"]
    #[inline(always)]
    pub fn r_scif_parity_en(&mut self) -> RScifParityEnW<'_, RegScifSetupSpec> {
        RScifParityEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - r_scif_bits"]
    #[inline(always)]
    pub fn r_scif_bits(&mut self) -> RScifBitsW<'_, RegScifSetupSpec> {
        RScifBitsW::new(self, 1)
    }
    #[doc = "Bit 3 - r_scif_stop_bits"]
    #[inline(always)]
    pub fn r_scif_stop_bits(&mut self) -> RScifStopBitsW<'_, RegScifSetupSpec> {
        RScifStopBitsW::new(self, 3)
    }
    #[doc = "Bit 4 - r_scif_rx_polling_en"]
    #[inline(always)]
    pub fn r_scif_rx_polling_en(&mut self) -> RScifRxPollingEnW<'_, RegScifSetupSpec> {
        RScifRxPollingEnW::new(self, 4)
    }
    #[doc = "Bit 5 - r_scif_rx_clean_fifo"]
    #[inline(always)]
    pub fn r_scif_rx_clean_fifo(&mut self) -> RScifRxCleanFifoW<'_, RegScifSetupSpec> {
        RScifRxCleanFifoW::new(self, 5)
    }
    #[doc = "Bit 8 - r_scif_en_tx"]
    #[inline(always)]
    pub fn r_scif_en_tx(&mut self) -> RScifEnTxW<'_, RegScifSetupSpec> {
        RScifEnTxW::new(self, 8)
    }
    #[doc = "Bit 9 - r_scif_en_rx"]
    #[inline(always)]
    pub fn r_scif_en_rx(&mut self) -> RScifEnRxW<'_, RegScifSetupSpec> {
        RScifEnRxW::new(self, 9)
    }
    #[doc = "Bits 14:15 - r_scif_clksel"]
    #[inline(always)]
    pub fn r_scif_clksel(&mut self) -> RScifClkselW<'_, RegScifSetupSpec> {
        RScifClkselW::new(self, 14)
    }
    #[doc = "Bits 16:31 - r_scif_div"]
    #[inline(always)]
    pub fn r_scif_div(&mut self) -> RScifDivW<'_, RegScifSetupSpec> {
        RScifDivW::new(self, 16)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_scif_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_scif_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegScifSetupSpec;
impl crate::RegisterSpec for RegScifSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_scif_setup::R`](R) reader structure"]
impl crate::Readable for RegScifSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_scif_setup::W`](W) writer structure"]
impl crate::Writable for RegScifSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SCIF_SETUP to value 0"]
impl crate::Resettable for RegScifSetupSpec {}
