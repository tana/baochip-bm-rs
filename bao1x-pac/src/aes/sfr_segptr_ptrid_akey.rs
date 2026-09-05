#[doc = "Register `SFR_SEGPTR_PTRID_AKEY` reader"]
pub type R = crate::R<SfrSegptrPtridAkeySpec>;
#[doc = "Register `SFR_SEGPTR_PTRID_AKEY` writer"]
pub type W = crate::W<SfrSegptrPtridAkeySpec>;
#[doc = "Field `PTRID_AKEY` reader - cr_segptrstart read/write control register"]
pub type PtridAkeyR = crate::FieldReader<u16>;
#[doc = "Field `PTRID_AKEY` writer - cr_segptrstart read/write control register"]
pub type PtridAkeyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_akey(&self) -> PtridAkeyR {
        PtridAkeyR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_akey(&mut self) -> PtridAkeyW<'_, SfrSegptrPtridAkeySpec> {
        PtridAkeyW::new(self, 0)
    }
}
#[doc = "See `aes.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_akey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_akey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridAkeySpec;
impl crate::RegisterSpec for SfrSegptrPtridAkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_akey::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridAkeySpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_akey::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridAkeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_AKEY to value 0"]
impl crate::Resettable for SfrSegptrPtridAkeySpec {}
