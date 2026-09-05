#[doc = "Register `SFR_SEGPTR_PTRID_PKB` reader"]
pub type R = crate::R<SfrSegptrPtridPkbSpec>;
#[doc = "Register `SFR_SEGPTR_PTRID_PKB` writer"]
pub type W = crate::W<SfrSegptrPtridPkbSpec>;
#[doc = "Field `PTRID_PKB` reader - cr_segptrstart read/write control register"]
pub type PtridPkbR = crate::FieldReader<u16>;
#[doc = "Field `PTRID_PKB` writer - cr_segptrstart read/write control register"]
pub type PtridPkbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pkb(&self) -> PtridPkbR {
        PtridPkbR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pkb(&mut self) -> PtridPkbW<'_, SfrSegptrPtridPkbSpec> {
        PtridPkbW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pkb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pkb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridPkbSpec;
impl crate::RegisterSpec for SfrSegptrPtridPkbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_pkb::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridPkbSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_pkb::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridPkbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_PKB to value 0"]
impl crate::Resettable for SfrSegptrPtridPkbSpec {}
