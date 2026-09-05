#[doc = "Register `SFR_OPT2` reader"]
pub type R = crate::R<SfrOpt2Spec>;
#[doc = "Register `SFR_OPT2` writer"]
pub type W = crate::W<SfrOpt2Spec>;
#[doc = "Field `cr_opt_scrtchk` reader - cr_opt.scrtchk read/write control register"]
pub type CrOptScrtchkR = crate::BitReader;
#[doc = "Field `cr_opt_scrtchk` writer - cr_opt.scrtchk read/write control register"]
pub type CrOptScrtchkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_opt_ifsob` reader - cr_opt.ifsob read/write control register"]
pub type CrOptIfsobR = crate::BitReader;
#[doc = "Field `cr_opt_ifsob` writer - cr_opt.ifsob read/write control register"]
pub type CrOptIfsobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_opt_ifstart` reader - cr_opt.ifstart read/write control register"]
pub type CrOptIfstartR = crate::BitReader;
#[doc = "Field `cr_opt_ifstart` writer - cr_opt.ifstart read/write control register"]
pub type CrOptIfstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `opt_schnr1_0` reader - opt_schnr\\[1:0\\] read/write control register"]
pub type OptSchnr1_0R = crate::BitReader;
#[doc = "Field `opt_schnr1_0` writer - opt_schnr\\[1:0\\] read/write control register"]
pub type OptSchnr1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsmode` reader - tsmode read/write control register"]
pub type TsmodeR = crate::BitReader;
#[doc = "Field `tsmode` writer - tsmode read/write control register"]
pub type TsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_opt_ifskey` reader - cr_opt.ifskey read/write control register"]
pub type CrOptIfskeyR = crate::BitReader;
#[doc = "Field `cr_opt_ifskey` writer - cr_opt.ifskey read/write control register"]
pub type CrOptIfskeyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - cr_opt.scrtchk read/write control register"]
    #[inline(always)]
    pub fn cr_opt_scrtchk(&self) -> CrOptScrtchkR {
        CrOptScrtchkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - cr_opt.ifsob read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifsob(&self) -> CrOptIfsobR {
        CrOptIfsobR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cr_opt.ifstart read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifstart(&self) -> CrOptIfstartR {
        CrOptIfstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - opt_schnr\\[1:0\\] read/write control register"]
    #[inline(always)]
    pub fn opt_schnr1_0(&self) -> OptSchnr1_0R {
        OptSchnr1_0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tsmode read/write control register"]
    #[inline(always)]
    pub fn tsmode(&self) -> TsmodeR {
        TsmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - cr_opt.ifskey read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifskey(&self) -> CrOptIfskeyR {
        CrOptIfskeyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - cr_opt.scrtchk read/write control register"]
    #[inline(always)]
    pub fn cr_opt_scrtchk(&mut self) -> CrOptScrtchkW<'_, SfrOpt2Spec> {
        CrOptScrtchkW::new(self, 0)
    }
    #[doc = "Bit 1 - cr_opt.ifsob read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifsob(&mut self) -> CrOptIfsobW<'_, SfrOpt2Spec> {
        CrOptIfsobW::new(self, 1)
    }
    #[doc = "Bit 2 - cr_opt.ifstart read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifstart(&mut self) -> CrOptIfstartW<'_, SfrOpt2Spec> {
        CrOptIfstartW::new(self, 2)
    }
    #[doc = "Bit 3 - opt_schnr\\[1:0\\] read/write control register"]
    #[inline(always)]
    pub fn opt_schnr1_0(&mut self) -> OptSchnr1_0W<'_, SfrOpt2Spec> {
        OptSchnr1_0W::new(self, 3)
    }
    #[doc = "Bit 4 - tsmode read/write control register"]
    #[inline(always)]
    pub fn tsmode(&mut self) -> TsmodeW<'_, SfrOpt2Spec> {
        TsmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - cr_opt.ifskey read/write control register"]
    #[inline(always)]
    pub fn cr_opt_ifskey(&mut self) -> CrOptIfskeyW<'_, SfrOpt2Spec> {
        CrOptIfskeyW::new(self, 5)
    }
}
#[doc = "See `combohasha.sv#L214 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L214>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOpt2Spec;
impl crate::RegisterSpec for SfrOpt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_opt2::R`](R) reader structure"]
impl crate::Readable for SfrOpt2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_opt2::W`](W) writer structure"]
impl crate::Writable for SfrOpt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPT2 to value 0"]
impl crate::Resettable for SfrOpt2Spec {}
