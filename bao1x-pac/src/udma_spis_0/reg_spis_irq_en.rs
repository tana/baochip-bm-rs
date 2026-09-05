#[doc = "Register `REG_SPIS_IRQ_EN` reader"]
pub type R = crate::R<RegSpisIrqEnSpec>;
#[doc = "Register `REG_SPIS_IRQ_EN` writer"]
pub type W = crate::W<RegSpisIrqEnSpec>;
#[doc = "Field `seot_irq_en` reader - seot_irq_en"]
pub type SeotIrqEnR = crate::BitReader;
#[doc = "Field `seot_irq_en` writer - seot_irq_en"]
pub type SeotIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - seot_irq_en"]
    #[inline(always)]
    pub fn seot_irq_en(&self) -> SeotIrqEnR {
        SeotIrqEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - seot_irq_en"]
    #[inline(always)]
    pub fn seot_irq_en(&mut self) -> SeotIrqEnW<'_, RegSpisIrqEnSpec> {
        SeotIrqEnW::new(self, 0)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_spis_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_spis_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpisIrqEnSpec;
impl crate::RegisterSpec for RegSpisIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_spis_irq_en::R`](R) reader structure"]
impl crate::Readable for RegSpisIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_spis_irq_en::W`](W) writer structure"]
impl crate::Writable for RegSpisIrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SPIS_IRQ_EN to value 0"]
impl crate::Resettable for RegSpisIrqEnSpec {}
