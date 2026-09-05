#[doc = "Register `SFR_SEGPTR_PTRID_PIB0` reader"]
pub type R = crate::R<SfrSegptrPtridPib0Spec>;
#[doc = "Register `SFR_SEGPTR_PTRID_PIB0` writer"]
pub type W = crate::W<SfrSegptrPtridPib0Spec>;
#[doc = "Field `PTRID_PIB0` reader - cr_segptrstart read/write control register"]
pub type PtridPib0R = crate::FieldReader<u16>;
#[doc = "Field `PTRID_PIB0` writer - cr_segptrstart read/write control register"]
pub type PtridPib0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pib0(&self) -> PtridPib0R {
        PtridPib0R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pib0(&mut self) -> PtridPib0W<'_, SfrSegptrPtridPib0Spec> {
        PtridPib0W::new(self, 0)
    }
}
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pib0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pib0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridPib0Spec;
impl crate::RegisterSpec for SfrSegptrPtridPib0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_pib0::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridPib0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_pib0::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridPib0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_PIB0 to value 0"]
impl crate::Resettable for SfrSegptrPtridPib0Spec {}
