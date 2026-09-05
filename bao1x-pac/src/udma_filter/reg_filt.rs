#[doc = "Register `REG_FILT` reader"]
pub type R = crate::R<RegFiltSpec>;
#[doc = "Register `REG_FILT` writer"]
pub type W = crate::W<RegFiltSpec>;
#[doc = "Field `r_filter_mode` reader - r_filter_mode"]
pub type RFilterModeR = crate::FieldReader;
#[doc = "Field `r_filter_mode` writer - r_filter_mode"]
pub type RFilterModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - r_filter_mode"]
    #[inline(always)]
    pub fn r_filter_mode(&self) -> RFilterModeR {
        RFilterModeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - r_filter_mode"]
    #[inline(always)]
    pub fn r_filter_mode(&mut self) -> RFilterModeW<'_, RegFiltSpec> {
        RFilterModeW::new(self, 0)
    }
}
#[doc = "See `udma_filter_reg_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ ips/udma/udma_filter/rtl/udma_filter_reg_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_filt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_filt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFiltSpec;
impl crate::RegisterSpec for RegFiltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_filt::R`](R) reader structure"]
impl crate::Readable for RegFiltSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_filt::W`](W) writer structure"]
impl crate::Writable for RegFiltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FILT to value 0"]
impl crate::Resettable for RegFiltSpec {}
