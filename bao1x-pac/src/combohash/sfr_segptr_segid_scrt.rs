#[doc = "Register `SFR_SEGPTR_SEGID_SCRT` reader"]
pub type R = crate::R<SfrSegptrSegidScrtSpec>;
#[doc = "Register `SFR_SEGPTR_SEGID_SCRT` writer"]
pub type W = crate::W<SfrSegptrSegidScrtSpec>;
#[doc = "Field `SEGID_SCRT` reader - cr_segptrstart read/write control register"]
pub type SegidScrtR = crate::FieldReader<u16>;
#[doc = "Field `SEGID_SCRT` writer - cr_segptrstart read/write control register"]
pub type SegidScrtW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_scrt(&self) -> SegidScrtR {
        SegidScrtR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_scrt(&mut self) -> SegidScrtW<'_, SfrSegptrSegidScrtSpec> {
        SegidScrtW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_scrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_scrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrSegidScrtSpec;
impl crate::RegisterSpec for SfrSegptrSegidScrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_segid_scrt::R`](R) reader structure"]
impl crate::Readable for SfrSegptrSegidScrtSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_segid_scrt::W`](W) writer structure"]
impl crate::Writable for SfrSegptrSegidScrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_SEGID_SCRT to value 0"]
impl crate::Resettable for SfrSegptrSegidScrtSpec {}
