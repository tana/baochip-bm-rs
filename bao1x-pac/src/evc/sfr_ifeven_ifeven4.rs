#[doc = "Register `SFR_IFEVEN_IFEVEN4` reader"]
pub type R = crate::R<SfrIfevenIfeven4Spec>;
#[doc = "Register `SFR_IFEVEN_IFEVEN4` writer"]
pub type W = crate::W<SfrIfevenIfeven4Spec>;
#[doc = "Field `ifeven4` reader - ifeven read/write control register"]
pub type Ifeven4R = crate::FieldReader<u32>;
#[doc = "Field `ifeven4` writer - ifeven read/write control register"]
pub type Ifeven4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven4(&self) -> Ifeven4R {
        Ifeven4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven4(&mut self) -> Ifeven4W<'_, SfrIfevenIfeven4Spec> {
        Ifeven4W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfevenIfeven4Spec;
impl crate::RegisterSpec for SfrIfevenIfeven4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeven_ifeven4::R`](R) reader structure"]
impl crate::Readable for SfrIfevenIfeven4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeven_ifeven4::W`](W) writer structure"]
impl crate::Writable for SfrIfevenIfeven4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVEN_IFEVEN4 to value 0"]
impl crate::Resettable for SfrIfevenIfeven4Spec {}
