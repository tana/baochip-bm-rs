#[doc = "Register `REG_IRQ_EN` reader"]
pub type R = crate::R<RegIrqEnSpec>;
#[doc = "Register `REG_IRQ_EN` writer"]
pub type W = crate::W<RegIrqEnSpec>;
#[doc = "Field `r_scif_rx_irq_en` reader - r_scif_rx_irq_en"]
pub type RScifRxIrqEnR = crate::BitReader;
#[doc = "Field `r_scif_rx_irq_en` writer - r_scif_rx_irq_en"]
pub type RScifRxIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_scif_err_irq_en` reader - r_scif_err_irq_en"]
pub type RScifErrIrqEnR = crate::BitReader;
#[doc = "Field `r_scif_err_irq_en` writer - r_scif_err_irq_en"]
pub type RScifErrIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - r_scif_rx_irq_en"]
    #[inline(always)]
    pub fn r_scif_rx_irq_en(&self) -> RScifRxIrqEnR {
        RScifRxIrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - r_scif_err_irq_en"]
    #[inline(always)]
    pub fn r_scif_err_irq_en(&self) -> RScifErrIrqEnR {
        RScifErrIrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - r_scif_rx_irq_en"]
    #[inline(always)]
    pub fn r_scif_rx_irq_en(&mut self) -> RScifRxIrqEnW<'_, RegIrqEnSpec> {
        RScifRxIrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - r_scif_err_irq_en"]
    #[inline(always)]
    pub fn r_scif_err_irq_en(&mut self) -> RScifErrIrqEnW<'_, RegIrqEnSpec> {
        RScifErrIrqEnW::new(self, 1)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
