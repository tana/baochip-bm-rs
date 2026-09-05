#[doc = "Register `SFR_SEGPTR_SEGID_HOUT2` reader"]
pub type R = crate::R<SfrSegptrSegidHout2Spec>;
#[doc = "Register `SFR_SEGPTR_SEGID_HOUT2` writer"]
pub type W = crate::W<SfrSegptrSegidHout2Spec>;
#[doc = "Field `SEGID_HOUT2` reader - cr_segptrstart read/write control register"]
pub type SegidHout2R = crate::FieldReader<u16>;
#[doc = "Field `SEGID_HOUT2` writer - cr_segptrstart read/write control register"]
pub type SegidHout2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_hout2(&self) -> SegidHout2R {
        SegidHout2R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_hout2(&mut self) -> SegidHout2W<'_, SfrSegptrSegidHout2Spec> {
        SegidHout2W::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_hout2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_hout2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrSegidHout2Spec;
impl crate::RegisterSpec for SfrSegptrSegidHout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_segid_hout2::R`](R) reader structure"]
impl crate::Readable for SfrSegptrSegidHout2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_segid_hout2::W`](W) writer structure"]
impl crate::Writable for SfrSegptrSegidHout2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_SEGID_HOUT2 to value 0"]
impl crate::Resettable for SfrSegptrSegidHout2Spec {}
