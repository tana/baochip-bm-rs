#[doc = "Register `SFR_IFEVEN_IFEVEN2` reader"]
pub type R = crate::R<SfrIfevenIfeven2Spec>;
#[doc = "Register `SFR_IFEVEN_IFEVEN2` writer"]
pub type W = crate::W<SfrIfevenIfeven2Spec>;
#[doc = "Field `ifeven2` reader - ifeven read/write control register"]
pub type Ifeven2R = crate::FieldReader<u32>;
#[doc = "Field `ifeven2` writer - ifeven read/write control register"]
pub type Ifeven2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven2(&self) -> Ifeven2R {
        Ifeven2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven2(&mut self) -> Ifeven2W<'_, SfrIfevenIfeven2Spec> {
        Ifeven2W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfevenIfeven2Spec;
impl crate::RegisterSpec for SfrIfevenIfeven2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeven_ifeven2::R`](R) reader structure"]
impl crate::Readable for SfrIfevenIfeven2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeven_ifeven2::W`](W) writer structure"]
impl crate::Writable for SfrIfevenIfeven2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVEN_IFEVEN2 to value 0"]
impl crate::Resettable for SfrIfevenIfeven2Spec {}
