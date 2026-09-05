#[doc = "Register `SFR_SEGPTR_SEGID_KEY` reader"]
pub type R = crate::R<SfrSegptrSegidKeySpec>;
#[doc = "Register `SFR_SEGPTR_SEGID_KEY` writer"]
pub type W = crate::W<SfrSegptrSegidKeySpec>;
#[doc = "Field `SEGID_KEY` reader - cr_segptrstart read/write control register"]
pub type SegidKeyR = crate::FieldReader<u16>;
#[doc = "Field `SEGID_KEY` writer - cr_segptrstart read/write control register"]
pub type SegidKeyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_key(&self) -> SegidKeyR {
        SegidKeyR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_key(&mut self) -> SegidKeyW<'_, SfrSegptrSegidKeySpec> {
        SegidKeyW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrSegidKeySpec;
impl crate::RegisterSpec for SfrSegptrSegidKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_segid_key::R`](R) reader structure"]
impl crate::Readable for SfrSegptrSegidKeySpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_segid_key::W`](W) writer structure"]
impl crate::Writable for SfrSegptrSegidKeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_SEGID_KEY to value 0"]
impl crate::Resettable for SfrSegptrSegidKeySpec {}
