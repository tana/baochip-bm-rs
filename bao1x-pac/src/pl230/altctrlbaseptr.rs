#[doc = "Register `ALTCTRLBASEPTR` reader"]
pub type R = crate::R<AltctrlbaseptrSpec>;
#[doc = "Field `ALT_CTRL_BASE_PTR` reader - ALT_CTRL_BASE_PTR"]
pub type AltCtrlBasePtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ALT_CTRL_BASE_PTR"]
    #[inline(always)]
    pub fn alt_ctrl_base_ptr(&self) -> AltCtrlBasePtrR {
        AltCtrlBasePtrR::new(self.bits)
    }
}
#[doc = "DMA Channel Alternate Control Data Base Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`altctrlbaseptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltctrlbaseptrSpec;
impl crate::RegisterSpec for AltctrlbaseptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altctrlbaseptr::R`](R) reader structure"]
impl crate::Readable for AltctrlbaseptrSpec {}
#[doc = "`reset()` method sets ALTCTRLBASEPTR to value 0"]
impl crate::Resettable for AltctrlbaseptrSpec {}
