#[doc = "Register `SFR_IFEVEN_IFEVEN1` reader"]
pub type R = crate::R<SfrIfevenIfeven1Spec>;
#[doc = "Register `SFR_IFEVEN_IFEVEN1` writer"]
pub type W = crate::W<SfrIfevenIfeven1Spec>;
#[doc = "Field `ifeven1` reader - ifeven read/write control register"]
pub type Ifeven1R = crate::FieldReader<u32>;
#[doc = "Field `ifeven1` writer - ifeven read/write control register"]
pub type Ifeven1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven1(&self) -> Ifeven1R {
        Ifeven1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven1(&mut self) -> Ifeven1W<'_, SfrIfevenIfeven1Spec> {
        Ifeven1W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfevenIfeven1Spec;
impl crate::RegisterSpec for SfrIfevenIfeven1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeven_ifeven1::R`](R) reader structure"]
impl crate::Readable for SfrIfevenIfeven1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeven_ifeven1::W`](W) writer structure"]
impl crate::Writable for SfrIfevenIfeven1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVEN_IFEVEN1 to value 0"]
impl crate::Resettable for SfrIfevenIfeven1Spec {}
