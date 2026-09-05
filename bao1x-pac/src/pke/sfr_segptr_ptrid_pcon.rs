#[doc = "Register `SFR_SEGPTR_PTRID_PCON` reader"]
pub type R = crate::R<SfrSegptrPtridPconSpec>;
#[doc = "Register `SFR_SEGPTR_PTRID_PCON` writer"]
pub type W = crate::W<SfrSegptrPtridPconSpec>;
#[doc = "Field `PTRID_PCON` reader - cr_segptrstart read/write control register"]
pub type PtridPconR = crate::FieldReader<u16>;
#[doc = "Field `PTRID_PCON` writer - cr_segptrstart read/write control register"]
pub type PtridPconW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pcon(&self) -> PtridPconR {
        PtridPconR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn ptrid_pcon(&mut self) -> PtridPconW<'_, SfrSegptrPtridPconSpec> {
        PtridPconW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L307 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L307>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_ptrid_pcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_ptrid_pcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrPtridPconSpec;
impl crate::RegisterSpec for SfrSegptrPtridPconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_ptrid_pcon::R`](R) reader structure"]
impl crate::Readable for SfrSegptrPtridPconSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_ptrid_pcon::W`](W) writer structure"]
impl crate::Writable for SfrSegptrPtridPconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_PTRID_PCON to value 0"]
impl crate::Resettable for SfrSegptrPtridPconSpec {}
