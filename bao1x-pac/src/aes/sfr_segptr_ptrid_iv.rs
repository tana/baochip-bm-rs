#[doc = "Register `SFR_SEGPTR_PTRID_IV` reader"]
pub type R = crate::R<SfrSegptrPtridIvSpec>;
#[doc = "Register `SFR_SEGPTR_PTRID_IV` writer"]
pub type W = crate::W<SfrSegptrPtridIvSpec>;
#[doc = "Field `PTRID_IV` reader - cr_segptrstart read/write control register"]
pub type PtridIvR = crate::FieldReader<u16>;
#[doc = "Field `PTRID_IV` writer - cr_segptrstart read/write control register"]
pub type PtridIvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_iv(&self) -> PtridIvR {
        PtridIvR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_iv(&mut self) -> PtridIvW<'_, SfrSegptrPtridIvSpec> {
        PtridIvW::new(self, 0)
    }
}
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridIvSpec;
impl crate::RegisterSpec for SfrSegptrPtridIvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_iv::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridIvSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_iv::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridIvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_IV to value 0"]
impl crate::Resettable for SfrSegptrPtridIvSpec {}
