#[doc = "Register `SFR_MLIE_CR_MLIE1` reader"]
pub type R = crate::R<SfrMlieCrMlie1Spec>;
#[doc = "Register `SFR_MLIE_CR_MLIE1` writer"]
pub type W = crate::W<SfrMlieCrMlie1Spec>;
#[doc = "Field `cr_mlie1` reader - cr_mlie read/write control register"]
pub type CrMlie1R = crate::FieldReader<u32>;
#[doc = "Field `cr_mlie1` writer - cr_mlie read/write control register"]
pub type CrMlie1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_mlie read/write control register"]
    #[inline(always)]
    pub fn cr_mlie1(&self) -> CrMlie1R {
        CrMlie1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_mlie read/write control register"]
    #[inline(always)]
    pub fn cr_mlie1(&mut self) -> CrMlie1W<'_, SfrMlieCrMlie1Spec> {
        CrMlie1W::new(self, 0)
    }
}
#[doc = "See `mesh.sv#L50 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L50>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlie_cr_mlie1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlie_cr_mlie1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMlieCrMlie1Spec;
impl crate::RegisterSpec for SfrMlieCrMlie1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mlie_cr_mlie1::R`](R) reader structure"]
impl crate::Readable for SfrMlieCrMlie1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mlie_cr_mlie1::W`](W) writer structure"]
impl crate::Writable for SfrMlieCrMlie1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MLIE_CR_MLIE1 to value 0"]
impl crate::Resettable for SfrMlieCrMlie1Spec {}
