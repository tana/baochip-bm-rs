#[doc = "Register `SFR_SEGPTR_PTRID_POB` reader"]
pub type R = crate::R<SfrSegptrPtridPobSpec>;
#[doc = "Register `SFR_SEGPTR_PTRID_POB` writer"]
pub type W = crate::W<SfrSegptrPtridPobSpec>;
#[doc = "Field `PTRID_POB` reader - cr_segptrstart read/write control register"]
pub type PtridPobR = crate::FieldReader<u16>;
#[doc = "Field `PTRID_POB` writer - cr_segptrstart read/write control register"]
pub type PtridPobW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pob(&self) -> PtridPobR {
        PtridPobR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pob(&mut self) -> PtridPobW<'_, SfrSegptrPtridPobSpec> {
        PtridPobW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pob::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pob::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridPobSpec;
impl crate::RegisterSpec for SfrSegptrPtridPobSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_pob::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridPobSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_pob::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridPobSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_POB to value 0"]
impl crate::Resettable for SfrSegptrPtridPobSpec {}
