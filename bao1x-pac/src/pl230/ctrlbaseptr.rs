#[doc = "Register `CTRLBASEPTR` reader"]
pub type R = crate::R<CtrlbaseptrSpec>;
#[doc = "Register `CTRLBASEPTR` writer"]
pub type W = crate::W<CtrlbaseptrSpec>;
#[doc = "Field `CTRL_BASE_PTR` reader - CTRL_BASE_PTR"]
pub type CtrlBasePtrR = crate::FieldReader<u32>;
#[doc = "Field `CTRL_BASE_PTR` writer - CTRL_BASE_PTR"]
pub type CtrlBasePtrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - CTRL_BASE_PTR"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&self) -> CtrlBasePtrR {
        CtrlBasePtrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - CTRL_BASE_PTR"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&mut self) -> CtrlBasePtrW<'_, CtrlbaseptrSpec> {
        CtrlBasePtrW::new(self, 8)
    }
}
#[doc = "DMA Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbaseptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbaseptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbaseptrSpec;
impl crate::RegisterSpec for CtrlbaseptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlbaseptr::R`](R) reader structure"]
impl crate::Readable for CtrlbaseptrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlbaseptr::W`](W) writer structure"]
impl crate::Writable for CtrlbaseptrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLBASEPTR to value 0"]
impl crate::Resettable for CtrlbaseptrSpec {}
