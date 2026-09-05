#[doc = "Register `CR_OCR` reader"]
pub type R = crate::R<CrOcrSpec>;
#[doc = "Register `CR_OCR` writer"]
pub type W = crate::W<CrOcrSpec>;
#[doc = "Field `cr_ocr` reader - cr_ocr read/write control register"]
pub type CrOcrR = crate::FieldReader<u32>;
#[doc = "Field `cr_ocr` writer - cr_ocr read/write control register"]
pub type CrOcrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - cr_ocr read/write control register"]
    #[inline(always)]
    pub fn cr_ocr(&self) -> CrOcrR {
        CrOcrR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - cr_ocr read/write control register"]
    #[inline(always)]
    pub fn cr_ocr(&mut self) -> CrOcrW<'_, CrOcrSpec> {
        CrOcrW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L116 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L116>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrOcrSpec;
impl crate::RegisterSpec for CrOcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_ocr::R`](R) reader structure"]
impl crate::Readable for CrOcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr_ocr::W`](W) writer structure"]
impl crate::Writable for CrOcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_OCR to value 0"]
impl crate::Resettable for CrOcrSpec {}
