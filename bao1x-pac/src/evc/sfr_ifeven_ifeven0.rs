#[doc = "Register `SFR_IFEVEN_IFEVEN0` reader"]
pub type R = crate::R<SfrIfevenIfeven0Spec>;
#[doc = "Register `SFR_IFEVEN_IFEVEN0` writer"]
pub type W = crate::W<SfrIfevenIfeven0Spec>;
#[doc = "Field `ifeven0` reader - ifeven read/write control register"]
pub type Ifeven0R = crate::FieldReader<u32>;
#[doc = "Field `ifeven0` writer - ifeven read/write control register"]
pub type Ifeven0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven0(&self) -> Ifeven0R {
        Ifeven0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven0(&mut self) -> Ifeven0W<'_, SfrIfevenIfeven0Spec> {
        Ifeven0W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfevenIfeven0Spec;
impl crate::RegisterSpec for SfrIfevenIfeven0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeven_ifeven0::R`](R) reader structure"]
impl crate::Readable for SfrIfevenIfeven0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeven_ifeven0::W`](W) writer structure"]
impl crate::Writable for SfrIfevenIfeven0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVEN_IFEVEN0 to value 0"]
impl crate::Resettable for SfrIfevenIfeven0Spec {}
