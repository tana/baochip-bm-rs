#[doc = "Register `SFR_IFEVEN_IFEVEN3` reader"]
pub type R = crate::R<SfrIfevenIfeven3Spec>;
#[doc = "Register `SFR_IFEVEN_IFEVEN3` writer"]
pub type W = crate::W<SfrIfevenIfeven3Spec>;
#[doc = "Field `ifeven3` reader - ifeven read/write control register"]
pub type Ifeven3R = crate::FieldReader<u32>;
#[doc = "Field `ifeven3` writer - ifeven read/write control register"]
pub type Ifeven3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven3(&self) -> Ifeven3R {
        Ifeven3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ifeven read/write control register"]
    #[inline(always)]
    pub fn ifeven3(&mut self) -> Ifeven3W<'_, SfrIfevenIfeven3Spec> {
        Ifeven3W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L147 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L147>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ifeven_ifeven3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ifeven_ifeven3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIfevenIfeven3Spec;
impl crate::RegisterSpec for SfrIfevenIfeven3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ifeven_ifeven3::R`](R) reader structure"]
impl crate::Readable for SfrIfevenIfeven3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ifeven_ifeven3::W`](W) writer structure"]
impl crate::Writable for SfrIfevenIfeven3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IFEVEN_IFEVEN3 to value 0"]
impl crate::Resettable for SfrIfevenIfeven3Spec {}
